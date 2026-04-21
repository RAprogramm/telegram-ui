// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Platform detection hook
//!
//! Provides a simple hook to access the current platform detection
//! result from the application context.

use crate::platform::Platform;

/// Hook that returns the current platform
///
/// Returns a [`Platform`] enum indicating the current platform:
/// - [`Platform::Ios`] for iOS devices
/// - [`Platform::Android`] for Android devices
/// - [`Platform::Base`] for web/desktop platforms
///
/// This hook is a simple wrapper around the platform detection logic
/// and is primarily used for consistency with the hooks pattern.
///
/// # Examples
///
/// ```ignore
/// use telegram_ui::hooks::use_platform;
/// use telegram_ui::Platform;
///
/// let platform = use_platform();
///
/// match platform {
///     Platform::Ios => { /* iOS-specific logic */ }
///     Platform::Android => { /* Android-specific logic */ }
///     Platform::Base => { /* Base platform logic */ }
/// }
/// ```
#[expect(dead_code)]
pub fn use_platform() -> Platform {
    Platform::detect()
}
