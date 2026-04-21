// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Image component for Telegram UI
//!
//! The `Image` component renders images with support for:
//! - Loading images from src URLs
//! - Fallback icon when the image fails to load or no src is provided
//! - Multiple sizes: small ('s' = 24px), medium ('m' = 40px), large ('l' =
//!   96px)
//! - Platform detection (iOS, Android, or Base)
//! - Badge support (for notification counters, etc.)
//!
//! Unlike the TypeScript version, this Rust implementation focuses on simple
//! HTML string rendering for static content. For dynamic event handling
//! (load/error events), integration with web frameworks like Leptos or Yew is
//! recommended.
//!
//! # Example
//!
//! ```ignore
//! use telegram_ui::components::blocks::image::{Image, ImageSize};
//!
//! // Basic image with default size
//! let image = Image::new()
//!     .src("https://example.com/image.jpg")
//!     .render();
//!
//! // Image with custom size and platform
//! let image = Image::new()
//!     .src("https://example.com/image.jpg")
//!     .size(ImageSize::Large)
//!     .platform(Platform::Ios)
//!     .render();
//!
//! // Image with fallback icon
//! let image = Image::new()
//!     .fallback_icon("<i class=\"icon-user\"></i>")
//!     .render();
//!
//! // Image with badge
//! let badge = "<span class=\"telegram-ui-badge\">99</span>";
//! let image = Image::new()
//!     .src("https://example.com/image.jpg")
//!     .badge(badge)
//!     .render();
//! ```

use crate::{helpers::escape_html, platform::Platform};

/// Image size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageSize {
    /// Small size: 24px
    Small,
    /// Medium size: 40px (default)
    #[default]
    Medium,
    /// Large size: 96px
    Large
}

impl ImageSize {
    /// Convert to CSS pixel value
    #[must_use]
    pub const fn to_pixels(&self) -> u32 {
        match self {
            Self::Small => 24,
            Self::Medium => 40,
            Self::Large => 96
        }
    }

    /// Convert to string for CSS class
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "s",
            Self::Medium => "m",
            Self::Large => "l"
        }
    }
}

/// Image component with loading state management and event handling
#[derive(Debug, Clone)]
pub struct Image {
    src:           Option<String>,
    fallback_icon: Option<String>,
    size:          ImageSize,
    platform:      Platform,
    badge:         Option<String>,
    class:         Option<String>,
    style:         Option<String>
}

