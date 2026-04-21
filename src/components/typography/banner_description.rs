// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `BannerDescriptionTypography` component

use std::fmt;

#[derive(Clone, Debug)]
pub struct BannerDescriptionTypography {
    text:      String,
    platform:  String,
    level:     String,
    weight:    String,
    component: String,
    plain:     bool,
    caps:      bool
}

impl BannerDescriptionTypography {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text:      String::new(),
            platform:  "ios".to_string(),
            level:     "1".to_string(),
            weight:    "3".to_string(),
            component: "span".to_string(),
            plain:     true,
            caps:      false
        }
    }

    #[must_use]
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn text_mut(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    #[must_use]
    pub fn with_platform(mut self, platform: &str) -> Self {
        self.platform = platform.to_string();
        self
    }

    #[must_use]
    pub fn platform(&self) -> &str {
        &self.platform
    }

    #[must_use]
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    #[must_use]
    pub fn level(&self) -> &str {
        &self.level
    }

    #[must_use]
    pub fn with_weight(mut self, weight: &str) -> Self {
        self.weight = weight.to_string();
        self
    }

    #[must_use]
    pub fn weight(&self) -> &str {
        &self.weight
    }

    #[must_use]
    pub fn with_component(mut self, component: &str) -> Self {
        self.component = component.to_string();
        self
    }

    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    #[must_use]
    pub const fn plain(&self) -> bool {
        self.plain
    }

    #[must_use]
    pub const fn with_plain(mut self, plain: bool) -> Self {
        self.plain = plain;
        self
    }

    #[must_use]
    pub const fn caps(&self) -> bool {
        self.caps
    }

    #[must_use]
    pub const fn with_caps(mut self, caps: bool) -> Self {
        self.caps = caps;
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        if self.platform == "ios" {
            let mut classes = vec!["telegram-ui-caption".to_string()];

            match self.level.as_str() {
                "1" => classes.push("telegram-ui-caption--1".to_string()),
                "2" => classes.push("telegram-ui-caption--2".to_string()),
                _ => {}
            }

            if !self.plain {
                classes.push("telegram-ui-caption--plain".to_string());
            }

            if self.caps {
                classes.push("telegram-ui-caption--caps".to_string());
            }

            let class_str = classes.join(" ");

            format!(
                "<{} class=\"{}\">{}</{}>",
                self.component, class_str, self.text, self.component
            )
        } else {
            let mut classes = vec!["telegram-ui-subheadline".to_string()];

            match self.level.as_str() {
                "1" => classes.push("telegram-ui-subheadline--1".to_string()),
                "2" => classes.push("telegram-ui-subheadline--2".to_string()),
                _ => {}
            }

            match self.weight.as_str() {
                "1" => classes.push("telegram-ui-subheadline--weight-1".to_string()),
                "2" => classes.push("telegram-ui-subheadline--weight-2".to_string()),
                "3" => classes.push("telegram-ui-subheadline--weight-3".to_string()),
                _ => {}
            }

            if !self.plain {
                classes.push("telegram-ui-subheadline--plain".to_string());
            }

            if self.caps {
                classes.push("telegram-ui-subheadline--caps".to_string());
            }

            let class_str = classes.join(" ");

            format!(
                "<{} class=\"{}\">{}</{}>",
                self.component, class_str, self.text, self.component
            )
        }
    }
}

impl Default for BannerDescriptionTypography {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BannerDescriptionTypography {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_description_typography_default() {
        let banner_typography = BannerDescriptionTypography::new();
        assert_eq!(banner_typography.text(), "");
        assert_eq!(banner_typography.platform(), "ios");
        assert_eq!(banner_typography.level(), "1");
        assert_eq!(banner_typography.weight(), "3");
        assert_eq!(banner_typography.component(), "span");
        assert!(banner_typography.plain());
        assert!(!banner_typography.caps());
    }

    #[test]
    fn test_banner_description_typography_custom_ios() {
        let banner_typography = BannerDescriptionTypography::new()
            .with_text("Banner")
            .with_platform("ios")
            .with_level("2")
            .with_weight("1")
            .with_component("h6")
            .with_plain(false)
            .with_caps(true);

        assert_eq!(banner_typography.text(), "Banner");
        assert_eq!(banner_typography.platform(), "ios");
        assert_eq!(banner_typography.level(), "2");
        assert_eq!(banner_typography.weight(), "1");
        assert_eq!(banner_typography.component(), "h6");
        assert!(!banner_typography.plain());
        assert!(banner_typography.caps());
    }

    #[test]
    fn test_banner_description_typography_custom_android() {
        let banner_typography = BannerDescriptionTypography::new()
            .with_text("Banner")
            .with_platform("android")
            .with_level("2")
            .with_weight("2")
            .with_component("h6")
            .with_plain(false)
            .with_caps(true);

        assert_eq!(banner_typography.text(), "Banner");
        assert_eq!(banner_typography.platform(), "android");
        assert_eq!(banner_typography.level(), "2");
        assert_eq!(banner_typography.weight(), "2");
        assert_eq!(banner_typography.component(), "h6");
        assert!(!banner_typography.plain());
        assert!(banner_typography.caps());
    }

    #[test]
    fn test_banner_description_typography_render_ios() {
        let banner_typography = BannerDescriptionTypography::new()
            .with_text("Test")
            .with_platform("ios")
            .with_level("1");

        let rendered = banner_typography.render();
        assert!(rendered.contains("telegram-ui-caption"));
        assert!(rendered.contains("telegram-ui-caption--1"));
        assert!(rendered.contains("Test"));
    }

    #[test]
    fn test_banner_description_typography_render_android() {
        let banner_typography = BannerDescriptionTypography::new()
            .with_text("Test")
            .with_platform("android")
            .with_level("2");

        let rendered = banner_typography.render();
        assert!(rendered.contains("telegram-ui-subheadline"));
        assert!(rendered.contains("telegram-ui-subheadline--2"));
        assert!(rendered.contains("Test"));
    }
}
