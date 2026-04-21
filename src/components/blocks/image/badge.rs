// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! ImageBadge component for Telegram UI

use crate::components::service::badge::{Badge, BadgeMode, BadgeType};

/// ImageBadge - wrapper around Badge for use in Image component
///
/// Only supports type="number" - other types will return None
#[derive(Debug, Clone)]
pub struct ImageBadge {
    badge: Badge,
}

impl ImageBadge {
    /// Create a new ImageBadge
    pub fn new() -> Self {
        Self {
            badge: Badge::new(),
        }
    }

    /// Set badge type (only number is supported)
    pub fn badge_type(mut self, badge_type: BadgeType) -> Self {
        self.badge = self.badge.badge_type(badge_type);
        self
    }

    /// Set badge type by string
    pub fn with_type_str(mut self, badge_type: &str) -> Self {
        self.badge = self.badge.with_type_str(badge_type);
        self
    }

    /// Set number value
    pub fn with_value(mut self, value: i32) -> Self {
        self.badge = self.badge.with_value(value);
        self
    }

    /// Set mode
    pub fn mode(mut self, mode: BadgeMode) -> Self {
        self.badge = self.badge.mode(mode);
        self
    }

    /// Set mode by string
    pub fn with_mode(mut self, mode: &str) -> Self {
        self.badge = self.badge.with_mode(mode);
        self
    }

    /// Set large variant
    pub fn large(mut self, large: bool) -> Self {
        self.badge = self.badge.large(large);
        self
    }

    /// Set custom children text
    pub fn children(mut self, children: &str) -> Self {
        self.badge = self.badge.children(children);
        self
    }

    /// Render the ImageBadge as HTML
    ///
    /// Only type="number" is supported. Other types will return None.
    pub fn render(&self) -> Option<String> {
        match self.badge.badge_type {
            BadgeType::Number => Some(format!(
                "<span class=\"telegram-ui-badge telegram-ui-badge--number {} {}\">{}</span>",
                self.mode_class(),
                if self.badge.large {
                    "telegram-ui-badge--large"
                } else {
                    ""
                },
                self.badge
                    .children
                    .as_ref()
                    .or(self.badge.value.map(|v| v.to_string()))
                    .unwrap_or_default()
            )),
            BadgeType::Dot => None,
        }
    }

    fn mode_class(&self) -> &'static str {
        match self.badge.mode {
            BadgeMode::Primary => "telegram-ui-badge--primary",
            BadgeMode::Critical => "telegram-ui-badge--critical",
            BadgeMode::Secondary => "telegram-ui-badge--secondary",
            BadgeMode::Gray => "telegram-ui-badge--gray",
            BadgeMode::White => "telegram-ui-badge--white",
        }
    }
}

impl Default for ImageBadge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imagebadge_number() {
        let badge = ImageBadge::new().with_value(99);
        assert_eq!(
            badge.render(),
            Some(String::from(
                "<span class=\"telegram-ui-badge telegram-ui-badge--number telegram-ui-badge--primary\">99</span>"
            ))
        );
    }

    #[test]
    fn test_imagebadge_dot_returns_none() {
        let badge = ImageBadge::new().badge_type(BadgeType::Dot);
        assert_eq!(badge.render(), None);
    }

    #[test]
    fn test_imagebadge_critical_mode() {
        let badge = ImageBadge::new().with_value(3).with_mode("critical");
        assert!(badge
            .render()
            .unwrap()
            .contains("telegram-ui-badge--critical"));
    }

    #[test]
    fn test_imagebadge_large() {
        let badge = ImageBadge::new().with_value(1).large(true);
        let html = badge.render().unwrap();
        assert!(html.contains("telegram-ui-badge--large"));
    }

    #[test]
    fn test_imagebadge_white_mode() {
        let badge = ImageBadge::new().with_value(5).with_mode("white");
        assert!(badge.render().unwrap().contains("telegram-ui-badge--white"));
    }
}
