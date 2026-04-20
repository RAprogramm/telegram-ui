// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Form components

pub mod checkbox;
pub mod chip;
pub mod color_input;
pub mod file_input;
pub mod form_input;
pub mod input;
pub mod multiselect;
pub mod multiselectable;
pub mod pin_input;
pub mod radio;
pub mod rating;
pub mod select;
pub mod selectable;
pub mod slider;
pub mod switch;
pub mod textarea;

// Re-export components
pub use checkbox::Checkbox;
pub use chip::Chip;
pub use color_input::ColorInput;
pub use file_input::FileInput;
pub use form_input::FormInput;
pub use input::Input;
pub use multiselect::Multiselect;
pub use multiselectable::Multiselectable;
pub use pin_input::PinInput;
pub use radio::Radio;
pub use rating::Rating;
pub use select::Select;
pub use selectable::Selectable;
pub use slider::Slider;
pub use switch::Switch;
pub use textarea::Textarea;
