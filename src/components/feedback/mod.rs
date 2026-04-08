// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Feedback components

pub mod alert;
pub mod circular_progress;
pub mod empty_state;
pub mod skeleton;
pub mod spinner;
pub mod spoiler;

// Re-export components
pub use alert::{Alert, AlertKind};
pub use circular_progress::CircularProgress;
pub use empty_state::EmptyState;
pub use skeleton::Skeleton;
pub use spinner::Spinner;
pub use spinner::SpinnerSize;
pub use spoiler::Spoiler;
