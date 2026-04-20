// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Service components

pub mod avatar;
pub mod badge;
pub mod divider;
pub mod horizontal_scroll;
pub mod progress;
pub mod root_renderer;
pub mod tappable;
pub mod touch;
pub mod visually_hidden;

// Re-export components
pub use avatar::Avatar;
pub use badge::{Badge, BadgeMode, BadgeType};
pub use divider::Divider;
pub use horizontal_scroll::HorizontalScroll;
pub use progress::Progress;
pub use root_renderer::RootRenderer;
pub use tappable::Tappable;
pub use touch::Touch;
pub use visually_hidden::VisuallyHidden;
