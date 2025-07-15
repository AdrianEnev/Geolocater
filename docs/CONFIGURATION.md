# Configuration Guide

This document explains all configuration options available for the Geolocation API service. Configuration is done through environment variables, which can be set in your environment or in a `.env` file.

## Table of Contents

- [Server Configuration](#server-configuration)
- [Database Paths](#database-paths)
- [Background Updater](#background-updater)
- [Logging](#logging)
- [Example .env File](#example-env-file)
- [Configuration Best Practices](#configuration-best-practices)

## Server Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `GEO_SERVER__HOST` | String | `0.0.0.0` | The host address to bind the server to |
| `GEO_SERVER__PORT` | Integer | `3000` | The port to listen on |

## Database Paths

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `GEO_MAXMIND__DB_PATH` | Path | `data/GeoLite2-City.mmdb` | Path to MaxMind GeoIP2 database |
| `GEO_VPN_DETECTOR__DB_PATH` | Path | `data/vpns/ipv4.txt` | Path to VPN/Datacenter IP list |
| `GEO_PROXY_DETECTOR__HTTP_DB_PATH` | Path | `data/proxies/http.txt` | Path to HTTP/HTTPS proxy list |
| `GEO_PROXY_DETECTOR__SOCKS4_DB_PATH` | Path | `data/proxies/socks4.txt` | Path to SOCKS4 proxy list |
| `GEO_PROXY_DETECTOR__SOCKS5_DB_PATH` | Path | `data/proxies/socks5.txt` | Path to SOCKS5 proxy list |

## Background Updater

The background updater automatically refreshes the VPN and proxy lists from remote sources.

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `GEO_BACKGROUND_UPDATER__ENABLED` | Boolean | `true` | Enable/disable background updates |
| `GEO_BACKGROUND_UPDATER__INTERVAL_SECS` | Integer | `86400` (24 hours) | Seconds between update checks |
| `GEO_BACKGROUND_UPDATER__VPN_URL` | URL | - | URL to fetch VPN list from |
| `GEO_BACKGROUND_UPDATER__HTTP_PROXY_URL` | URL | - | URL to fetch HTTP/HTTPS proxy list from |
| `GEO_BACKGROUND_UPDATER__SOCKS4_PROXY_URL` | URL | - | URL to fetch SOCKS4 proxy list from |
| `GEO_BACKGROUND_UPDATER__SOCKS5_PROXY_URL` | URL | - | URL to fetch SOCKS5 proxy list from |

## Logging

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RUST_LOG` | String | `geolocation=info,tower_http=info` | Log level configuration |

## Example .env File

```env
# Server Configuration
GEO_SERVER__HOST=0.0.0.0
GEO_SERVER__PORT=3000

# Database Paths
GEO_MAXMIND__DB_PATH=data/GeoLite2-City.mmdb
GEO_VPN_DETECTOR__DB_PATH=data/vpns/ipv4.txt
GEO_PROXY_DETECTOR__HTTP_DB_PATH=data/proxies/http.txt
GEO_PROXY_DETECTOR__SOCKS4_DB_PATH=data/proxies/socks4.txt
GEO_PROXY_DETECTOR__SOCKS5_DB_PATH=data/proxies/socks5.txt

# Background Updater
GEO_BACKGROUND_UPDATER__ENABLED=true
GEO_BACKGROUND_UPDATER__INTERVAL_SECS=86400
GEO_BACKGROUND_UPDATER__VPN_URL=https://example.com/vpn.txt
GEO_BACKGROUND_UPDATER__HTTP_PROXY_URL=https://example.com/http.txt
GEO_BACKGROUND_UPDATER__SOCKS4_PROXY_URL=https://example.com/socks4.txt
GEO_BACKGROUND_UPDATER__SOCKS5_PROXY_URL=https://example.com/socks5.txt

# Logging
RUST_LOG=geolocation=info,tower_http=info
```

## Configuration Best Practices

1. **Environment Variables**:
   - Use `.env` files for development
   - Use environment variables in production
   - Never commit sensitive data to version control

2. **File Paths**:
   - Use absolute paths in production for reliability
   - Ensure the service has read/write permissions to the specified directories

3. **Background Updater**:
   - Set appropriate update intervals based on your requirements
   - Monitor the logs for update status
   - Ensure the service has internet access to download updates

4. **Security**:
   - Keep your MaxMind database up to date
   - Use HTTPS for remote updates
   - Restrict access to the service using firewalls or API gateways

5. **Performance**:
   - Place database files on fast storage
   - Monitor memory usage with large IP lists
   - Adjust log levels in production to balance verbosity and performance