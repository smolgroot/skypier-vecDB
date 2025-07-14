<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# SkyPier VecDB - Decentralized Vector Database

This is a high-performance, decentralized vector database written in Rust for AI infrastructure. 

## Architecture Guidelines

- **Modular Design**: Use the crate-based architecture with clear separation of concerns
- **Async/Await**: Use tokio for async operations throughout the codebase
- **Error Handling**: Use `anyhow::Result` for error handling and proper error propagation
- **Performance**: Optimize for vector similarity search performance and network efficiency
- **Safety**: Leverage Rust's memory safety and type system

## Key Components

1. **skypier-core**: Core vector database functionality and data structures
2. **skypier-storage**: Persistent storage using redb (embedded key-value store)
3. **skypier-index**: Vector indexing with HNSW algorithm for fast similarity search
4. **skypier-network**: P2P networking, consensus, and replication using libp2p

## Development Practices

- Follow Rust best practices and idioms
- Use comprehensive error handling with meaningful error messages
- Implement proper logging with the `tracing` crate
- Write unit tests for all core functionality
- Use appropriate data structures for vector operations
- Optimize for both memory usage and computational efficiency

## Dependencies

- **tokio**: Async runtime
- **libp2p**: P2P networking stack
- **redb**: Embedded database for persistence
- **axum**: Web framework for REST API
- **serde**: Serialization/deserialization
- **anyhow/thiserror**: Error handling
- **tracing**: Structured logging
