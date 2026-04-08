// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! # Telegram UI - Rust Library for Telegram Mini Apps
//!
//! Telegram UI is a Rust library for creating Telegram-style interfaces
//! **exclusively for Telegram Mini Apps**. It provides a comprehensive set of
//! ready-to-use components that follow Telegram's design language, with full
//! integration into the Telegram WebApp SDK.
//!
//! ## ⚠️ Important: Telegram-Only Library
//!
//! This library is designed **exclusively** for Telegram Mini Apps and cannot
//! be used outside of Telegram. The Telegram WebApp SDK is only available when
//! your app is running inside Telegram.
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
//! - **CSS Variables** - Customizable via CSS custom properties
//!
//! # Examples
//!
//! ```ignore
//! use telegram_ui::{Button, Spinner};
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
//! - `--telegram-button-filled-bg`: Background color for filled buttons
//!   (#0088cc)
//! - `--telegram-button-filled-color`: Text color for filled buttons (white)
//! - `--telegram-button-bezeled-bg`: Background color for bezeled buttons
//!   (transparent)
//! - `--telegram-button-bezeled-color`: Text color for bezeled buttons
//!   (#0088cc)
//! - `--telegram-button-plain-color`: Text color for plain buttons (#0088cc)
//! - `--telegram-button-gray-bg`: Background color for gray buttons (#f0f0f0)
//! - `--telegram-button-gray-color`: Text color for gray buttons (#333)

mod error;
mod platform;
mod theme;

pub mod components;
pub mod context;
pub mod helpers;
pub use error::{Result, UiError, ValidationError};
pub use platform::Platform;
pub use theme::{Theme, ThemeContext};

/// Get the CSS styles for Telegram UI
pub fn get_styles() -> &'static str {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/styles.css"))
}

// Re-export all components
pub use components::{
    Alert, AlertKind, AppRoot, Backdrop, Button, Caption, Card, Cell, Column, Container, Divider,
    EmptyState, Headline, Input, List, Modal, Placeholder, Progress, Row, Select, Skeleton,
    Spacer, Spinner, Subtitle, Text, Textarea, Title, Toast
};
