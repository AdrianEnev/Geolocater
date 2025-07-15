# Deployment Guide

This guide covers the deployment of the Geolocation API in various environments, from development to production.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Building for Production](#building-for-production)
- [Docker Deployment](#docker-deployment)
- [Systemd Service](#systemd-service)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Reverse Proxy Setup](#reverse-proxy-setup)
- [Monitoring and Logging](#monitoring-and-logging)
- [Backup and Recovery](#backup-and-recovery)
- [Scaling](#scaling)
- [Security Considerations](#security-considerations)

## Prerequisites

- Linux/Unix-based system (Ubuntu 20.04+ recommended)
- Rust toolchain (stable)
- Git
- Required system libraries (OpenSSL, etc.)
- Root/sudo access (for system-wide installation)
- Domain name (for production)
- SSL certificates (for HTTPS)

## Building for Production

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/geolocation.git
   cd geolocation
   ```

2. Build the release binary:
   ```bash
   cargo build --release
   ```

   The binary will be available at `./target/release/geolocation`.

3. Install system dependencies:
   ```bash
   # For Ubuntu/Debian
   sudo apt update
   sudo apt install -y libssl-dev pkg-config
   ```

## Docker Deployment

### Building the Docker Image

1. Build the Docker image:
   ```bash
   docker build -t geolocation:latest .
   ```

2. Run the container:
   ```bash
   docker run -d \
     --name geolocation \
     -p 3000:3000 \
     --restart unless-stopped \
     -v $(pwd)/data:/app/data \
     --env-file .env \
     geolocation:latest
   ```

### Docker Compose

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  geolocation:
    image: geolocation:latest
    container_name: geolocation
    restart: unless-stopped
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
    env_file:
      - .env
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/api/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 5s
```

Start the service:

```bash
docker-compose up -d
```

## Systemd Service

For production deployments on Linux, it's recommended to run the service as a systemd service.

1. Create a service file at `/etc/systemd/system/geolocation.service`:

   ```ini
   [Unit]
   Description=Geolocation API Service
   After=network.target

   [Service]
   Type=simple
   User=geolocation
   Group=geolocation
   WorkingDirectory=/opt/geolocation
   ExecStart=/opt/geolocation/target/release/geolocation
   Restart=always
   RestartSec=10
   EnvironmentFile=/opt/geolocation/.env
   StandardOutput=journal
   StandardError=journal
   SyslogIdentifier=geolocation

   [Install]
   WantedBy=multi-user.target
   ```

2. Set up the service user and permissions:
   ```bash
   sudo useradd -r -s /sbin/nologin geolocation
   sudo mkdir -p /opt/geolocation/data
   sudo chown -R geolocation:geolocation /opt/geolocation
   ```

3. Enable and start the service:
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable geolocation
   sudo systemctl start geolocation
   ```

4. Check the service status:
   ```bash
   sudo systemctl status geolocation
   ```

## Kubernetes Deployment

### Deployment Configuration

Create a `geolocation-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: geolocation
  labels:
    app: geolocation
spec:
  replicas: 3
  selector:
    matchLabels:
      app: geolocation
  template:
    metadata:
      labels:
        app: geolocation
    spec:
      containers:
      - name: geolocation
        image: your-registry/geolocation:latest
        ports:
        - containerPort: 3000
        envFrom:
        - configMapRef:
            name: geolocation-config
        - secretRef:
            name: geolocation-secrets
        resources:
          requests:
            cpu: "100m"
            memory: "256Mi"
          limits:
            cpu: "500m"
            memory: "512Mi"
        livenessProbe:
          httpGet:
            path: /api/health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
```

### Service Configuration

Create a `geolocation-service.yaml`:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: geolocation
spec:
  selector:
    app: geolocation
  ports:
  - protocol: TCP
    port: 80
    targetPort: 3000
  type: ClusterIP
```

### Ingress Configuration

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: geolocation
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: api.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: geolocation
            port:
              number: 80
  tls:
  - hosts:
    - api.yourdomain.com
    secretName: geolocation-tls
```

## Reverse Proxy Setup

### Nginx Configuration

```nginx
server {
    listen 80;
    server_name api.yourdomain.com;
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/api.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourdomain.com/privkey.pem;
    
    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384';
    
    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;
    add_header Content-Security-Policy "default-src 'self' http: https: data: blob: 'unsafe-inline'" always;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        
        # Rate limiting
        limit_req zone=api_limit burst=20 nodelay;
    }
    
    # Rate limiting configuration
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
    
    # Logging
    access_log /var/log/nginx/geolocation_access.log;
    error_log /var/log/nginx/geolocation_error.log;
}
```

## Monitoring and Logging

### Prometheus Metrics

The service exposes Prometheus metrics at `/metrics`.

### Logging Configuration

Configure logging using the `RUST_LOG` environment variable:

```bash
# Development
RUST_LOG=geolocation=debug,tower_http=debug

# Production
RUST_LOG=geolocation=info,tower_http=info
```

### Log Rotation

Add to `/etc/logrotate.d/geolocation`:

```
/var/log/geolocation/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 geolocation adm
    sharedscripts
    postrotate
        systemctl reload geolocation > /dev/null 2>/dev/null || true
    endscript
}
```

## Backup and Recovery

### Database Backup

Create a backup script at `/usr/local/bin/backup-geolocation.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/var/backups/geolocation"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

# Stop the service to ensure data consistency
systemctl stop geolocation

# Create backup
tar -czf "$BACKUP_DIR/geolocation_$TIMESTAMP.tar.gz" /opt/geolocation/data

# Restart the service
systemctl start geolocation

# Remove backups older than 30 days
find $BACKUP_DIR -name "geolocation_*.tar.gz" -type f -mtime +30 -delete
```

Make it executable:
```bash
chmod +x /usr/local/bin/backup-geolocation.sh
```

### Cron Job for Backups

Add to `/etc/crontab`:
```
0 2 * * * root /usr/local/bin/backup-geolocation.sh
```

## Scaling

### Horizontal Scaling

1. **Stateless Architecture**: The service is designed to be stateless, allowing easy horizontal scaling.
2. **Load Balancing**: Use a load balancer to distribute traffic across multiple instances.
3. **Database Considerations**: When scaling, consider using a shared filesystem or distributed cache for the MaxMind database.

### Vertical Scaling

1. **Memory**: Increase memory allocation for handling larger datasets.
2. **CPU**: Add more CPU cores for concurrent request handling.

## Security Considerations

1. **Network Security**:
   - Use a firewall to restrict access to the service
   - Implement rate limiting
   - Use private networking where possible

2. **Data Security**:
   - Encrypt sensitive data at rest
   - Use HTTPS for all communications
   - Regularly update dependencies

3. **Authentication**:
   - Implement API key authentication
   - Use OAuth2 for user authentication if needed

4. **Updates**:
   - Regularly update the service and its dependencies
   - Monitor for security advisories

## Troubleshooting

### Common Issues

1. **Port Already in Use**:
   ```bash
   sudo lsof -i :3000
   sudo kill <PID>
   ```

2. **Permission Issues**:
   ```bash
   sudo chown -R geolocation:geolocation /opt/geolocation
   ```

3. **Service Not Starting**:
   ```bash
   journalctl -u geolocation -f
   ```

### Getting Help

If you encounter issues, please check:
1. Service logs: `journalctl -u geolocation -f`
2. Open an issue on our GitHub repository
3. Check the documentation for configuration options