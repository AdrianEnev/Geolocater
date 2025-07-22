# Appendices

## Appendix A: Configuration Reference

### Web-API Configuration

#### Environment Variables

```env
# Server Configuration
NODE_ENV=production
PORT=3000
HOST=0.0.0.0

# Redis Configuration
REDIS_URL=redis://localhost:6379
REDIS_TTL=86400  # Cache TTL in seconds

# Authentication
API_KEYS=key1:name1,key2:name2
INTERNAL_SERVICE_TOKEN=your-secure-token

# Rate Limiting
RATE_LIMIT_WINDOW_MS=900000  # 15 minutes
RATE_LIMIT_MAX=100  # Max requests per window

# Logging
LOG_LEVEL=info
LOG_FORMAT=json
```

#### Configuration File (config/default.json)

```json
{
  "server": {
    "port": 3000,
    "host": "0.0.0.0",
    "environment": "production"
  },
  "redis": {
    "url": "redis://localhost:6379",
    "ttl": 86400
  },
  "security": {
    "rateLimit": {
      "windowMs": 900000,
      "max": 100
    },
    "apiKeys": [
      {
        "key": "key1",
        "name": "name1"
      }
    ]
  },
  "logging": {
    "level": "info",
    "format": "json"
  }
}
```

### Rust-Service Configuration

#### Environment Variables

```env
# Server Configuration
RUST_LOG=info
LISTEN_ADDR=0.0.0.0:8080

# Data Files
IPV4_DATABASE=data/ipv4.dat
IPV6_DATABASE=data/ipv6.dat
THREAT_INTEL_DB=data/threats.dat

# Performance
WORKER_THREADS=4
MAX_CONNECTIONS=1000

# Caching
CACHE_SIZE_MB=512
CACHE_TTL_SECONDS=3600
```

#### Configuration File (config.toml)

```toml
[server]
listen_addr = "0.0.0.0:8080"
worker_threads = 4
max_connections = 1000

[database]
ipv4_path = "data/ipv4.dat"
ipv6_path = "data/ipv6.dat"
threat_intel_path = "data/threats.dat"

[cache]
enabled = true
size_mb = 512
ttl_seconds = 3600

[logging]
level = "info"
format = "json"

[metrics]
enabled = true
endpoint = "/metrics"
push_gateway = ""
```

## Appendix B: API Reference

### Lookup Endpoint

```http
GET /api/v1/lookup?ip=1.1.1.1
Authorization: Bearer your-api-key
```

#### Response

```json
{
  "ip": "1.1.1.1",
  "country": "US",
  "country_name": "United States",
  "region": "CA",
  "region_name": "California",
  "city": "Los Angeles",
  "zip": "90001",
  "lat": 34.0522,
  "lon": -118.2437,
  "timezone": "America/Los_Angeles",
  "isp": "Cloudflare",
  "org": "APNIC and Cloudflare DNS Resolver project",
  "as": "AS13335 Cloudflare, Inc.",
  "threat": {
    "is_vpn": false,
    "is_proxy": false,
    "is_tor": false,
    "is_known_attacker": false,
    "threat_score": 0,
    "threat_level": "low"
  },
  "cached": false,
  "query_duration_ms": 0.45
}
```

### Batch Lookup Endpoint

```http
POST /api/v1/batch-lookup
Authorization: Bearer your-api-key
Content-Type: application/json

["1.1.1.1", "8.8.8.8", "2001:4860:4860::8888"]
```

## Appendix C: Data Structures

### IP Range Entry

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRange {
    pub start_ip: IpAddr,
    pub end_ip: IpAddr,
    pub country_code: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub usage_type: Option<String>,
    pub asn: Option<u32>,
    pub as_name: Option<String>,
    pub is_anonymous_proxy: bool,
    pub is_satellite_provider: bool,
    pub is_cloud_provider: bool,
    pub threat_score: u8,
    pub last_updated: DateTime<Utc>,
}
```

### Threat Detection Result

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatDetectionResult {
    pub is_vpn: bool,
    pub is_proxy: bool,
    pub is_tor: bool,
    pub is_known_attacker: bool,
    pub threat_score: u8,
    pub threat_level: ThreatLevel,
    pub confidence: f32,
    pub last_seen: Option<DateTime<Utc>>,
    pub first_seen: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}
```

## Appendix D: Performance Tuning

### Web-API Tuning

1. **Node.js Memory Limits**
   ```bash
   # Increase Node.js heap size
   NODE_OPTIONS="--max-old-space-size=4096"
   ```

2. **Redis Optimization**
   ```bash
   # In redis.conf
   maxmemory 2gb
   maxmemory-policy allkeys-lru
   ```

3. **Connection Pooling**
   ```javascript
   // Redis client configuration
   const redis = new Redis({
     host: 'localhost',
     port: 6379,
     maxRetriesPerRequest: 3,
     enableReadyCheck: false,
     retryStrategy: (times) => Math.min(times * 50, 2000)
   });
   ```

### Rust-Service Tuning

1. **Runtime Configuration**
   ```toml
   # In Cargo.toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   ```

2. **Memory Management**
   ```rust
   // Pre-allocate vectors when size is known
   let mut entries = Vec::with_capacity(1_000_000);
   
   // Use Box for large structs
   let large_data = Box::new(LargeStruct::new());
   ```

3. **Concurrency**
   ```rust
   // Use thread pool for CPU-bound work
   let pool = ThreadPoolBuilder::new()
       .num_threads(num_cpus::get())
       .build()
       .unwrap();
   ```

