// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Components module

pub mod app;
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
pub use app::AppRoot;
pub use blocks::{
    accordion::Accordion,
    avatar_stack::AvatarStack,
    banner::{Banner, BannerType},
    button::{Button, ButtonMode, ButtonSize},
    card::Card,
    cell::Cell,
    image::Image,
    list::List,
    placeholder::Placeholder,
    section::{Section, SectionHeader},
    steps::{Orientation, StepState, Steps},
};
pub use feedback::{Alert, AlertKind, EmptyState, Skeleton, Spinner, spinner::SpinnerSize};
pub use form::{Checkbox, Input, Radio, Select, Switch, Textarea};
pub use layout::{Column, Container, Row, Spacer};
pub use navigation::{Button as NavButton, Link};
pub use overlays::{Backdrop, Modal, Toast};
pub use service::{Avatar, Badge, Divider, Progress};
pub use typography::{Caption, Headline, Subtitle, Text, Title};
