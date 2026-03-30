# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## ⚠️ Important: Telegram-Only Library

**This library is designed exclusively for Telegram Mini Apps.** All components and utilities are intended to be used within Telegram's environment.

## Reporting a Vulnerability

If you discover a security vulnerability, please follow these steps:

1. **Do not** open a public issue
2. Send an email to security@telegram-ui.dev
3. Include details about the vulnerability
4. Allow us 7 days to respond before public disclosure

## Security Practices

This project follows security best practices:

- Regular dependency updates
- Security audit integration
- No hardcoded credentials
- Input validation
- Secure coding standards

## Dependencies

All dependencies are managed through Cargo and audited regularly using rustsec.

## Disclosure Policy

We follow a 7-day disclosure policy for security vulnerabilities.

## Security Features

- ✅ No hardcoded secrets
- ✅ Dependency auditing
- ✅ Type-safe Rust code
- ✅ Memory safety guaranteed
- ✅ No unsafe code (where possible)

## ⚠️ Telegram-Only Environment

Since this library is designed exclusively for Telegram Mini Apps:
- All security considerations apply within Telegram's secure environment
- Data transmitted through Telegram WebApp SDK is protected by Telegram's infrastructure
- No additional security measures needed for non-Telegram usage (as it's not supported)
