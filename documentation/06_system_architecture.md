# System Architecture

## High-Level Architecture

InfraLock is designed as a distributed system with clear separation of concerns between the web API layer and the IP processing service. This architecture enables horizontal scaling and independent evolution of each component.

### Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Application                      │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Load Balancer (TODO)                      │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Web API Layer                           │
│  ┌─────────────────┐     ┌─────────────────┐     ┌───────────┐│
│  │   API Endpoints │◄───►│  Auth & Rate    │◄───►│  Redis    ││
│  │  (Express.js)   │     │     Limiting    │     │  Cache    ││
│  └─────────────────┘     └─────────────────┘     └───────────┘│
│           │                                                    │
│           ▼                                                    │
│  ┌─────────────────┐     ┌─────────────────┐                  │
│  │ Request/Response│     │  Error Handling │                  │
│  │   Processing    │◄───►│  & Logging      │                  │
│  └─────────────────┘     └─────────────────┘                  │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Rust IP Processing Service                  │
│  ┌─────────────────┐     ┌─────────────────┐     ┌───────────┐│
│  │   API Handlers  │◄───►│  IP Lookup      │◄───►│  Radix    ││
│  │  (axum)         │     │  Service        │     │  Tree     ││
│  └─────────────────┘     └─────────────────┘     └───────────┘│
│           │                                                    │
│           ▼                                                    │
│  ┌─────────────────┐     ┌─────────────────┐     ┌───────────┐│
│  │  Threat         │     │  GeoIP          │     │  Data     ││
│  │  Detection      │◄───►│  Lookup        │◄───►│  Loader   ││
│  │  (VPN, Proxy,   │     │  (MaxMind)     │     │  (Updates)││
│  │   TOR)          │     │                 │     │           ││
│  └─────────────────┘     └─────────────────┘     └───────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Client Request**:
   - The client sends an HTTP request to the Web API with an IP address to look up
   - The request is authenticated and rate-limited
   - The system checks the Redis cache for a cached response

2. **Request Processing**:
   - If not in cache, the request is forwarded to the Rust IP Processing Service
   - The service performs the IP lookup using the radix tree data structure
   - Additional threat detection modules are consulted (VPN, proxy, TOR)
   - Geolocation data is retrieved from the MaxMind database

3. **Response Generation**:
   - The results are compiled into a structured response
   - The response is cached in Redis for future requests
   - The response is returned to the client

### Communication Protocols

- **Web API to Client**: RESTful HTTP/HTTPS with JSON payloads
- **Web API to Rust Service**: HTTP/2 with Protocol Buffers for efficient binary serialization
- **Internal Service Communication**: Direct function calls within the same process
- **Cache Layer**: Redis protocol for in-memory caching

## Web API Service

### Architecture Overview

The Web API service is built on Node.js with Express.js, providing a robust and scalable foundation for handling HTTP requests. It serves as the entry point for all client interactions and is responsible for:

- Request validation and sanitization
- Authentication and authorization
- Rate limiting and request throttling
- Response formatting and error handling
- Caching layer integration

### Key Components

1. **API Endpoints**:
   - `GET /api/v1/lookup/:ip` - Look up information for a specific IP
   - `GET /api/v1/me` - Look up information for the client's IP
   - `GET /health` - Health check endpoint

2. **Middleware Layer**:
   - Authentication (API key validation)
   - Rate limiting
   - Request validation
   - Error handling
   - Logging and monitoring

3. **Service Layer**:
   - Handles business logic
   - Coordinates between different components
   - Manages caching strategies

### Performance Considerations

- **Stateless Design**: The API is stateless, allowing for horizontal scaling
- **Connection Pooling**: Database and Redis connections are pooled for efficiency
- **Response Caching**: Responses are cached to reduce load on the Rust service
- **Request Batching**: Multiple IP lookups can be batched in a single request
- **Compression**: Responses are compressed to reduce bandwidth usage

## Rust IP Processing Service

### Core Algorithms and Data Structures

The Rust service is the heart of the system, responsible for high-performance IP lookups and threat detection. It uses several advanced algorithms and data structures:

1. **Radix Tree**:
   - Custom implementation optimized for IP range lookups
   - Supports both IPv4 and IPv6 addresses
   - Provides O(k) lookup time, where k is the number of bits in the address

2. **IP Range Storage**:
   - Efficient storage of IP ranges with associated metadata
   - Support for overlapping ranges with priority-based conflict resolution
   - Memory-mapped files for persistence and fast startup

3. **Threat Detection**:
   - Multiple detection methods for different types of threats
   - Configurable confidence thresholds
   - Support for both IP-based and behavioral analysis

### Performance Optimization Techniques

1. **Zero-Copy Deserialization**:
   - Uses Serde with zero-copy deserialization for maximum performance
   - Minimizes memory allocations during request processing

2. **Asynchronous I/O**:
   - Built on Tokio for high-performance asynchronous I/O
   - Non-blocking operations for maximum concurrency

3. **Memory Management**:
   - Custom allocators for specific workloads
   - Object pooling for frequently allocated structures
   - Memory-mapped files for large datasets

4. **Parallel Processing**:
   - Multi-threaded execution for CPU-bound tasks
   - Work stealing for optimal load balancing

### Memory Management

The Rust service employs several techniques to manage memory efficiently:

1. **Arena Allocation**:
   - Custom allocator for IP range data
   - Reduces memory fragmentation
   - Improves cache locality

2. **Reference Counting**:
   - Smart pointers for shared ownership
   - Thread-safe reference counting for concurrent access

3. **Memory Mapping**:
   - Memory-mapped files for large datasets
   - Lazy loading of data on demand
   - Efficient sharing between processes

4. **Garbage Collection**:
   - Custom garbage collection for cached data
   - Generational garbage collection for temporary objects
   - Background compaction of memory

This architecture provides a solid foundation for high-performance IP intelligence while maintaining flexibility for future enhancements and scalability to handle increasing loads.
