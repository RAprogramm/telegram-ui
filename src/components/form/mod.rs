// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Form components

pub mod chip;
pub mod checkbox;
pub mod color_input;
pub mod file_input;
pub mod form_input;
pub mod input;
pub mod multiselect;
pub mod multiselectable;
pub mod pin_input;
pub mod rating;
pub mod radio;
pub mod selectable;
pub mod select;
pub mod slider;
pub mod switch;
pub mod textarea;

// Re-export components
pub use chip::Chip;
pub use checkbox::Checkbox;
pub use color_input::ColorInput;
pub use file_input::FileInput;
pub use form_input::FormInput;
pub use input::Input;
pub use multiselect::Multiselect;
pub use multiselectable::Multiselectable;
pub use pin_input::PinInput;
pub use rating::Rating;
pub use radio::Radio;
pub use selectable::Selectable;
pub use select::Select;
pub use slider::Slider;
pub use switch::Switch;
pub use textarea::Textarea;
