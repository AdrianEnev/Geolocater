use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};
use ip_network::IpNetwork;
use url::Url;

use crate::ip_lookup::{
    loader::{IpRangeLoader, IpRangeLoaderConfig},
    tree::RadixTree,
    types::{IpCategory, IpRange, SourceFormat, IpVersion},
    SharedRadixTree,
};

/// Configuration for the IP lookup service
#[derive(Debug, Clone)]
pub struct IpLookupServiceConfig {
    /// Base directory for storing IP range data
    pub data_dir: PathBuf,
    /// Whether to check for updates on startup
    pub check_updates: bool,
    /// How often to check for updates (in seconds)
    pub update_interval_secs: u64,
    /// Maximum age of cached data before updating (in seconds)
    pub max_cache_age_secs: u64,
    /// List of data sources to load
    pub sources: Vec<IpRangeSource>,
}

/// Configuration for an IP range data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRangeSource {
    pub url: String,
    pub category: IpCategory,
    pub name: String,
    pub enabled: bool,
    #[serde(default)]
    pub format: SourceFormat,
    pub ip_version: IpVersion,
}

/// The IP lookup service
#[derive(Debug)]
pub struct IpLookupService {
    /// The radix tree for fast IP lookups
    tree: SharedRadixTree,
    /// The loader for fetching and parsing IP ranges
    loader: IpRangeLoader,
    /// Service configuration
    config: IpLookupServiceConfig,
}

impl IpLookupService {
    /// Create a new IP lookup service
    pub fn new(config: IpLookupServiceConfig) -> Self {
        let loader_config = IpRangeLoaderConfig {
            data_dir: config.data_dir.clone(),
            check_updates: config.check_updates,
            max_cache_age_secs: config.max_cache_age_secs,
        };

        Self {
            tree: SharedRadixTree::new(),
            loader: IpRangeLoader::new(loader_config),
            config,
        }
    }

    /// Get a reference to the radix tree
    pub fn tree(&self) -> &SharedRadixTree {
        &self.tree
    }

    /// Start the background update task
    pub fn start_background_updates(&self) -> tokio::task::JoinHandle<()> {
        let service = self.clone();
        tokio::spawn(async move {
            service.run_update_loop().await;
        })
    }

    /// Run the update loop
    async fn run_update_loop(&self) {
        let update_interval = Duration::from_secs(self.config.update_interval_secs);
        let mut interval = tokio::time::interval(update_interval);

        // Initial update
        if let Err(e) = self.update_all_sources().await {
            error!(error = %e, "Failed to perform initial update");
        }

        // Periodic updates
        loop {
            interval.tick().await;
            if let Err(e) = self.update_all_sources().await {
                error!(error = %e, "Periodic update failed");
            }
        }
    }

    /// Update all data sources
    pub async fn update_all_sources(&self) -> anyhow::Result<()> {
        info!("Starting update of all IP range sources");
        let mut all_ranges = Vec::new();
        let mut errors = Vec::new();

        for source in &self.config.sources {
            if !source.enabled {
                continue;
            }

            match self.update_source(source).await {
                Ok(ranges) => {
                    info!(
                        source = %source.name,
                        category = ?source.category,
                        num_ranges = ranges.len(),
                        "Successfully updated source"
                    );
                    all_ranges.extend(ranges);
                }
                Err(e) => {
                    let error_msg = format!(
                        "Failed to update source {} ({}): {}",
                        source.name, source.url, e
                    );
                    error!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }

        // Update the radix tree with all ranges
        if !all_ranges.is_empty() {
            self.update_tree(all_ranges).await?;
        }

        // Log any errors that occurred
        if !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Errors occurred during update:\n{}",
                errors.join("\n")
            ));
        }

