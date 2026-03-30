<!-- SPDX-License-Identifier: MIT -->
<!-- SPDX-FileCopyrightText: 2026 Telegram UI contributors -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-29

### ⚠️ Important: Telegram-Only Library

**This library is designed exclusively for Telegram Mini Apps.** All components and utilities are intended to be used within Telegram's environment. The Telegram WebApp SDK is only available when your app is running inside Telegram.

### Added
- Initial release of Telegram UI library
- Button component with multiple styles (filled, outline, plain, gray, white)
- Spinner component with three sizes (s, m, l)
- Support for Leptos and Yew web frameworks
- CSS styles for Telegram UI components
- Example applications for both frameworks
- Comprehensive documentation
- CI/CD pipeline with GitHub Actions
- Security audit integration
- Test suite

### Features
- **Button Component**
  - Multiple size options (s, m, l)
  - Six button modes (filled, bezeled, plain, gray, outline, white)
  - Custom children content
  - Default trait implementation
  - Method chaining for easy configuration

- **Spinner Component**
  - Three size options (s, m, l)
  - Default trait implementation
  - Method chaining for easy configuration

- **CSS Variables**
  - Fully customizable via CSS custom properties
  - Support for Telegram's color scheme

- **Telegram WebApp SDK Integration**
  - Full integration with `telegram-webapp-sdk` v0.4
  - `webapp-sdk` feature flag for WebApp utilities
  - `webapp` module with 15 utility functions for Telegram integration

### Security
- Security audit integration via rustsec/audit-check
- Dependency management with Cargo
- No hardcoded secrets or credentials

### Documentation
- Comprehensive library documentation
- Code examples for both frameworks
- CSS variable reference
- Component API documentation

## [Unreleased] - TBD

### Planned
- Input component
- Card component
- Header component
- Navigation component
- Theme switcher
- Dark mode support
- Accessibility improvements
- Performance optimizations

### Breaking Changes (Planned)
- Future versions will maintain Telegram-only focus
- No support for non-Telegram usage
