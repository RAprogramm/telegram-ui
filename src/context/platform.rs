// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
/// Platform context for components
use crate::platform::Platform;

#[derive(Debug, Clone)]
pub struct PlatformContext {
    platform: Platform
}

impl PlatformContext {
    /// Create a new platform context
    #[must_use]
    pub const fn new(platform: Platform) -> Self {
        Self {
            platform
        }
    }

    /// Get the current platform
    #[must_use]
    pub const fn platform(&self) -> Platform {
        self.platform
    }

    /// Check if platform is iOS
    #[must_use]
    pub const fn is_ios(&self) -> bool {
        self.platform.is_ios()
    }

    /// Check if platform is Android
    #[must_use]
    pub const fn is_android(&self) -> bool {
        self.platform.is_android()
    }

    /// Check if platform is Base
    #[must_use]
    pub const fn is_base(&self) -> bool {
        self.platform.is_base()
    }
}

impl Default for PlatformContext {
    fn default() -> Self {
        Self::new(Platform::Base)
    }
}
