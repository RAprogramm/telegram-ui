// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Block components

pub mod accordion;
pub mod accordion_content;
pub mod accordion_summary;
pub mod avatar;
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
pub mod inline_buttons_item;
pub mod list;
pub mod placeholder;
pub mod section;
pub mod segmented_control;
pub mod steps;
pub mod tab_bar;
pub mod timeline;
pub mod timeline_item;

// Re-export components
pub use accordion::Accordion;
pub use accordion_summary::AccordionSummary;
pub use avatar::{AvatarAcronym, Badge, BadgeMode, BadgeType};
pub use avatar_stack::AvatarStack;
pub use banner::{Banner, BannerType};
pub use blockquote::Blockquote;
pub use button::{Button, ButtonMode, ButtonSize};
pub use card::{Card, CardCell, CardChip};
pub use cell::{ButtonCell, Cell, Info, Navigation};
pub use icon_button::IconButton;
pub use icon_container::IconContainer;
pub use image::Image;
pub use inline_buttons::InlineButtons;
pub use inline_buttons_item::InlineButtonsItem;
pub use list::List;
pub use placeholder::Placeholder;
pub use section::{Footer, Header, HeaderVariant, Section};
pub use segmented_control::{Segment, SegmentedControl, SegmentedControlSize};
pub use steps::{Orientation, StepState, Steps};
pub use tab_bar::{Tab, TabBar};
pub use timeline::Timeline;
pub use timeline_item::TimelineItem;
