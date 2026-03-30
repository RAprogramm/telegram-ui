//! # Telegram UI - Rust Library for Telegram Mini Apps
//!
//! Telegram UI is a Rust library for creating Telegram-style interfaces **exclusively for Telegram Mini Apps**.
//! It provides a comprehensive set of ready-to-use components that follow Telegram's design language, with full integration into the Telegram WebApp SDK.
//!
//! ## ⚠️ Important: Telegram-Only Library
//!
//! This library is designed **exclusively** for Telegram Mini Apps and cannot be used outside of Telegram.
//! The Telegram WebApp SDK is only available when your app is running inside Telegram.
//!
//! # Features
//!
//! - **Button** - Multiple button styles (filled, outline, plain, gray, white)
//! - **Spinner** - Loading indicators in multiple sizes
//! - **Card** - Container with optional ambient style
//! - **Cell** - List item with before/after content
//! - **List** - Container for cells
//! - **Input** - Text input field
//! - **Textarea** - Multi-line text input
//! - **Select** - Dropdown select
//! - **Text** - Customizable text element
//! - **Title/Subtitle/Headline/Caption** - Typography elements
//! - **Alert** - Info/success/warning/error messages
//! - **Modal** - Overlay modal dialog
//! - **Toast** - Floating notification
//! - **Divider** - Horizontal separator
//! - **Badge** - Notification badge
//! - **Progress** - Progress indicator
//! - **Skeleton** - Loading placeholder
//! - **EmptyState** - Empty state with action
//! - **Container/Row/Column** - Layout components
//! - **Framework Support** - Works with Leptos and Yew web frameworks
//! - **Telegram WebApp Integration** - Full SDK integration with `webapp-sdk` feature
//! - **CSS Variables** - Customizable via CSS custom properties
//!
//! # Examples
//!
//! ```ignore
//! use telegram_ui_core::{Button, Spinner};
//!
//! let button = Button::new()
//!     .size("m")
//!     .mode("filled")
//!     .children("Click me");
//!
//! let spinner = Spinner::new().size("m");
//! ```
//!
//! # CSS Variables
//!
//! - `--telegram-button-filled-bg`: Background color for filled buttons (#0088cc)
//! - `--telegram-button-filled-color`: Text color for filled buttons (white)
//! - `--telegram-button-bezeled-bg`: Background color for bezeled buttons (transparent)
//! - `--telegram-button-bezeled-color`: Text color for bezeled buttons (#0088cc)
//! - `--telegram-button-plain-color`: Text color for plain buttons (#0088cc)
//! - `--telegram-button-gray-bg`: Background color for gray buttons (#f0f0f0)
//! - `--telegram-button-gray-color`: Text color for gray buttons (#333)
//!
//! # Telegram WebApp Integration
//!
//! When compiled with the `webapp-sdk` feature, this library provides utilities for
//! integrating with Telegram WebApp SDK:
//!
//! ```ignore
//! use telegram_ui_core::webapp::{init_webapp, show_alert, expand};
//!
//! fn main() {
//!     if let Ok(true) = init_webapp() {
//!         // Running inside Telegram
//!         show_alert("Welcome to Telegram WebApp!").ok();
//!         expand().ok();
//!     }
//! }
//! ```
//!
//! ## Prerequisites
//!
//! - Telegram Bot (created via @BotFather)
//! - Mini App configured in your bot
//! - HTTPS hosting for your WebAssembly build

mod platform;
mod webapp;

pub mod components;
pub mod context;
pub mod helpers;

pub use platform::Platform;

/// Get the CSS styles for Telegram UI
pub fn get_styles() -> &'static str {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/styles.css"))
}

// Re-export all components
pub use components::Button;
pub use components::Spinner;
pub use components::Card;
pub use components::Cell;
pub use components::List;
pub use components::Placeholder;

pub use components::Alert;
pub use components::AlertKind;
pub use components::Badge;
pub use components::EmptyState;
pub use components::Progress;
pub use components::Skeleton;
pub use components::Input;
pub use components::Select;
pub use components::Textarea;
pub use components::Container;
pub use components::Row;
pub use components::Column;
pub use components::Spacer;
pub use components::Backdrop;
pub use components::Modal;
pub use components::Toast;
pub use components::Divider;
pub use components::Text;
pub use components::Title;
pub use components::Subtitle;
pub use components::Headline;
pub use components::Caption;
