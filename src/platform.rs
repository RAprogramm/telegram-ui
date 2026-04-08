// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Platform detection for Telegram UI components

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Platform {
    /// iOS platform
    Ios,
    /// Android platform
    Android,
    /// Base (web) platform
    #[default]
    Base
}

impl Platform {
    pub fn is_ios(&self) -> bool {
        matches!(self, Platform::Ios)
    }

    pub fn is_android(&self) -> bool {
        matches!(self, Platform::Android)
    }

    pub fn is_base(&self) -> bool {
        matches!(self, Platform::Base)
    }
}
