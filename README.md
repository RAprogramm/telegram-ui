# Telegram UI

[![CI/CD](https://github.com/telegram-ui/telegram-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/telegram-ui/telegram-ui/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-stable-blue.svg)](https://webassembly.org)

Telegram UI is a Rust library for creating Telegram-style interfaces **exclusively for Telegram Mini Apps**. It provides a set of ready-to-use components that follow Telegram's design language, with full integration into the Telegram WebApp SDK.

## ⚠️ Important: Telegram-Only Library

This library is designed **exclusively** for Telegram Mini Apps and cannot be used outside of Telegram. The Telegram WebApp SDK is only available when your app is running inside Telegram.

## ✨ Features

- **Button Component** - Multiple styles (filled, outline, plain, gray, white) and sizes (s, m, l)
- **Spinner Component** - Loading indicators with three size options
- **CSS Variables** - Fully customizable via CSS custom properties
- **Framework Support** - Works seamlessly with Leptos 0.8 and Yew 0.23
- **Type Safety** - Rust's type system ensures compile-time safety
- **Memory Safety** - Zero-cost abstractions with guaranteed memory safety
- **Comprehensive Tests** - Unit tests for all components
- **CI/CD Pipeline** - Automated testing and deployment

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
telegram-ui-core = "0.1.0"
```

## 🚀 Prerequisites

- **Telegram Bot** - You need a Telegram bot to host your Mini App
- **Telegram Mini App** - Your app must be added to a Telegram bot as a Mini App
- **Web Hosting** - Host your WebAssembly build on HTTPS (GitHub Pages, Vercel, etc.)

## 🚀 Quick Start

### Prerequisites

Before starting, ensure you have:
1. A Telegram bot created via [@BotFather](https://t.me/BotFather)
2. Your Mini App configured in the bot
3. Your app hosted on HTTPS

### Leptos Example

```rust
use leptos::prelude::*;
use telegram_ui_core::{Button, Spinner};

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"My Telegram App"</h1>
            
            <Button
                size="m"
                mode="filled"
                children="Click me!"
            />
            
            <Spinner size="m" />
        </div>
    }
}
```

### Yew Example

```rust
use yew::prelude::*;
use telegram_ui_core::{Button, Spinner};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <h1>"My Telegram App"</h1>
            
            <Button
                size="l"
                mode="outline"
                children="Outline Button"
            />
            
            <Spinner size="s" />
        </div>
    }
}
```

## 🎨 Components

### Button

The Button component supports multiple styles:

```rust
// Filled button (default)
Button::new().mode("filled").children("Submit")

// Outline button
Button::new().mode("outline").children("Cancel")

// Plain button
Button::new().mode("plain").children("Delete")

// Gray button
Button::new().mode("gray").children("Secondary")

// White button
Button::new().mode("white").children("Action")
```

### Spinner

The Spinner component provides loading indicators:

```rust
// Small spinner
Spinner::new().size("s")

// Medium spinner (default)
Spinner::new().size("m")

// Large spinner
Spinner::new().size("l")
```

## 🎨 CSS Variables

Customize the appearance using CSS custom properties:

```css
:root {
    --telegram-button-filled-bg: #0088cc;
    --telegram-button-filled-color: white;
    --telegram-button-bezeled-bg: transparent;
    --telegram-button-bezeled-color: #0088cc;
    --telegram-button-bezeled-border: #0088cc;
    --telegram-button-plain-color: #0088cc;
    --telegram-button-gray-bg: #f0f0f0;
    --telegram-button-gray-color: #333;
    --telegram-button-outline-color: #0088cc;
    --telegram-button-outline-border: #0088cc;
    --telegram-button-white-bg: white;
    --telegram-button-white-color: #333;
    --telegram-button-white-border: #e0e0e0;
}
```

## 🧪 Testing

Run the test suite:

```bash
cargo test --all-targets
```

## 📝 Documentation

Generate documentation:

```bash
cargo doc --no-deps --document-private-items
```

## 🌐 Deployment

### Telegram Mini App Requirements

1. **HTTPS Host** - Your app must be hosted on HTTPS (no HTTP)
2. **Manifest File** - Create `manifest.json` for your Mini App
3. **App URL** - Set the URL to your hosted app in BotFather

### Example manifest.json

```json
{
    "url": "https://your-username.github.io/telegram-ui-app/",
    "bot_name": "YourBotName",
    "name": "My Telegram App",
    "description": "A Telegram Mini App built with Rust",
    "primary_color": "#0088cc",
    "icons": [
        {
            "size": "128x128",
            "url": "https://your-username.github.io/telegram-ui-app/icon.png"
        }
    ]
}
```

### Deployment Steps

1. Build with `trunk build --release`
2. Upload `dist/` to your hosting (GitHub Pages, Vercel, etc.)
3. Configure Mini App URL in BotFather
4. Test your app in Telegram

## 🔧 Development

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)
- Trunk for building WebAssembly

### Building Examples

```bash
# Build Yew example
cd examples/yew-example
trunk build

# Build Leptos example
cd examples/leptos-example
trunk build
```

### Running Linter

```bash
cargo clippy --all-targets
```

### Formatting Code

```bash
cargo fmt
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linter
5. Submit a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## 🙏 Acknowledgments

- Telegram for the beautiful design language
- The Rust web framework communities for inspiration

## 📞 Support

- 📧 Email: support@telegram-ui.dev
- 🐛 Report issues: [GitHub Issues](https://github.com/telegram-ui/telegram-ui/issues)
- 💬 Discussion: [GitHub Discussions](https://github.com/telegram-ui/telegram-ui/discussions)
- 📚 Telegram WebApp Docs: [https://docs.telegram.org/apps/intro](https://docs.telegram.org/apps/intro)

## 📚 Additional Resources

- [Telegram Mini Apps Documentation](https://docs.telegram.org/apps/intro)
- [Telegram WebApp SDK](https://docs.rs/telegram-webapp-sdk)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

---

Made with ❤️ using Rust, WebAssembly, and Telegram WebApp SDK
