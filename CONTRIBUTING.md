# Contributing to Puller

Thank you for your interest in contributing to Puller! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/puller.git
   cd puller
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## Development

### Project Structure

```
src/
├── main.rs           # CLI entry point and orchestration
├── error.rs          # Custom error types
├── config.rs         # Platform API configuration
├── platform.rs       # Platform enum definitions
├── article.rs        # Article struct and frontmatter generation
├── state.rs          # Pull state tracking
├── writer.rs         # Write articles to Markdown files
└── adapters/
    ├── mod.rs        # Puller trait definition
    └── devto.rs      # Dev.to API implementation
```

## Making Changes

### Code Style

- Follow standard Rust conventions and idioms
- Run `cargo fmt` before committing
- Run `cargo clippy` and address any warnings
- Add documentation for public APIs

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 72 characters

### Pull Requests

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes and commit them
3. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
4. Open a pull request against the `main` branch

### PR Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Ensure all tests pass
- Update documentation if needed

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

## Reporting Issues

When reporting issues, please include:

- A clear description of the problem
- Steps to reproduce the issue
- Expected vs actual behavior
- Rust version (`rustc --version`)
- Operating system

## Questions

If you have questions, feel free to open an issue for discussion.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
