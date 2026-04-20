// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Block components

pub mod accordion;
pub mod avatar_stack;
pub mod banner;
pub mod blockquote;
pub mod button;
pub mod card;
pub mod cell;
pub mod icon_button;
pub mod icon_container;
pub mod image;
pub mod inline_buttons;
pub mod list;
pub mod placeholder;
pub mod section;
pub mod segmented_control;
pub mod steps;
pub mod tab_bar;
pub mod timeline;

// Re-export components
pub use accordion::Accordion;
pub use avatar_stack::AvatarStack;
pub use banner::{Banner, BannerType};
pub use blockquote::Blockquote;
pub use button::{Button, ButtonMode, ButtonSize};
pub use card::Card;
pub use cell::Cell;
pub use icon_button::IconButton;
pub use icon_container::IconContainer;
pub use image::Image;
pub use inline_buttons::InlineButtons;
pub use list::List;
pub use placeholder::Placeholder;
pub use section::{Section, SectionHeader};
pub use segmented_control::{Segment, SegmentedControl, SegmentedControlSize};
pub use steps::{Orientation, StepState, Steps};
pub use tab_bar::{Tab, TabBar};
pub use timeline::Timeline;
