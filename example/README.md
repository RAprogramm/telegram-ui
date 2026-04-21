# Telegram UI Example in Rust

A clone of the TGUI Example reimagined in Rust with Trunk for WASM.

## Features

- **Dynamic CSS Injection**: Telegram UI styles are injected at runtime via `get_styles()`
- **Raw Rust**: No web frameworks - direct DOM manipulation with web-sys
- **WASM Optimized**: Tree-shaken and optimized for production

## Prerequisites

1. Rust and Cargo (install via [rustup](https://rustup.rs))
2. WASM target: `rustup target add wasm32-unknown-unknown`
3. Trunk: `cargo binstall trunk`

## Building and Running

### Development Mode

```bash
trunk serve --open
```

This will start a development server at `http://localhost:8080` with hot reloading.

### Production Build

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## What's Included

This example demonstrates all major Telegram UI components:
- **Cell Section** - List items with before/after content
- **Form Section** - Text input fields with headers
- **Banner Section** - Promotional banners with images and CTAs
- **Timeline Section** - Step-by-step progress visualization
- **Tooltip Section** - Interactive tooltip on button click
- **Modal Section** - Placeholder with modal action

## Technology Stack

- **Language**: Rust 2021 Edition
- **WASM Toolchain**: Trunk
- **DOM Manipulation**: web-sys + wasm-bindgen
- **No Frameworks**: Raw Rust for maximum performance and minimal bundle size
