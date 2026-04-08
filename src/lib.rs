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
//! - **Avatar** - User avatar with image or initials
//! - **Input** - Text input field with validation
//! - **Textarea** - Multi-line text input with validation
//! - **Select** - Dropdown select with validation
//! - **Checkbox** - Checkbox with label
//! - **Radio** - Radio button with label
//! - **Switch** - Toggle switch
//! - **Text** - Customizable text element
//! - **Title/Subtitle/Headline/Caption** - Typography elements
//! - **Alert** - Info/success/warning/error messages
//! - **Modal** - Overlay modal dialog
//! - **Toast** - Floating notification
//! - **Backdrop** - Overlay backdrop
//! - **Divider** - Horizontal separator
//! - **Badge** - Notification badge
//! - **Progress** - Progress indicator
//! - **Skeleton** - Loading placeholder
//! - **EmptyState** - Empty state with action
//! - **Container/Row/Column** - Layout components
//! - **Framework Support** - Works with Leptos and Yew web frameworks
//! - **CSS Variables** - Customizable via CSS custom properties
//! - **Responsive Design** - Mobile-first approach with breakpoints
//! - **Platform Detection** - Automatic iOS/Android/Base detection
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
//!
//! # AppRoot Component
//!
//! The `AppRoot` component is the root wrapper for your Telegram Mini App.
//! It provides platform detection, theme support, and automatic styling.
//!
//! ```ignore
//! use telegram_ui::{AppRoot, Theme, ThemeContext};
//!
//! let app_root = AppRoot::new()
//!     .theme(Theme::Auto)
//!     .theme_context(
//!         ThemeContext::default()
//!             .with_bg_color("#ffffff")
//!             .with_text_color("#000000")
//!     );
//! ```
//!
//! # Platform Detection
//!
//! The library automatically detects the platform (iOS/Android/Base) and
//! applies appropriate styling. You can also manually set the platform:
//!
//! ```ignore
//! use telegram_ui::{AppRoot, Platform};
//!
//! let app_root = AppRoot::new().platform(Platform::Ios);
//! ```
//!
//! # Theme Support
//!
//! Supports Light, Dark, and Auto themes with full Telegram color scheme:
//!
//! ```ignore
//! use telegram_ui::{Theme, ThemeContext};
//!
//! let theme_context = ThemeContext::default()
//!     .with_bg_color("#ffffff")
//!     .with_text_color("#000000")
//!     .with_hint_color("#999999")
//!     .with_link_color("#2481cc")
//!     .with_button_color("#2481cc")
//!     .with_button_text_color("#ffffff")
//!     .with_secondary_bg_color("#f4f4f5")
//!     .with_header_bg_color("#ffffff")
//!     .with_bottom_bar_bg_color("#ffffff")
//!     .with_accent_text_color("#2481cc")
//!     .with_section_bg_color("#f4f4f5")
//!     .with_section_header_text_color("#707579")
//!     .with_section_separator_color("#c8c7cc")
//!     .with_subtitle_text_color("#707579")
//!     .with_destructive_text_color("#e53935");
//! ```
//!
//! # Responsive Design
//!
//! The library uses a mobile-first approach with the following breakpoints:
//! - **Mobile**: Default (optimized for phones)
//! - **768px**: Tablets
//! - **1024px**: Desktop
//! - **1280px**: Large desktop
//!
//! It also supports:
//! - Touch-friendly adjustments for coarse pointers
//! - Landscape orientation on mobile
//! - High contrast mode
//! - Reduced motion preference
//! - Auto dark/light theme via `prefers-color-scheme`

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
    Alert, AlertKind, AppRoot, Avatar, Backdrop, Button, Caption, Card, Cell, Checkbox, Column,
    Container, Divider, EmptyState, Headline, Input, List, Modal, Placeholder, Progress, Radio,
    Row, Select, Skeleton, Spacer, Spinner, Subtitle, Switch, Text, Textarea, Title, Toast
};