## Appendix E: Monitoring and Metrics

### Prometheus Metrics

```rust
lazy_static! {
    static ref REQUESTS_TOTAL: IntCounter = register_int_counter!(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds"
    ).unwrap();
}
```

### Grafana Dashboard

Example dashboard configuration for monitoring:

```json
{
  "annotations": {
    "list": [
      {
        "builtIn": 1,
        "datasource": "-- Grafana --",
        "enable": true,
        "hide": true,
        "iconColor": "rgba(0, 211, 255, 1)",
        "name": "Annotations & Alerts",
        "type": "dashboard"
      }
    ]
  },
  "editable": true,
  "gnetId": null,
  "graphTooltip": 0,
  "id": 1,
  "links": [],
  "panels": [
    {
      "datasource": "Prometheus",
      "fieldConfig": {
        "defaults": {
          "custom": {},
          "thresholds": {
            "mode": "absolute",
            "steps": [
              { "color": "green", "value": null },
              { "color": "red", "value": 80 }
            ]
          }
        },
        "overrides": []
      },
      "gridPos": { "h": 8, "w": 12, "x": 0, "y": 0 },
      "id": 2,
      "options": {},
      "targets": [
        {
          "expr": "sum(rate(http_requests_total[5m])) by (status)",
          "legendFormat": "{{status}}",
          "refId": "A"
        }
      ],
      "title": "HTTP Requests Rate",
      "type": "timeseries"
    }
  ],
  "schemaVersion": 27,
  "style": "dark",
  "tags": [],
  "templating": {
    "list": []
  },
  "time": {
    "from": "now-6h",
    "to": "now"
  },
  "timepicker": {},
  "timezone": "browser",
  "title": "InfraLock Monitoring",
  "version": 1
}
```

## Appendix F: Troubleshooting

### Common Issues

1. **High Memory Usage**
   - Check for memory leaks in long-running processes
   - Review cache size and eviction policies
   - Monitor for unbounded data structures

2. **Slow Lookups**
   - Verify Redis connection health
   - Check for network latency between services
   - Review database query performance

3. **Authentication Failures**
   - Verify API keys and tokens
   - Check token expiration
   - Review rate limiting settings

### Log Analysis

Example log entry:

```json
{
  "timestamp": "2025-06-15T14:32:10.123Z",
  "level": "ERROR",
  "message": "Failed to process IP lookup",
  "error": "Invalid IP address format",
  "ip": "300.400.500.600",
  "request_id": "abc123",
  "service": "ip-lookup"
}
```

## Appendix G: Security Considerations

### Data Protection
- All sensitive data is encrypted at rest
- API keys are stored hashed using bcrypt
- Regular security audits and penetration testing

### Rate Limiting
- Implemented at multiple levels (Nginx, application, API)
- Configurable thresholds based on client type
- Automatic IP blocking for suspicious activity

### Compliance
- GDPR compliant data handling
- Regular security assessments
- Incident response plan in place

## Appendix H: Deployment Examples

### Docker Compose

```yaml
version: '3.8'

services:
  web-api:
    image: infralock/web-api:latest
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - REDIS_URL=redis://redis:6379
    depends_on:
      - redis
      - rust-service

  rust-service:
    image: infralock/rust-service:latest
    ports:
      - "8080:8080"
    volumes:
      - ./data:/app/data
    environment:
      - RUST_LOG=info

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes
    volumes:
      - redis-data:/data

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3001:3000"
    volumes:
      - grafana-storage:/var/lib/grafana

volumes:
  redis-data:
  grafana-storage:
```

## Appendix I: Performance Benchmarks

### Test Environment
- **CPU**: 4 vCPUs (Intel Xeon @ 2.5GHz)
- **Memory**: 16GB RAM
- **Storage**: 100GB SSD
- **OS**: Ubuntu 22.04 LTS

### Test Results

| Test Case | Requests/sec | Avg Latency | 95th %ile | Memory Usage |
|-----------|--------------|--------------|------------|--------------|
| Single IP Lookup | 45,000 | 0.45ms | 0.89ms | 1.2GB |
| Batch (100 IPs) | 12,000 | 12ms | 25ms | 1.5GB |
| Mixed Workload | 28,000 | 8ms | 18ms | 2.1GB |
| Stress Test | 120,000 | 15ms | 42ms | 3.8GB |

## Appendix J: Glossary

- **ASN**: Autonomous System Number, a unique identifier for a network on the internet
- **CDN**: Content Delivery Network, a distributed network of servers
- **DDoS**: Distributed Denial of Service, an attack that overwhelms a target with traffic
- **GEOIP**: Geolocation based on IP address
- **ISP**: Internet Service Provider
- **RTT**: Round-Trip Time, the time it takes for a signal to travel to a destination and back
- **TOR**: The Onion Router, a network for anonymous communication
- **VPN**: Virtual Private Network, a service that masks your IP address

## Appendix K: Changelog

### v1.0.0 (2025-06-15)
- Initial release of InfraLock
- High-performance IP geolocation
- Advanced threat detection
- Comprehensive API

### v0.9.0 (2025-05-01)
- Beta release
- Performance optimizations
- Improved documentation

## Appendix L: License

InfraLock is released under the MIT License. See the LICENSE file for details.

## Appendix M: Contact Information

For support, please contact:
- Email: support@infralock.example.com
- GitHub: https://github.com/AdrianEnev/Geolocater/issues
- Documentation: https://infralock.example.com/docs
