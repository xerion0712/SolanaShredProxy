# SolanaShredProxy

SolanaShredProxy is a Rust-based proxy for Solana ShredStream, providing ultra-low latency access to leader shreds on the Solana blockchain.

---

## Disclaimer
Use this software at your own risk. The developers do not assume responsibility for losses or network issues.  

---

## Features
- Low-latency access to leader shreds on Solana  
- Configurable target regions for optimized connectivity  
- gRPC service for downstream clients  
- Authenticated access via keypair  
- Configurable logging and connection settings
- Health check endpoints

---

## Prerequisites
- Rust (latest stable recommended) - [Install Rust](https://rustup.rs/)
- Linux system with GLIBC ≥ 2.35 (for production)
- Windows/macOS for development

---

## Installation & Build

### 1. Clone the repository
```bash
git clone https://github.com/xerion0712/solanashredproxy.git
cd solanashredproxy
```

### 2. Build the project
```bash
# Build in debug mode
cargo build

# Build in release mode (recommended for production)
cargo build --release
```

### 3. Run the application
```bash
# Run with default configuration
cargo run

# Run with custom configuration file
cargo run -- --config custom_config.toml

# Run with debug logging
cargo run -- --debug

# Run the release binary directly
./target/release/solanashredproxy
```

---

## Configuration

The application uses a TOML configuration file (`config.toml` by default). Key configuration options:

- `solana_rpc_url`: Solana RPC endpoint
- `grpc_address`: gRPC server address and port
- `target_regions`: List of target regions for optimized connectivity
- `keypair_path`: Path to authentication keypair (optional)
- `log_level`: Logging level (trace, debug, info, warn, error)
- `connection_timeout`: Connection timeout in seconds
- `max_connections`: Maximum concurrent connections

---

## Usage

### Starting the service
```bash
# Basic startup
cargo run

# With custom config
cargo run -- --config production.toml

# With debug logging
cargo run -- --debug
```

### gRPC Client Connection
The service exposes a gRPC server on the configured address (default: 127.0.0.1:50051).

Available RPC methods:
- `StreamShreds`: Stream shreds from Solana network
- `GetShred`: Get specific shred by ID
- `HealthCheck`: Service health check

---

## Development

### Project Structure
```
src/
├── main.rs          # Application entry point
├── config.rs        # Configuration management
├── proxy.rs         # Main proxy logic
├── grpc.rs          # gRPC server implementation
└── shred_stream.rs  # Solana shred stream handling

proto/
└── solana_shred.proto  # Protocol buffer definitions

build.rs              # Build script for protobuf compilation
Cargo.toml            # Rust dependencies and project metadata
config.toml           # Default configuration
```

### Adding new features
1. Implement the feature in the appropriate module
2. Add configuration options if needed
3. Update the protobuf definitions if adding new RPC methods
4. Add tests and documentation

---

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

---

## Troubleshooting

### Common Issues

1. **Build fails with protobuf errors**
   - Ensure `tonic-build` is in build-dependencies
   - Check that proto files are in the correct location

2. **Connection timeout errors**
   - Verify Solana RPC endpoint is accessible
   - Check network connectivity
   - Adjust `connection_timeout` in config

3. **gRPC server won't start**
   - Check if port is already in use
   - Verify `grpc_address` format in config
   - Check firewall settings

### Logs
Enable debug logging with `--debug` flag to get detailed information about what's happening.

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

---

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## Support

For issues and questions:
- Open an issue on GitHub
- Check the troubleshooting section
- Review the configuration options