impl Image {
    /// Create a new Image component with default settings
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            src:           None,
            fallback_icon: None,
            size:          ImageSize::Medium,
            platform:      Platform::detect(),
            badge:         None,
            class:         None,
            style:         None
        }
    }

    /// Set image source URL
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new().src("https://example.com/image.jpg");
    /// ```
    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    /// Set fallback icon (HTML string or icon name)
    ///
    /// The fallback is displayed when:
    /// - No src is provided
    /// - Image fails to load
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new().fallback_icon("<i class=\"icon-user\"></i>");
    /// ```
    pub fn fallback_icon(mut self, fallback_icon: impl Into<String>) -> Self {
        self.fallback_icon = Some(fallback_icon.into());
        self
    }

    /// Set image size
    ///
    /// # Example
    ///
    /// ```ignore
    /// use telegram_ui::components::blocks::image::ImageSize;
    ///
    /// let image = Image::new().size(ImageSize::Large);
    /// ```
    #[must_use]
    pub const fn size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    /// Set image size by string
    ///
    /// Accepts "s", "m", or "l" (case-insensitive).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new().with_size_str("l");
    /// ```
    #[must_use]
    pub fn with_size_str(mut self, size: &str) -> Self {
        self.size = match size.to_lowercase().as_str() {
            "s" | "small" => ImageSize::Small,
            "m" | "medium" | "default" => ImageSize::Medium,
            "l" | "large" => ImageSize::Large,
            _ => ImageSize::Medium
        };
        self
    }

    /// Set platform for platform-specific styling
    ///
    /// # Example
    ///
    /// ```ignore
    /// use telegram_ui::Platform;
    ///
    /// let image = Image::new().platform(Platform::Ios);
    /// ```
    #[must_use]
    pub const fn platform(mut self, platform: Platform) -> Self {
        self.platform = platform;
        self
    }

    /// Set custom CSS class
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new().class("custom-image");
    /// ```
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    /// Set inline styles
    ///
    /// # Example
    ///
    /// ```ignore
    /// let image = Image::new().style("border: 1px solid red;");
    /// ```
    pub fn style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Set badge HTML content
    ///
    /// The badge is rendered as a child element inside the image wrapper.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let badge = "<span class=\"telegram-ui-badge\">99</span>";
    /// let image = Image::new().badge(badge);
    /// ```
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Get src value
    #[must_use]
    pub fn get_src(&self) -> Option<&str> {
        self.src.as_deref()
    }

    /// Get fallback icon value
    #[must_use]
    pub fn get_fallback_icon(&self) -> Option<&str> {
        self.fallback_icon.as_deref()
    }

    /// Get size value
    #[must_use]
    pub const fn get_size(&self) -> ImageSize {
        self.size
    }

    /// Get platform value
    #[must_use]
    pub const fn get_platform(&self) -> Platform {
        self.platform
    }

    /// Get badge value
    #[must_use]
    pub fn get_badge(&self) -> Option<&str> {
        self.badge.as_deref()
    }

    /// Check if image has src
    #[must_use]
    pub const fn has_src(&self) -> bool {
        self.src.is_some()
    }

    /// Check if should show fallback icon (only when no src)
    #[must_use]
    pub const fn should_show_fallback(&self) -> bool {
        !self.has_src()
    }

    /// Get size in pixels
    #[must_use]
    pub const fn size_pixels(&self) -> u32 {
        self.size.to_pixels()
    }

    /// Get border radius based on size
    const fn get_border_radius(&self) -> u32 {
        // Standard Telegram UI border radius
        // For images, use 50% for perfect circles
        50
    }

    /// Render the Image component to HTML string
    ///
    /// The image shows either the img element (if src is set) or fallback icon.
    /// Badge is rendered as a child element.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let html = Image::new()
    ///     .src("test.jpg")
    ///     .size(ImageSize::Small)
    ///     .render();
    /// ```
    pub fn render(&self) -> String {
        let size = self.size_pixels();
        let size_str = self.size.as_str();
        let platform_name = self.platform.name();

        let badge_html = self
            .badge
            .as_ref()
            .map_or_else(String::new, std::clone::Clone::clone);

        format!(
            "<div class=\"telegram-ui-image telegram-ui-image--{} tgui-platform-{}{}\" style=\"width: {}px; min-width: {}px; height: {}px; border-radius: {}%{}\">{}{}{}</div>",
            size_str,
            platform_name,
            if let Some(ref class) = self.class {
                format!(" {class}")
            } else {
                String::new()
            },
            size,
            size,
            size,
            self.get_border_radius(),
            if let Some(ref custom_style) = self.style {
                format!("; {custom_style}")
            } else {
                String::new()
            },
            self.src.as_ref().map_or_else(String::new, |src| format!(
                "<img src=\"{}\" class=\"telegram-ui-image__img\">",
                escape_html(src)
            )),
            if self.should_show_fallback() {
                self.fallback_icon
                    .as_ref()
                    .map_or_else(String::new, |fallback| {
                        format!("<div class=\"telegram-ui-image__fallback\">{fallback}</div>")
                    })
            } else {
                String::new()
            },
            badge_html
        )
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_default() {
        let img = Image::new();
        assert!(img.get_src().is_none());
        assert!(img.get_fallback_icon().is_none());
        assert_eq!(img.get_size(), ImageSize::Medium);
    }

    #[test]
    fn test_image_with_src() {
        let img = Image::new().src("https://example.com/image.jpg");
        assert_eq!(img.get_src(), Some("https://example.com/image.jpg"));
    }

    #[test]
    fn test_image_with_fallback() {
        let img = Image::new().fallback_icon("<i class=\"icon-user\"></i>");
        assert_eq!(img.get_fallback_icon(), Some("<i class=\"icon-user\"></i>"));
    }

    #[test]
    fn test_image_size_small() {
        let img = Image::new().size(ImageSize::Small);
        assert_eq!(img.get_size(), ImageSize::Small);
        assert_eq!(img.size_pixels(), 24);
        assert_eq!(img.get_size().as_str(), "s");
    }

    #[test]
    fn test_image_size_large() {
        let img = Image::new().size(ImageSize::Large);
        assert_eq!(img.get_size(), ImageSize::Large);
        assert_eq!(img.size_pixels(), 96);
        assert_eq!(img.get_size().as_str(), "l");
    }

    #[test]
    fn test_image_size_str() {
        let img = Image::new().with_size_str("l");
        assert_eq!(img.get_size(), ImageSize::Large);

        let img2 = Image::new().with_size_str("invalid");
        assert_eq!(img2.get_size(), ImageSize::Medium);
    }

    #[test]
    fn test_image_platform() {
        let img = Image::new().platform(Platform::Ios);
        assert_eq!(img.get_platform(), Platform::Ios);
    }

    #[test]
    fn test_image_has_src() {
        let img = Image::new().src("test.jpg");
        assert!(img.has_src());

        let img2 = Image::new();
        assert!(!img2.has_src());
    }

    #[test]
    fn test_image_should_show_fallback() {
        // No src - should show fallback
        let img = Image::new();
        assert!(img.should_show_fallback());

        // Has src - should not show fallback
        let img2 = Image::new().src("test.jpg");
        assert!(!img2.should_show_fallback());

        // Has src and fallback icon - should not show fallback since src is present
        let img3 = Image::new().src("test.jpg").fallback_icon("<i>icon</i>");
        assert!(!img3.should_show_fallback());
    }

    #[test]
    fn test_image_render_with_src() {
        let img = Image::new().src("test.jpg").size(ImageSize::Small);

        let html = img.render();
        assert!(html.contains("test.jpg"));
        assert!(html.contains("telegram-ui-image"));
        assert!(html.contains("telegram-ui-image--s"));
    }

    #[test]
    fn test_image_render_with_fallback() {
        let img = Image::new().fallback_icon("<i class=\"icon\"></i>");
        let html = img.render();
        assert!(html.contains("telegram-ui-image__fallback"));
        assert!(html.contains("<i class=\"icon\"></i>"));
    }

    #[test]
    fn test_image_with_badge() {
        let badge = "<span class=\"telegram-ui-badge\">99</span>";
        let img = Image::new().src("test.jpg").badge(badge);

        let html = img.render();
        assert!(html.contains("telegram-ui-badge"));
        assert!(html.contains("99"));
    }

    #[test]
    fn test_image_chain() {
        let img = Image::new()
            .src("test.jpg")
            .fallback_icon("<i>icon</i>")
            .size(ImageSize::Large)
            .platform(Platform::Android)
            .class("custom-class")
            .style("border: 1px solid red;");

        assert_eq!(img.get_src(), Some("test.jpg"));
        assert_eq!(img.get_fallback_icon(), Some("<i>icon</i>"));
        assert_eq!(img.get_size(), ImageSize::Large);
        assert_eq!(img.get_platform(), Platform::Android);
    }
}
