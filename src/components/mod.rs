// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Components module

pub mod blocks;
pub mod feedback;
pub mod form;
pub mod layout;
pub mod misc;
pub mod navigation;
pub mod overlays;
pub mod service;
pub mod typography;

// Re-export all components
pub use blocks::{button::Button, card::Card, cell::Cell, list::List, placeholder::Placeholder};
pub use feedback::{Alert, AlertKind, EmptyState, Skeleton, Spinner};
pub use form::{Input, Select, Textarea};
pub use layout::{Column, Container, Row, Spacer};
pub use navigation::{Button as NavButton, Link};
pub use overlays::{Backdrop, Modal, Toast};
pub use service::{Badge, Divider, Progress};
pub use typography::{Caption, Headline, Subtitle, Text, Title};
