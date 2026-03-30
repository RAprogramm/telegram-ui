// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Feedback components

pub mod alert;
pub mod empty_state;
pub mod skeleton;
pub mod spinner;

// Re-export components
pub use alert::Alert;
pub use alert::AlertKind;
pub use empty_state::EmptyState;
pub use skeleton::Skeleton;
pub use spinner::Spinner;
