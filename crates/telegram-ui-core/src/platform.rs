/// Platform detection for Telegram UI components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// iOS platform
    Ios,
    /// Android platform
    Android,
    /// Base (web) platform
    Base,
}

impl Default for Platform {
    fn default() -> Self {
        Self::Base
    }
}

impl Platform {
    /// Detect platform from user agent or return default
    pub fn detect() -> Self {
        // In a real implementation, this would check navigator.userAgent
        // For now, default to Base
        Self::Base
    }

    /// Check if platform is iOS
    pub fn is_ios(&self) -> bool {
        *self == Platform::Ios
    }

    /// Check if platform is Android
    pub fn is_android(&self) -> bool {
        *self == Platform::Android
    }

    /// Check if platform is Base
    pub fn is_base(&self) -> bool {
        *self == Platform::Base
    }
}
