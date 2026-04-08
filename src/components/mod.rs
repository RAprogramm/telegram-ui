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
    blockquote::Blockquote,
    button::{Button, ButtonMode, ButtonSize},
    card::Card,
    cell::Cell,
    icon_button::IconButton,
    icon_container::IconContainer,
    image::Image,
    inline_buttons::InlineButtons,
    list::List,
    placeholder::Placeholder,
    section::{Section, SectionHeader},
    steps::{Orientation, StepState, Steps},
    timeline::Timeline,
};
pub use feedback::{
    Alert, AlertKind, CircularProgress, EmptyState, Skeleton, Spinner, SpinnerSize, Spoiler,
};
pub use form::{
    Chip, Checkbox, ColorInput, FileInput, FormInput, Input, Multiselect, Multiselectable,
    PinInput, Rating, Radio, Selectable, Select, Slider, Switch, Textarea,
};
pub use layout::{Column, Container, Row, Spacer};
pub use navigation::{Button as NavButton, Link};
pub use overlays::{Backdrop, Modal, Snackbar, Toast};
pub use service::{
    Avatar, Badge, Divider, HorizontalScroll, Progress, RootRenderer, Tappable, Touch,
    VisuallyHidden,
};
pub use typography::{Caption, Headline, Subtitle, Text, Title};
