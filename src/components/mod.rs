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
    segmented_control::{Segment, SegmentedControl, SegmentedControlSize},
    steps::{Orientation, StepState, Steps},
    tab_bar::{Tab, TabBar},
    timeline::Timeline
};
pub use feedback::{
    Alert, AlertKind, CircularProgress, EmptyState, Skeleton, Spinner, SpinnerSize, Spoiler
};
pub use form::{
    Checkbox, Chip, ColorInput, FileInput, FormInput, Input, Multiselect, Multiselectable,
    PinInput, Radio, Rating, Select, Selectable, Slider, Switch, Textarea
};
pub use layout::{Column, Container, Row, Spacer};
pub use navigation::{Button as NavButton, Link};
pub use overlays::{Backdrop, Modal, Snackbar, Toast};
pub use service::{
    Avatar, Badge, BadgeMode, BadgeType, Divider, HorizontalScroll, Progress, RootRenderer,
    Tappable, Touch, VisuallyHidden
};
pub use typography::{Caption, Headline, Subtitle, Text, Title};
