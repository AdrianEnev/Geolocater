# API Reference

This document provides detailed information about the Geolocation API endpoints, request/response formats, and usage examples.

## Table of Contents

- [Base URL](#base-url)
- [Authentication](#authentication)
- [Rate Limiting](#rate-limiting)
- [Error Handling](#error-handling)
- [Endpoints](#endpoints)
  - [Health Check](#health-check)
  - [IP Lookup](#ip-lookup)
  - [Self Lookup](#self-lookup)
  - [Proxy Check](#proxy-check)
  - [VPN/Datacenter Check](#vpndatacenter-check)
  - [Batch Lookup](#batch-lookup)
  - [Metrics](#metrics)
- [Response Objects](#response-objects)
- [Examples](#examples)
- [Client Libraries](#client-libraries)

## Base URL

All API endpoints are relative to the base URL of your deployment:

```
https://api.yourdomain.com/v1
```

> **Note**: The `/v1` prefix indicates the API version. Future versions may introduce breaking changes under different version prefixes.

## Authentication

### API Key Authentication

Include your API key in the `X-API-Key` header:

```http
GET /api/health HTTP/1.1
Host: api.yourdomain.com
X-API-Key: your-api-key-here
```

### Required Permissions

| Endpoint | Permission Required |
|----------|---------------------|
| `/health` | None |
| `/lookup/*` | `geolocation:lookup` |
| `/batch/lookup` | `geolocation:batch_lookup` |
| `/is_proxy/*` | `geolocation:proxy_check` |
| `/is_vpn_or_datacenter/*` | `geolocation:vpn_check` |
| `/metrics` | `admin:metrics` |

## Rate Limiting

- **Rate Limit**: 100 requests per minute per IP address
- **Header Information**:
  - `X-RateLimit-Limit`: Maximum number of requests allowed in the time window
  - `X-RateLimit-Remaining`: Remaining number of requests in the current window
  - `X-RateLimit-Reset`: Time at which the current rate limit window resets (UTC epoch seconds)

Example response headers:
```
HTTP/1.1 200 OK
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1678901234
```

## Error Handling

### Error Response Format

All error responses follow this JSON structure:

```json
{
  "error": {
    "code": "error_code",
    "message": "Human-readable error message",
    "details": {
      "field_name": "Additional error details"
    }
  },
  "request_id": "a1b2c3d4e5f6g7h8"
}
```

### Common Error Codes

| Status Code | Error Code | Description |
|-------------|------------|-------------|
| 400 | `invalid_request` | Invalid request format or parameters |
| 401 | `unauthorized` | Missing or invalid authentication |
| 403 | `forbidden` | Insufficient permissions |
| 404 | `not_found` | Resource not found |
| 422 | `validation_error` | Request validation failed |
| 429 | `rate_limit_exceeded` | Rate limit exceeded |
| 500 | `internal_error` | Internal server error |
| 503 | `service_unavailable` | Service temporarily unavailable |

## Endpoints

### Health Check

Check if the service is running and healthy.

```http
GET /api/health
```

#### Response

```json
{
  "status": "ok",
  "version": "1.0.0",
  "timestamp": "2023-01-01T12:00:00Z",
  "services": {
    "database": true,
    "cache": true,
    "background_worker": true
  }
}
```

### IP Lookup

Get geolocation information for a specific IP address.

```http
GET /api/lookup/{ip}
```

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ip` | string | Yes | IPv4 or IPv6 address to look up |

#### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `language` | string | `en` | Language for location names (ISO 639-1) |
| `fields` | string | all | Comma-separated list of fields to return |

#### Response

```json
{
  "ip": "8.8.8.8",
  "location": {
    "city": {
      "name": "Mountain View",
      "names": {
        "en": "Mountain View",
        "es": "Mountain View"
      }
    },
    "country": {
      "code": "US",
      "name": "United States",
      "names": {
        "en": "United States",
        "es": "Estados Unidos"
      }
    },
    "continent": {
      "code": "NA",
      "name": "North America"
    },
    "coordinates": {
      "latitude": 37.386,
      "longitude": -122.0838,
      "time_zone": "America/Los_Angeles",
      "accuracy_radius": 1000
    }
  },
  "asn": {
    "number": 15169,
    "organization": "Google LLC",
    "domain": "google.com"
  },
  "security": {
    "is_proxy": false,
    "proxy_type": null,
    "is_vpn_or_datacenter": true,
    "threat_level": "low"
  },
  "network": {
    "network": "8.8.8.0/24",
    "autonomous_system_organization": "Google LLC",
    "isp": "Google"
  },
  "metadata": {
    "cached": false,
    "processed_in_ms": 12.5
  }
}
```

### Self Lookup

Get geolocation information for the client's IP address.

```http
GET /api/lookup/self
```

This endpoint accepts the same query parameters as the IP Lookup endpoint.

### Proxy Check

Check if an IP is a known proxy and get its type.

```http
GET /api/is_proxy/{ip_or_range}
```

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ip_or_range` | string | Yes | IPv4, IPv6 address, or CIDR range |

#### Response

```json
{
  "ip": "1.2.3.4",
  "is_proxy": true,
  "proxy_type": "HTTP/HTTPS",
  "last_seen": "2023-01-01T12:00:00Z",
  "threat_level": "medium"
}
```

### VPN/Datacenter Check

Check if an IP or CIDR range is associated with a VPN or datacenter.

```http
GET /api/is_vpn_or_datacenter/{ip_or_range}
```

#### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ip_or_range` | string | Yes | IPv4, IPv6 address, or CIDR range |

#### Response

```json
{
  "ip": "1.2.3.4",
  "is_vpn_or_datacenter": true,
  "type": "datacenter",
  "organization": "Amazon Technologies Inc.",
  "last_updated": "2023-01-01T00:00:00Z"
}
```

### Batch Lookup

Perform lookups for multiple IP addresses in a single request.

```http
POST /api/batch/lookup
```

#### Request Body

```json
{
  "ips": ["8.8.8.8", "1.1.1.1", "2001:4860:4860::8888"],
  "language": "en",
  "fields": ["ip", "location.country.name", "security"]
}
```

#### Response

```json
{
  "results": [
    {
      "ip": "8.8.8.8",
      "location": {
        "country": {
          "name": "United States"
        }
      },
      "security": {
        "is_proxy": false,
        "is_vpn_or_datacenter": true
      }
    },
    {
      "ip": "1.1.1.1",
      "location": {
        "country": {
          "name": "United States"
        }
      },
      "security": {
        "is_proxy": false,
        "is_vpn_or_datacenter": true
      }
    },
    {
      "ip": "2001:4860:4860::8888",
      "location": {
        "country": {
          "name": "United States"
        }
      },
      "security": {
        "is_proxy": false,
        "is_vpn_or_datacenter": true
      }
    }
  ],
  "metadata": {
    "processed_in_ms": 25.8,
    "cached_entries": 2,
    "total_ips": 3
  }
}
```

### Metrics

Get Prometheus-compatible metrics.

```http
GET /api/metrics
```

#### Response

```
# HELP http_requests_total Total number of HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",endpoint="/health",status="200"} 42
http_requests_total{method="GET",endpoint="/lookup",status="200"} 123

# HELP http_request_duration_seconds HTTP request duration in seconds
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.1"} 100
http_request_duration_seconds_bucket{le="0.5"} 200
http_request_duration_seconds_bucket{le="1"} 250
http_request_duration_seconds_sum 123.45
http_request_duration_seconds_count 250
```

## Response Objects

### Location Object

```json
{
  "city": {
    "name": "Mountain View",
    "names": {
      "en": "Mountain View",
      "es": "Mountain View"
    },
    "geoname_id": 5375480
  },
  "country": {
    "code": "US",
    "name": "United States",
    "names": {
      "en": "United States",
      "es": "Estados Unidos"
    },
    "geoname_id": 6252001,
    "is_in_european_union": false
  },
  "continent": {
    "code": "NA",
    "name": "North America",
    "geoname_id": 6255149
  },
  "coordinates": {
    "latitude": 37.386,
    "longitude": -122.0838,
    "time_zone": "America/Los_Angeles",
    "accuracy_radius": 1000
  }
}
```

### Security Object

```json
{
  "is_proxy": false,
  "proxy_type": null,
  "is_vpn_or_datacenter": true,
  "threat_level": "low",
  "last_seen": "2023-01-01T12:00:00Z"
}
```

## Examples

### cURL Example

```bash
# IP Lookup
curl -X GET "https://api.yourdomain.com/v1/lookup/8.8.8.8" \
     -H "X-API-Key: your-api-key"

# Batch Lookup
curl -X POST "https://api.yourdomain.com/v1/batch/lookup" \
     -H "Content-Type: application/json" \
     -H "X-API-Key: your-api-key" \
     -d '{"ips": ["8.8.8.8", "1.1.1.1"]}'
```

### Python Example

```python
import requests

BASE_URL = "https://api.yourdomain.com/v1"
API_KEY = "your-api-key"

headers = {
    "X-API-Key": API_KEY,
    "Content-Type": "application/json"
}

# Single IP lookup
response = requests.get(f"{BASE_URL}/lookup/8.8.8.8", headers=headers)
print(response.json())

# Batch lookup
batch_data = {
    "ips": ["8.8.8.8", "1.1.1.1"],
    "language": "en",
    "fields": ["ip", "location.country.name", "security"]
}
response = requests.post(
    f"{BASE_URL}/batch/lookup",
    headers=headers,
    json=batch_data
)
print(response.json())
```

## Client Libraries

### Official Libraries

- **Python**: `pip install geolocation-api-client`
- **Node.js**: `npm install geolocation-api-client`
- **Go**: `go get github.com/your-org/geolocation-go-client`

### Community Libraries

- **Ruby**: `gem install geolocation-api`
- **PHP**: `composer require your-org/geolocation-php`
- **Java**: Add Maven dependency `com.your-org:geolocation-java:1.0.0`

## Versioning

This API is versioned using the URL path (e.g., `/v1/...`). Breaking changes will be introduced in new versions, and older versions will be maintained for a reasonable period.

## Support

For support, please contact [support@yourdomain.com](mailto:support@yourdomain.com) or open an issue in our [GitHub repository](https://github.com/your-org/geolocation).