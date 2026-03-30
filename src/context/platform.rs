// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
/// Platform context for components
use crate::platform::Platform;

#[derive(Debug, Clone)]
pub struct PlatformContext {
    platform: Platform,
}

impl PlatformContext {
    /// Create a new platform context
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }

    /// Get the current platform
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Check if platform is iOS
    pub fn is_ios(&self) -> bool {
        self.platform.is_ios()
    }

    /// Check if platform is Android
    pub fn is_android(&self) -> bool {
        self.platform.is_android()
    }

    /// Check if platform is Base
    pub fn is_base(&self) -> bool {
        self.platform.is_base()
    }
}

impl Default for PlatformContext {
    fn default() -> Self {
        Self::new(Platform::Base)
    }
}
