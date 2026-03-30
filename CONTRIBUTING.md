# Contributing to Telegram UI

Thank you for your interest in contributing to Telegram UI! This document provides guidelines for contributing to this project.

## ⚠️ Important: Telegram-Only Library

**This library is designed exclusively for Telegram Mini Apps.** All components and utilities are intended to be used within Telegram's environment. The Telegram WebApp SDK is only available when your app is running inside Telegram.

## Code of Conduct

Please be respectful and constructive in all interactions.

## How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/telegram-ui/telegram-ui.git
cd telegram-ui

# Install trunk for building
cargo install trunk

# Run tests
cargo test

# Run linter
cargo clippy --all-targets

# Format code
cargo fmt
```

## Development Guidelines

### Code Style

- Follow Rust API Guidelines
- Use `clippy` to catch common mistakes
- Format code with `rustfmt`
- Write comprehensive documentation

### Commit Messages

Use conventional commits format:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `chore:` - Maintenance tasks

Example: `feat: add input component with validation`

### Pull Request Process

1. Update documentation as needed
2. Add tests for new functionality
3. Ensure all checks pass
4. Get at least one approval from maintainers
5. Maintain code quality standards

### Testing

- Write unit tests for new features
- Ensure all existing tests pass
- Add integration tests for complex features
- Test with both Leptos and Yew frameworks

### Documentation

- Document all public APIs
- Add examples where appropriate
- Update README for major changes
- Document breaking changes clearly

## Questions?

Feel free to open an issue for any questions about contributing.
