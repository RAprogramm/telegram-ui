// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Platform detection for Telegram UI components

/// Platform enumeration for cross-platform UI rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Platform {
    /// iOS platform with Apple design guidelines
    Ios,
    /// Android platform with Material Design guidelines
    Android,
    /// Base (web/desktop) platform
    #[default]
    Base
}

impl Platform {
    /// Create platform from user agent string
    ///
    /// Detects platform by analyzing the user agent string.
    /// iOS is detected first, then Android, falling back to Base.
    #[must_use]
    pub fn from_user_agent(user_agent: &str) -> Self {
        let ua_lower = user_agent.to_lowercase();

        // iOS detection - must check before Android (iPad shows both)
        // Check for iPhone, iPad, or Mac with touch capability or Mobile Safari
        if ua_lower.contains("iphone")
            || ua_lower.contains("ipad")
            || (ua_lower.contains("mac")
                && (ua_lower.contains("touch") || ua_lower.contains("mobile")))
        {
            return Self::Ios;
        }

        // Android detection
        if ua_lower.contains("android") {
            return Self::Android;
        }

        Self::Base
    }

    /// Detect platform from current environment
    ///
    /// Uses WEBKIT_USER_AGENT environment variable when available,
    /// otherwise falls back to Base platform.
    #[cfg(target_arch = "wasm32")]
    pub fn detect() -> Self {
        use wasm_bindgen::{JsCast, prelude::*};

        web_sys::window()
            .and_then(|win| win.navigator().user_agent().ok())
            .map_or(Platform::Base, |ua| Platform::from_user_agent(&ua))
    }

    /// Detect platform from current environment (non-WASM)
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn detect() -> Self {
        std::env::var("USER_AGENT").map_or(Self::Base, |ua| Self::from_user_agent(&ua))
    }

    /// Check if this is iOS platform
    #[inline]
    #[must_use]
    pub const fn is_ios(&self) -> bool {
        matches!(self, Self::Ios)
    }

    /// Check if this is Android platform
    #[inline]
    #[must_use]
    pub const fn is_android(&self) -> bool {
        matches!(self, Self::Android)
    }

    /// Check if this is base platform
    #[inline]
    #[must_use]
    pub const fn is_base(&self) -> bool {
        matches!(self, Self::Base)
    }

    /// Get CSS class name for this platform
    ///
    /// Returns the platform-specific CSS class name for styling.
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Ios => "tgui-platform-ios",
            Self::Android => "tgui-platform-android",
            Self::Base => "tgui-platform-base"
        }
    }

    /// Get the platform name as a string
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Ios => "ios",
            Self::Android => "android",
            Self::Base => "base"
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for Platform {
    type Err = crate::error::UiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ios" => Ok(Self::Ios),
            "android" => Ok(Self::Android),
            "base" | "web" | "desktop" => Ok(Self::Base),
            _ => Err(crate::error::UiError::InvalidPlatform(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_user_agent_ios() {
        let ios_agents = vec![
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X)",
            "Mozilla/5.0 (iPad; CPU OS 16_0 like Mac OS X)",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Mobile Safari",
        ];

        for agent in ios_agents {
            assert_eq!(Platform::from_user_agent(agent), Platform::Ios);
        }
    }

    #[test]
    fn test_from_user_agent_android() {
        let android_agents = vec![
            "Mozilla/5.0 (Linux; Android 13; SM-G991B)",
            "Mozilla/5.0 (Linux; Android 12; Pixel 6)",
        ];

        for agent in android_agents {
            assert_eq!(Platform::from_user_agent(agent), Platform::Android);
        }
    }

    #[test]
    fn test_from_user_agent_base() {
        let base_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
            "",
        ];

        for agent in base_agents {
            assert_eq!(Platform::from_user_agent(agent), Platform::Base);
        }
    }

    #[test]
    fn test_css_class() {
        assert_eq!(Platform::Ios.css_class(), "tgui-platform-ios");
        assert_eq!(Platform::Android.css_class(), "tgui-platform-android");
        assert_eq!(Platform::Base.css_class(), "tgui-platform-base");
    }

    #[test]
    fn test_from_str() {
        use crate::error::UiError;

        assert_eq!("ios".parse::<Platform>(), Ok(Platform::Ios));
        assert_eq!("Android".parse::<Platform>(), Ok(Platform::Android));
        assert_eq!("web".parse::<Platform>(), Ok(Platform::Base));
        let err = "invalid".parse::<Platform>().unwrap_err();
        assert!(matches!(err, UiError::InvalidPlatform(_)));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Platform::Ios), "ios");
        assert_eq!(format!("{}", Platform::Android), "android");
        assert_eq!(format!("{}", Platform::Base), "base");
    }
}
