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
pub use blocks::button::Button;
pub use blocks::card::Card;
pub use blocks::cell::Cell;
pub use blocks::list::List;
pub use blocks::placeholder::Placeholder;

pub use feedback::Alert;
pub use feedback::AlertKind;
pub use feedback::EmptyState;
pub use feedback::Skeleton;
pub use feedback::Spinner;

pub use form::Input;
pub use form::Select;
pub use form::Textarea;

pub use layout::Column;
pub use layout::Container;
pub use layout::Row;
pub use layout::Spacer;

pub use navigation::Button as NavButton;
pub use navigation::Link;

pub use overlays::Backdrop;
pub use overlays::Modal;
pub use overlays::Toast;

pub use service::Badge;
pub use service::Divider;
pub use service::Progress;

pub use typography::Caption;
pub use typography::Headline;
pub use typography::Subtitle;
pub use typography::Text;
pub use typography::Title;
