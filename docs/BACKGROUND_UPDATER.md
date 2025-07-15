# Background Updater

The Background Updater is a core component of the Geolocation API that automatically keeps your IP databases up to date. It periodically checks for updates to VPN, proxy, and other IP-related databases, ensuring your geolocation data remains current and accurate.

## Table of Contents

- [Overview](#overview)
- [How It Works](#how-it-works)
- [Configuration](#configuration)
  - [Environment Variables](#environment-variables)
  - [Update Sources](#update-sources)
  - [Scheduling](#scheduling)
- [Database Files](#database-files)
- [Troubleshooting](#troubleshooting)
- [Monitoring](#monitoring)
- [Security Considerations](#security-considerations)
- [Advanced Usage](#advanced-usage)
- [FAQ](#frequently-asked-questions)

## Overview

The Background Updater is responsible for:

1. **Scheduled Updates**: Periodically checks for updates to IP databases
2. **Automatic Downloads**: Downloads new database files from configured sources
3. **Atomic Updates**: Safely replaces old database files without service interruption
4. **Verification**: Validates downloaded files before use
5. **Error Handling**: Implements retry logic and failure notifications

## How It Works

1. **Initialization**:
   - Loads configuration from environment variables
   - Creates necessary directories if they don't exist
   - Verifies write permissions for database directories

2. **Update Cycle**:
   - For each configured database source:
     1. Downloads the file to a temporary location
     2. Validates the file (checksum, format, etc.)
     3. If valid, atomically replaces the existing file
     4. Updates the last modified timestamp
   - Sleeps for the configured interval before the next update check

3. **Error Handling**:
   - Implements exponential backoff for failed downloads
   - Sends notifications for critical failures
   - Preserves the last known good database if an update fails

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `GEO_BACKGROUND_UPDATER__ENABLED` | `true` | Enable/disable the background updater |
| `GEO_BACKGROUND_UPDATER__INTERVAL_SECS` | `86400` (24h) | Update check interval in seconds |
| `GEO_BACKGROUND_UPDATER__TIMEOUT_SECS` | `300` (5m) | Download timeout in seconds |
| `GEO_BACKGROUND_UPDATER__RETRY_ATTEMPTS` | `3` | Number of retry attempts for failed downloads |
| `GEO_BACKGROUND_UPDATER__RETRY_DELAY_SECS` | `60` | Delay between retry attempts in seconds |
| `GEO_BACKGROUND_UPDATER__TEMP_DIR` | `data/tmp_update` | Directory for temporary files |
| `GEO_BACKGROUND_UPDATER__VPN_URL` | - | URL for VPN IP database |
| `GEO_BACKGROUND_UPDATER__HTTP_PROXY_URL` | - | URL for HTTP proxy database |
| `GEO_BACKGROUND_UPDATER__SOCKS4_PROXY_URL` | - | URL for SOCKS4 proxy database |
| `GEO_BACKGROUND_UPDATER__SOCKS5_PROXY_URL` | - | URL for SOCKS5 proxy database |
| `GEO_BACKGROUND_UPDATER__MAX_DOWNLOAD_SIZE` | `104857600` (100MB) | Maximum allowed download size in bytes |

### Update Sources

Configure the source URLs for each database type in your `.env` file:

```env
# VPN IP Database
GEO_BACKGROUND_UPDATER__VPN_URL=https://example.com/path/to/vpn_ips.csv

# Proxy Databases
GEO_BACKGROUND_UPDATER__HTTP_PROXY_URL=https://example.com/path/to/http_proxies.csv
GEO_BACKGROUND_UPDATER__SOCKS4_PROXY_URL=https://example.com/path/to/socks4_proxies.csv
GEO_BACKGROUND_UPDATER__SOCKS5_PROXY_URL=https://example.com/path/to/socks5_proxies.csv
```

### Scheduling

The updater runs in a continuous loop with a configurable interval. The default is 24 hours between update checks. You can adjust this based on your requirements:

```env
# Check for updates every 6 hours
GEO_BACKGROUND_UPDATER__INTERVAL_SECS=21600
```

## Database Files

### File Locations

| Database Type | Default Path |
|---------------|--------------|
| VPN IPs | `data/vpn/vpn_ips.csv` |
| HTTP Proxies | `data/proxy/http_proxies.csv` |
| SOCKS4 Proxies | `data/proxy/socks4_proxies.csv` |
| SOCKS5 Proxies | `data/proxy/socks5_proxies.csv` |

### File Formats

#### VPN IP Database (CSV)
```csv
ip_address,first_seen,last_seen,organization
1.2.3.4,2023-01-01,2023-07-15,Example VPN Inc.
5.6.7.8,2023-02-15,2023-07-15,Another VPN Provider
```

#### Proxy Database (CSV)
```csv
ip_address,port,type,country,first_seen,last_seen
1.2.3.4,8080,http,US,2023-01-01,2023-07-15
5.6.7.8,1080,socks5,DE,2023-02-15,2023-07-15
```

## Troubleshooting

### Common Issues

#### Updates Not Happening
1. **Check Logs**: Look for error messages in the application logs
2. **Verify Permissions**: Ensure the application has write access to the data directories
3. **Check Network**: Verify the server can reach the update URLs
4. **Verify Configuration**: Ensure all required environment variables are set

#### Corrupt Database Files
1. **Automatic Recovery**: The updater should detect corrupt files and keep the previous version
2. **Manual Recovery**: Restore from backup or delete the corrupt file to trigger a fresh download

### Logging

The updater logs its activities to the application log. Look for messages with the `[BackgroundUpdater]` prefix.

Example log entries:
```
[INFO] [BackgroundUpdater] Starting update check
[DEBUG] [BackgroundUpdater] Downloading VPN database from https://...
[INFO] [BackgroundUpdater] Successfully updated VPN database (1,234 new entries)
[ERROR] [BackgroundUpdater] Failed to download HTTP proxy database: Connection timed out
[WARN] [BackgroundUpdater] Retrying HTTP proxy database download (attempt 1/3)
```

## Monitoring

### Health Check Endpoint

The `/api/health` endpoint includes information about the last update status:

```json
{
  "status": "ok",
  "services": {
    "background_updater": {
      "status": "running",
      "last_run": "2023-07-15T12:00:00Z",
      "next_run": "2023-07-16T12:00:00Z",
      "last_success": "2023-07-15T12:00:00Z",
      "databases": [
        {
          "type": "vpn",
          "last_updated": "2023-07-15T12:00:00Z",
          "entries": 12345,
          "status": "ok"
        },
        {
          "type": "http_proxy",
          "last_updated": "2023-07-15T12:00:01Z",
          "entries": 6789,
          "status": "ok"
        }
      ]
    }
  }
}
```

### Metrics

The following Prometheus metrics are available at `/metrics`:

```
# HELP background_updater_runs_total Total number of update runs
# TYPE background_updater_runs_total counter
background_updater_runs_total{status="success"} 42
background_updater_runs_total{status="error"} 3

# HELP background_updater_database_entries Number of entries in each database
# TYPE background_updater_database_entries gauge
background_updater_database_entries{type="vpn"} 12345
background_updater_database_entries{type="http_proxy"} 6789

# HELP background_updater_download_duration_seconds Time spent downloading database updates
# TYPE background_updater_download_duration_seconds histogram
background_updater_download_duration_seconds_bucket{type="vpn",le="5.0"} 1
background_updater_download_duration_seconds_sum{type="vpn"} 3.2
background_updater_download_duration_seconds_count{type="vpn"} 1
```

## Security Considerations

1. **HTTPS**: Always use HTTPS for update URLs to prevent MITM attacks
2. **Authentication**: If your update sources require authentication, use URL authentication or API keys
3. **File Validation**: The updater validates file signatures and checksums when available
4. **File Permissions**: Database files are stored with restricted permissions (0600)
5. **Temporary Files**: Temporary files are created in a secure directory with restricted permissions

## Advanced Usage

### Manual Updates

You can trigger an immediate update by sending a SIGHUP signal to the process:

```bash
# Find the process ID
pgrep -f geolocation

# Send SIGHUP
kill -HUP <pid>
```

Or via the API (if enabled):

```bash
curl -X POST http://localhost:8080/api/admin/update-databases \
     -H "X-API-Key: your-admin-api-key"
```

### Custom Update Scripts

You can extend the updater by adding custom scripts to the `scripts/updates` directory. Any executable files in this directory will be run after a successful update.

Example script (`scripts/updates/99-notify.sh`):
```bash
#!/bin/bash
# This script runs after a successful database update

# Send a notification
curl -X POST https://api.example.com/notify \
     -H "Content-Type: application/json" \
     -d '{"message": "Geolocation databases updated"}'
```

## Frequently Asked Questions

### How often should I update the databases?

- **VPN/Proxy Databases**: Daily or weekly, depending on your accuracy requirements
- **MaxMind GeoIP**: Monthly, as new versions are typically released at the beginning of each month

### Can I use my own update sources?

Yes, you can configure any HTTP/HTTPS URL as an update source. The updater supports:
- Direct file downloads
- Basic authentication
- Custom headers (via environment variables)

### How are updates performed atomically?

The updater:
1. Downloads the new file to a temporary location
2. Validates the file
3. Creates a backup of the existing file (if any)
4. Atomically renames the temporary file to the target name
5. Updates file permissions

This ensures that the service always has a consistent database file, even if the update is interrupted.

### How can I verify the integrity of downloaded files?

The updater supports checksum verification. Create a `.sha256` file next to your database file with the same name (e.g., `vpn_ips.csv.sha256`) containing the SHA-256 hash of the file.

### What happens if the update fails?

The updater will:
1. Log the error
2. Keep the existing database file
3. Retry according to the configured retry policy
4. Continue with the next database if one update fails

### How can I monitor the update process?

1. Check the application logs
2. Use the health check endpoint
3. Monitor the Prometheus metrics
4. Set up alerts for failed updates

## Support

For additional help, please contact [support@yourdomain.com](mailto:support@yourdomain.com) or open an issue in our [GitHub repository](https://github.com/your-org/geolocation/issues).