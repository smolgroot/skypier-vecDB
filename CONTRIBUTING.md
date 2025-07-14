# Contributing to SkyPier VecDB

Thank you for your interest in contributing to SkyPier VecDB! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A GitHub account

### Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork locally**:
   ```bash
   git clone https://github.com/your-username/skypier-vecdb.git
   cd skypier-vecdb
   ```

3. **Set up the upstream remote**:
   ```bash
   git remote add upstream https://github.com/original-owner/skypier-vecdb.git
   ```

4. **Install development dependencies**:
   ```bash
   # Install rustfmt and clippy
   rustup component add rustfmt clippy
   
   # Install cargo-audit for security checks
   cargo install cargo-audit
   
   # Install tarpaulin for coverage (Linux only)
   cargo install cargo-tarpaulin
   ```

## Development Workflow

### Before Making Changes

1. **Create a new branch** for your feature/fix:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number
   ```

2. **Make sure everything works**:
   ```bash
   cargo build
   cargo test
   ```

### Making Changes

1. **Follow Rust conventions**:
   - Use `cargo fmt` to format your code
   - Run `cargo clippy` to catch common mistakes
   - Write documentation for public APIs
   - Add tests for new functionality

2. **Test your changes**:
   ```bash
   # Format code
   cargo fmt
   
   # Check for issues
   cargo clippy
   
   # Run all tests
   cargo test
   
   # Run benchmarks (optional)
   cargo bench
   ```

3. **Security and dependency checks**:
   ```bash
   cargo audit
   ```

### Commit Guidelines

- Use clear, descriptive commit messages
- Follow conventional commit format:
  - `feat: add new vector search algorithm`
  - `fix: resolve memory leak in storage layer`
  - `docs: update API documentation`
  - `test: add unit tests for similarity functions`
  - `refactor: simplify database initialization`
  - `perf: optimize HNSW index construction`

### Testing

- **Unit tests**: Test individual components
- **Integration tests**: Test API endpoints and workflows
- **Benchmarks**: Measure performance of critical paths
- **Documentation tests**: Ensure code examples work

```bash
# Run specific test suites
cargo test --lib                    # Library tests
cargo test --bin skypier-vecdb     # Binary tests
cargo test --doc                   # Documentation tests
cargo bench                        # Benchmarks

# Test with different features
cargo test --all-features
cargo test --no-default-features
```

## Code Organization

The project follows a modular architecture:

```
skypier-vecdb/
├── src/                    # Main application
│   ├── main.rs            # Entry point
│   ├── api.rs             # HTTP API (with tests)
│   └── config.rs          # Configuration
├── crates/                # Workspace crates
│   ├── skypier-core/      # Core database logic
│   ├── skypier-storage/   # Persistent storage
│   ├── skypier-index/     # Vector indexing
│   └── skypier-network/   # P2P networking
├── benches/               # Performance benchmarks
└── .github/workflows/     # CI/CD pipelines
```

## Quality Standards

### Code Quality

- **No compiler warnings**: Code must compile without warnings
- **Clippy compliance**: Must pass `cargo clippy` without errors
- **Formatting**: Must be formatted with `cargo fmt`
- **Documentation**: Public APIs must be documented
- **Tests**: New features must include tests

### Performance

- **Benchmarks**: Performance-critical code should include benchmarks
- **Memory efficiency**: Avoid unnecessary allocations
- **Async/await**: Use async for I/O operations
- **Zero-copy**: Prefer zero-copy operations where possible

### Security

- **No unsafe code**: Unless absolutely necessary and well-documented
- **Dependency audit**: Regular security audits with `cargo audit`
- **Input validation**: Validate all external inputs
- **Error handling**: Proper error handling with `anyhow::Result`

## Submitting Changes

### Pull Request Process

1. **Update your branch** with upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your changes**:
   ```bash
   git push origin your-branch-name
   ```

3. **Create a Pull Request** on GitHub with:
   - Clear title and description
   - Reference to related issues
   - Screenshots/examples if applicable
   - Checklist of completed items

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] New tests added for new features
- [ ] Security audit passes (`cargo audit`)
- [ ] Benchmarks updated if performance-critical

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Environment**: OS, Rust version, dependencies
- **Steps to reproduce**: Minimal example
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Logs/errors**: Relevant error messages

### Feature Requests

For feature requests, please provide:

- **Use case**: Why is this needed?
- **Proposed solution**: How should it work?
- **Alternatives**: Other approaches considered
- **Implementation ideas**: Technical considerations

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and ideas
- **Documentation**: Check the README and API docs
- **Code Review**: Ask for feedback on draft PRs

## Recognition

Contributors will be:

- Listed in the project's contributors
- Credited in release notes for significant contributions
- Invited to join the core team for ongoing contributors

Thank you for contributing to SkyPier VecDB! 🚀
