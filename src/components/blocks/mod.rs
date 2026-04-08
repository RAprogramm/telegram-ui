// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Block components

pub mod accordion;
pub mod avatar_stack;
pub mod banner;
pub mod button;
pub mod card;
pub mod cell;
pub mod image;
pub mod list;
pub mod placeholder;
pub mod section;
pub mod steps;

// Re-export components
pub use accordion::Accordion;
pub use avatar_stack::AvatarStack;
pub use banner::{Banner, BannerType};
pub use button::Button;
pub use card::Card;
pub use cell::Cell;
pub use image::Image;
pub use list::List;
pub use placeholder::Placeholder;
pub use section::{Section, SectionHeader};
pub use steps::{Orientation, StepState, Steps};