        Ok(())
    }

    /// Update a single data source
    async fn update_source(&self, source: &IpRangeSource) -> anyhow::Result<Vec<IpRange>> {
        info!("Checking source: {} ({})", source.name, source.url);
        
        // Generate a filename for this source
        let url = Url::parse(&source.url)?;
        let filename = self.loader.filename_from_url(&url, source.category, source.ip_version);
        let filepath = self.config.data_dir.join(&filename);
        
        // Check if the file exists and needs an update
        if filepath.exists() {
            if !self.loader.needs_update(&filepath) {
                info!("Source {} is up to date, loading from cache", source.name);
                return self.loader.load_from_file(&filepath, source.category, &source.name, source.format).await
                    .map_err(|e| anyhow::anyhow!("Failed to load from cache: {}", e));
            }
            
            if let Some(modified) = self.loader.last_modified(&filepath) {
                info!(
                    "Source {} was last updated at {}",
                    source.name,
                    modified.to_rfc3339()
                );
            }
        } else {
            info!("No local cache found for {}", source.name);
        }
        
        // Download and parse the ranges
        info!("Downloading ranges from {}", source.url);
        let ranges = self.loader.download_ranges(&source.url, source).await?;
        
        info!(
            "Downloaded {} ranges from {}",
            ranges.len(),
            source.name
        );
        
        Ok(ranges)
    }

    /// Update the radix tree with new ranges
    async fn update_tree(&self, ranges: Vec<IpRange>) -> anyhow::Result<()> {
        //info!("Updating radix tree with {} ranges", ranges.len());
        let mut v4_count = 0;
        let mut v6_count = 0;
        let mut parse_errors = 0;
        let mut v4_inserted = 0;
        let mut v6_inserted = 0;
        let mut v4_skipped = 0;
        let mut v6_skipped = 0;
        
        // Log the first 10 ranges for debugging
        //info!("First 10 ranges to process:");
        for (i, range) in ranges.iter().take(10).enumerate() {
            //info!("[{}] network={}, category={:?}, source={}", 
                 //i, range.network, range.category, range.source);
        }
        
        // Create a new tree to build up
        let mut new_tree = RadixTree::new();
        
        // Process each range
        for range in &ranges {
            match range.network.parse::<IpNetwork>() {
                Ok(network) => {
                    match network {
                        IpNetwork::V4(_) => v4_count += 1,
                        IpNetwork::V6(net) => {
                            v6_count += 1;
                            //debug!(
                            //    "Processing IPv6 range: {}/{} - Category: {:?} - Source: {}",
                            //    net.network_address(),
                            //    net.netmask(),
                            //    range.category,
                            //    range.source
                            //);
                        }
                    }
                    
                    // Insert into the new tree
                    let prev_category = new_tree.insert(network, range.category);
                    
                    // Track insertions vs skips
                    match network {
                        IpNetwork::V4(_) => {
                            if prev_category.is_some() {
                                v4_skipped += 1;
                            } else {
                                v4_inserted += 1;
                            }
                        },
                        IpNetwork::V6(net) => {
                            if prev_category.is_some() {
                                v6_skipped += 1;
                                //debug!("Skipped duplicate IPv6 network: {}/{}", net.network_address(), net.netmask());
                            } else {
                                v6_inserted += 1;
                                //debug!("Inserted new IPv6 network: {}/{}", net.network_address(), net.netmask());
                                
                                // Verify the insertion
                                let check = new_tree.lookup(net.network_address().into());
                                if check.is_some() {
                                    //debug!("Verified IPv6 network is in the tree");
                                } else {
                                    //error!("CRITICAL: Failed to verify IPv6 network {}/{} is in the tree!", net.network_address(), net.netmask());
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    parse_errors += 1;
                    if range.network.contains(':') {
                        error!("Failed to parse IPv6 network '{}' from source '{}': {}", 
                            range.network, range.source, e);
                    } else {
                        error!("Failed to parse network '{}' from source '{}': {}", 
                            range.network, range.source, e);
                    }
                }
            }
        }
        
        // Log summary before replacing the tree
        //info!(
        //    "Processed {} ranges ({} IPv4, {} IPv6, {} errors)",
        //    ranges.len(), v4_count, v6_count, parse_errors
        //);
        //info!(
        //    "IPv4: {} inserted, {} skipped",
        //    v4_inserted, v4_skipped
        //);
        //info!(
        //    "IPv6: {} inserted, {} skipped",
        //    v6_inserted, v6_skipped
        //);
        
        // Log tree sizes before replacement
        let (v4_size, v6_size) = new_tree.len();
        //info!(
        //    "New tree size before replacement - IPv4: {}, IPv6: {}, Total: {}",
        //    v4_size, v6_size, v4_size + v6_size
        //);

        // Atomically replace the tree
        info!("Replacing the radix tree with new data...");
        self.tree.replace(new_tree);
        
        // Log final tree size (using the tree we just updated)
        let (final_v4, final_v6) = self.tree.len();
        //info!(
        //    "Final tree size after replacement - IPv4: {}, IPv6: {}, Total: {}",
        //    final_v4, final_v6, final_v4 + final_v6
        //);
        
        if final_v6 == 0 && v6_count > 0 {
            error!(
                "CRITICAL: No IPv6 networks were inserted into the tree, but {} were processed. This indicates a critical issue with IPv6 handling.",
                v6_count
            );
            
            // Log some diagnostic information
            error!("IPv6 statistics from processing:");
            error!("- Total IPv6 ranges processed: {}", v6_count);
            error!("- IPv6 ranges inserted: {}", v6_inserted);
            error!("- IPv6 ranges skipped (duplicates): {}", v6_skipped);
            error!("- IPv6 parse errors: {}", parse_errors);
            
            // Log some sample IPv6 networks that should have been inserted
            let sample_networks: Vec<_> = ranges.iter()
                .filter(|r| r.network.contains(':'))
                .take(5)
                .map(|r| r.network.clone())
                .collect();
                
            error!("Sample IPv6 networks that should have been inserted: {:?}", sample_networks);
        }
        
        Ok(())
    }
}

impl Clone for IpLookupService {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree.clone(),
            loader: self.loader.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_service_update() {
        // Create a temporary directory for the test
        let temp_dir = tempdir().unwrap();
        let test_source = IpRangeSource {
            url: "https://example.com/ip_ranges.txt".to_string(),
            category: IpCategory::Vpn,
            name: "test".to_string(),
            enabled: true,
            format: SourceFormat::Default,
            ip_version: IpVersion::V4,
        };

        let config = IpLookupServiceConfig {
            data_dir: temp_dir.path().to_path_buf(),
            check_updates: true,
            update_interval_secs: 3600,
            max_cache_age_secs: 86400,
            sources: vec![test_source],
        };

        let service = IpLookupService::new(config);
        
        // Initial tree should be empty
        assert_eq!(service.tree().len(), (0, 0));
        assert_eq!(service.tree().total_len(), 0);
        
        // Test with some sample ranges
        let test_ranges = vec![
            IpRange {
                network: "192.168.1.0/24".to_string(),
                category: IpCategory::Vpn,
                source: "test".to_string(),
                first_seen: Utc::now(),
                last_updated: Utc::now(),
                format: SourceFormat::Default,
            },
            IpRange {
                network: "2001:db8::/32".to_string(),
                category: IpCategory::Vpn,
                source: "test".to_string(),
                first_seen: Utc::now(),
                last_updated: Utc::now(),
                format: SourceFormat::Default,
            },
        ];
        
        // Update the tree with test ranges
        service.update_tree(test_ranges).await.unwrap();
        
        // Verify the tree was updated
        let (v4_count, v6_count) = service.tree().len();
        assert!(v4_count > 0 || v6_count > 0, "Tree should contain some entries");
        assert_eq!(service.tree().total_len(), v4_count + v6_count);
        
        // Test that the service can be started
        let _handle = service.start_background_updates();
    }
}
