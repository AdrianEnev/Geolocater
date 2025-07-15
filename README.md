# Geolocation API

A high-performance geolocation API service built with Rust and Axum.

## Features

- Fast IP geolocation lookups
- VPN and datacenter IP detection
- Proxy detection (HTTP/HTTPS, SOCKS4, SOCKS5)
- Automatic background updates
- RESTful API with JSON responses

## Quick Start

```bash
# 1. Clone and build
git clone <repository-url>
cd geolocation
cargo build --release

# 2. Configure (see docs/CONFIGURATION.md)
cp .env.example .env
# Edit .env with your settings

# 3. Run
./target/release/geolocation
```

## Documentation

The listed documentation files are not yet finished and have been set up as templates temporarily

- [Configuration](docs/CONFIGURATION.md)
- [Getting Started](docs/GETTING_STARTED.md)
- [API Reference](docs/API_REFERENCE.md)
- [Background Updater](docs/BACKGROUND_UPDATER.md)
- [Deployment](docs/DEPLOYMENT.md)

## License

This project is licensed under the MIT License