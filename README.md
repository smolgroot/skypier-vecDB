# Skypier VecDB

[![CI](https://github.com/smolgroot/skypier-vecdb/workflows/CI/badge.svg)](https://github.com/smolgroot/skypier-vecdb/actions)
[![Coverage](https://codecov.io/gh/smolgroot/skypier-vecdb/branch/main/graph/badge.svg)](https://codecov.io/gh/smolgroot/skypier-vecdb)
[![Security Audit](https://github.com/smolgroot/skypier-vecdb/workflows/Security%20Audit/badge.svg)](https://github.com/smolgroot/skypier-vecdb/actions)
[![Crates.io](https://img.shields.io/crates/v/skypier-vecdb.svg)](https://crates.io/crates/skypier-vecdb)
[![Documentation](https://docs.rs/skypier-vecdb/badge.svg)](https://docs.rs/skypier-vecdb)

A high-performance, decentralized minimal vector database written in Rust for AI infrastructure. Built from the ground up for distributed environments.

## Features

- 🚀 **High Performance**: Rust-based implementation with zero-cost abstractions
- 🔗 **Decentralized**: P2P networking with libp2p for distributed operations
- 🎯 **HNSW Indexing**: Hierarchical Navigable Small World graphs for fast similarity search
- 💾 **Embedded Storage**: ReDB for efficient key-value storage without external dependencies
- 🌐 **REST API**: Clean HTTP API for easy integration
- 🔍 **Multiple Distance Metrics**: Support for cosine, euclidean, and dot product similarity
- 📦 **Collection Support**: Organize vectors into collections for better data management

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     SkyPier VecDB                           │
├─────────────────────────────────────────────────────────────┤
│  REST API (Axum)           │  P2P Network (libp2p)          │
│  - Insert vectors          │  - Gossipsub messaging         │
│  - Search similarity       │  - Kademlia DHT                │
│  - Manage collections      │  - mDNS discovery              │
├─────────────────────────────────────────────────────────────┤
│  Core Database Engine      |                                │
│  - Vector operations       │  - HNSW Index                  │
│  - Similarity computation  │  - Flat index fallback         │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer (ReDB)      |                                │
│  - Persistent vectors      │  - Metadata storage            │
│  - ACID transactions       │  - Backup/restore              │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70+ (installed automatically if not present)

### Building

```bash
# Clone the repository
git clone <repository-url>
cd skypier-vecDB

# Build the project
cargo build --release

# Run tests
cargo test
```

### Running

```bash
# Start the database server
cargo run -- --port 8080 --p2p-port 7777

# Or with custom configuration
cargo run -- --config config.toml
```

### API Usage

#### Insert Vectors

```bash
curl -X POST http://localhost:8080/vectors \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": [
      {
        "id": "doc1",
        "data": [0.1, 0.2, 0.3, 0.4],
        "metadata": {"title": "Example Document"},
        "collection": "documents"
      }
    ]
  }'
```

#### Search Vectors

```bash
curl -X POST http://localhost:8080/search \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.1, 0.2, 0.3, 0.4],
    "k": 10,
    "threshold": 0.5
  }'
```

#### Get Statistics

```bash
curl http://localhost:8080/stats
```

## Configuration

Create a `config.toml` file:

```toml
[server]
host = "0.0.0.0"
port = 8080

[p2p]
port = 7777
bootstrap_peers = []
max_peers = 50

[storage]
data_dir = "./data"
max_file_size = 1073741824  # 1GB
compression = true

[index]
index_type = "embedded"  # or "faiss"
dimensions = 768
distance_metric = "cosine"  # "euclidean", "dot_product"
ef_construction = 200
ef_search = 50
max_connections = 16
```

## Development

### Project Structure

```
skypier-vecDB/
├── src/                    # Main application
│   ├── main.rs            # Entry point
│   ├── api.rs             # HTTP API handlers
│   └── config.rs          # Configuration management
├── crates/                # Modular crates
│   ├── skypier-core/      # Core database logic
│   ├── skypier-storage/   # Storage abstraction & ReDB
│   ├── skypier-index/     # HNSW and indexing algorithms
│   └── skypier-network/   # P2P networking & consensus
└── Cargo.toml            # Workspace configuration
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p skypier-core

# Run with logging
RUST_LOG=debug cargo test

# Run only unit tests
cargo test --bin skypier-vecdb

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --all-features --workspace
```

### Continuous Integration

This project uses GitHub Actions for CI/CD:

- **Test Suite**: Runs on every push and PR (stable, beta, nightly Rust)
- **Code Coverage**: Generates coverage reports using cargo-tarpaulin
- **Security Audit**: Runs cargo-audit to check for vulnerabilities
- **Cross-platform Builds**: Tests on Linux, Windows, and macOS
- **Documentation**: Auto-generates and deploys docs to GitHub Pages
- **Release Automation**: Creates releases with binaries for multiple platforms

[![CI](https://github.com/smolgroot/skypier-vecdb/workflows/CI/badge.svg)](https://github.com/smolgroot/skypier-vecdb/actions)
[![Coverage](https://codecov.io/gh/smolgroot/skypier-vecdb/branch/main/graph/badge.svg)](https://codecov.io/gh/smolgroot/skypier-vecdb)
````
