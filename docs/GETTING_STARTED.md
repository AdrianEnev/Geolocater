# Getting Started with Geolocation API

Welcome to the Geolocation API! This guide will help you quickly set up and start using our geolocation services.

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Docker (Recommended)](#docker-recommended)
  - [Manual Installation](#manual-installation)
  - [Kubernetes](#kubernetes)
- [Configuration](#configuration)
- [Your First Request](#your-first-request)
- [Authentication](#authentication)
- [Basic Usage Examples](#basic-usage-examples)
  - [Single IP Lookup](#single-ip-lookup)
  - [Self Lookup](#self-lookup)
  - [Proxy Detection](#proxy-detection)
  - [Batch Lookup](#batch-lookup)
- [Rate Limits](#rate-limits)
- [Next Steps](#next-steps)
- [Troubleshooting](#troubleshooting)
- [Getting Help](#getting-help)

## Quick Start

1. **Get an API Key**
   - Sign up at [https://console.yourdomain.com](https://console.yourdomain.com)
   - Create a new API key with appropriate permissions

2. **Make your first request**
   ```bash
   curl -X GET "https://api.yourdomain.com/v1/lookup/8.8.8.8" \
        -H "X-API-Key: your-api-key-here"
   ```

## Prerequisites

- **For self-hosting**:
  - Linux/macOS (Windows not officially supported but may work)
  - Docker 20.10+ and Docker Compose (recommended)
  - or Rust 1.70+ and Cargo for manual installation
  - 2GB+ RAM (4GB recommended for production)
  - 10GB+ free disk space (for databases)

- **For API usage only**:
  - Any platform with HTTP client capabilities
  - cURL, Postman, or your preferred HTTP client

## Installation

### Docker (Recommended)

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/geolocation.git
   cd geolocation
   ```

2. Copy the example environment file and update with your configuration:
   ```bash
   cp .env.example .env
   nano .env  # Edit with your settings
   ```

3. Start the services:
   ```bash
   docker-compose up -d
   ```

4. Verify the service is running:
   ```bash
   curl http://localhost:8080/api/health
   ```

### Manual Installation

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install system dependencies (Ubuntu/Debian example):
   ```bash
   sudo apt update
   sudo apt install -y build-essential pkg-config libssl-dev
   ```

3. Clone and build the project:
   ```bash
   git clone https://github.com/your-org/geolocation.git
   cd geolocation
   cargo build --release
   ```

4. Configure and run:
   ```bash
   cp .env.example .env
   nano .env  # Edit with your settings
   ./target/release/geolocation
   ```

### Kubernetes

1. Create a namespace:
   ```bash
   kubectl create namespace geolocation
   ```

2. Create a secret for environment variables:
   ```bash
   kubectl create secret generic geolocation-secrets \
     --from-env-file=.env \
     -n geolocation
   ```

3. Deploy the application:
   ```bash
   kubectl apply -f k8s/
   ```

## Configuration

### Required Environment Variables

```env
# Server Configuration
GEO_SERVER__HOST=0.0.0.0
GEO_SERVER__PORT=8080

# Database Paths
GEO_MAXMIND__DB_PATH=/data/geolite/GeoLite2-City.mmdb
GEO_VPN_DETECTOR__DB_PATH=/data/vpn/vpn_ips.csv
GEO_PROXY_DETECTOR__HTTP_DB_PATH=/data/proxy/http_proxies.csv
GEO_PROXY_DETECTOR__SOCKS4_DB_PATH=/data/proxy/socks4_proxies.csv
GEO_PROXY_DETECTOR__SOCKS5_DB_PATH=/data/proxy/socks5_proxies.csv

# Background Updater (Optional)
GEO_BACKGROUND_UPDATER__ENABLED=true
GEO_BACKGROUND_UPDATER__INTERVAL_SECS=86400  # 24 hours
```

### Recommended Configuration for Production

1. **Enable HTTPS**:
   - Set up a reverse proxy like Nginx or Caddy
   - Obtain SSL certificates (Let's Encrypt recommended)

2. **Enable Authentication**:
   - Set `AUTH_REQUIRED=true`
   - Configure API keys or JWT

3. **Monitoring**:
   - Enable Prometheus metrics at `/metrics`
   - Set up alerting for error rates

## Your First Request

Let's make a simple request to get information about an IP address:

```bash
curl -X GET "http://localhost:8080/api/lookup/8.8.8.8" \
     -H "X-API-Key: your-api-key"
```

You should receive a JSON response with geolocation data for 8.8.8.8 (Google's DNS server).

## Authentication

All API endpoints require authentication using an API key:

1. Include the API key in the `X-API-Key` header:
   ```http
   GET /api/lookup/1.1.1.1 HTTP/1.1
   Host: api.yourdomain.com
   X-API-Key: your-api-key-here
   ```

2. Or as a query parameter (less secure):
   ```
   GET /api/lookup/1.1.1.1?api_key=your-api-key-here
   ```

> **Security Note**: Always keep your API keys secure. Never commit them to version control.

## Basic Usage Examples

### Single IP Lookup

```bash
curl -X GET "http://localhost:8080/api/lookup/1.1.1.1" \
     -H "X-API-Key: your-api-key"
```

### Self Lookup

Get information about your own IP address:

```bash
curl -X GET "http://localhost:8080/api/lookup/self" \
     -H "X-API-Key: your-api-key"
```

### Proxy Detection

Check if an IP is a known proxy:

```bash
curl -X GET "http://localhost:8080/api/is_proxy/123.45.67.89" \
     -H "X-API-Key: your-api-key"
```

### Batch Lookup

Lookup multiple IPs in a single request:

```bash
curl -X POST "http://localhost:8080/api/batch/lookup" \
     -H "Content-Type: application/json" \
     -H "X-API-Key: your-api-key" \
     -d '{"ips": ["8.8.8.8", "1.1.1.1"]}'
```

## Rate Limits

By default, the API is rate limited to:
- 100 requests per minute per IP address
- 10,000 requests per day per API key

These limits can be adjusted in the configuration.

## Next Steps

1. **Explore the API**:
   - Check out the [API Reference](../API_REFERENCE.md) for detailed endpoint documentation
   - Try different query parameters to filter responses

2. **Integration**:
   - Use one of our [client libraries](../API_REFERENCE.md#client-libraries)
   - Set up webhooks for real-time notifications

3. **Advanced Configuration**:
   - Set up a [reverse proxy](../DEPLOYMENT.md#reverse-proxy-setup)
   - Configure [monitoring and alerting](../DEPLOYMENT.md#monitoring--alerting)
   - Set up [backups](../DEPLOYMENT.md#backup--recovery)

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Ensure the service is running: `docker ps` or check service status
   - Verify the port is correct and not blocked by a firewall

2. **Invalid API Key**
   - Double-check the `X-API-Key` header
   - Verify the API key has the required permissions

3. **Database Not Found**
   - Check the database paths in your `.env` file
   - Ensure the background updater has run successfully

4. **Rate Limit Exceeded**
   - Implement exponential backoff in your client
   - Consider upgrading your plan for higher limits

### Checking Logs

```bash
# Docker
docker-compose logs -f

# Systemd
journalctl -u geolocation -f

# Kubernetes
kubectl logs -l app=geolocation -n geolocation -f
```

## Getting Help

- **Documentation**:
  - [API Reference](../API_REFERENCE.md)
  - [Configuration Guide](../CONFIGURATION.md)
  - [Deployment Guide](../DEPLOYMENT.md)

- **Support**:
  - [GitHub Issues](https://github.com/your-org/geolocation/issues)
  - Email: [support@yourdomain.com](mailto:support@yourdomain.com)
  - Community Forum: [community.yourdomain.com](https://community.yourdomain.com)

- **Contributing**:
  We welcome contributions! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## License

[Your License Here] - See [LICENSE](../LICENSE) for more information.