// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Badge component for Telegram UI

use std::fmt;

use crate::components::typography::{Caption, Subheadline};

/// Badge type: number (shows numeric value) or dot (simple dot indicator)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BadgeType {
    Number,
    Dot
}

/// Badge visual modes matching Telegram UI design system
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BadgeMode {
    #[default]
    Primary,
    Critical,
    Secondary,
    Gray,
    White
}

/// Badge component - renders a small numeric or dot indicator
#[derive(Debug, Clone)]
pub struct Badge {
    badge_type: BadgeType,
    mode:       BadgeMode,
    large:      bool,
    value:      Option<i32>,
    children:   Option<String>
}

impl Badge {
    /// Create a new Badge
    #[must_use]
    pub fn new() -> Self {
        Self {
            badge_type: BadgeType::Number,
            mode:       BadgeMode::default(),
            large:      false,
            value:      None,
            children:   None
        }
    }

    /// Set badge type by enum
    #[must_use]
    pub const fn badge_type(mut self, badge_type: BadgeType) -> Self {
        self.badge_type = badge_type;
        self
    }

    /// Set badge type by string (for dynamic type selection)
    #[must_use]
    pub fn with_type_str(mut self, badge_type: &str) -> Self {
        self.badge_type = match badge_type {
            "dot" | "Dot" => BadgeType::Dot,
            _ => BadgeType::Number
        };
        self
    }

    /// Set number value (for type=number)
    #[must_use]
    pub const fn with_value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    /// Set dot mode
    #[must_use]
    pub const fn with_dot(mut self) -> Self {
        self.badge_type = BadgeType::Dot;
        self
    }

    /// Set mode: primary, critical, secondary, gray, white
    #[must_use]
    pub const fn mode(mut self, mode: BadgeMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set mode by string
    #[must_use]
    pub fn with_mode(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "primary" => BadgeMode::Primary,
            "critical" => BadgeMode::Critical,
            "secondary" => BadgeMode::Secondary,
            "gray" => BadgeMode::Gray,
            "white" => BadgeMode::White,
            _ => BadgeMode::Primary
        };
        self
    }

    /// Make badge larger (only for number type)
    #[must_use]
    pub const fn large(mut self, large: bool) -> Self {
        self.large = large;
        self
    }

    /// Set custom children text
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Render the badge as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let type_class = match self.badge_type {
            BadgeType::Number => "telegram-ui-badge--number",
            BadgeType::Dot => "telegram-ui-badge--dot"
        };

        let mode_class = match self.mode {
            BadgeMode::Primary => "telegram-ui-badge--primary",
            BadgeMode::Critical => "telegram-ui-badge--critical",
            BadgeMode::Secondary => "telegram-ui-badge--secondary",
            BadgeMode::Gray => "telegram-ui-badge--gray",
            BadgeMode::White => "telegram-ui-badge--white"
        };

        let large_class = if self.large && self.badge_type == BadgeType::Number {
            "telegram-ui-badge--large"
        } else {
            ""
        };

        let class = format!("telegram-ui-badge {type_class} {mode_class} {large_class}")
            .trim()
            .to_string();

        if self.badge_type == BadgeType::Dot {
            return format!("<span class=\"{class}\"></span>");
        }

        if let Some(ref text) = self.children {
            if self.large {
                return Subheadline::new()
                    .with_text(text)
                    .with_level("2")
                    .with_weight("2")
                    .with_component("span")
                    .render();
            }
            return Caption::new().with_text(text).with_bold(false).render();
        }

        let content = self.value.map(|v| v.to_string()).unwrap_or_default();

        format!("<span class=\"{class}\">{content}</span>")
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.badge_type {
            BadgeType::Dot => write!(f, "Badge(dot)"),
            BadgeType::Number => {
                let val = self.children.clone().or(self.value.map(|v| v.to_string()));
                write!(f, "Badge({val:?})")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_default() {
        let badge = Badge::new();
        assert_eq!(badge.badge_type, BadgeType::Number);
        assert_eq!(badge.mode, BadgeMode::Primary);
        assert!(!badge.large);
        assert!(badge.value.is_none());
    }

    #[test]
    fn test_badge_number() {
        let badge = Badge::new().with_value(99);
        assert_eq!(
            badge.render(),
            "<span class=\"telegram-ui-badge telegram-ui-badge--number telegram-ui-badge--primary\">99</span>"
        );
    }

    #[test]
    fn test_badge_dot() {
        let badge = Badge::new().with_dot();
        assert_eq!(
            badge.render(),
            "<span class=\"telegram-ui-badge telegram-ui-badge--dot telegram-ui-badge--primary\"></span>"
        );
    }

    #[test]
    fn test_badge_critical() {
        let badge = Badge::new().with_value(3).mode(BadgeMode::Critical);
        assert_eq!(
            badge.render(),
            "<span class=\"telegram-ui-badge telegram-ui-badge--number telegram-ui-badge--critical\">3</span>"
        );
    }

    #[test]
    fn test_badge_large() {
        let badge = Badge::new().with_value(1).large(true);
        assert!(badge.render().contains("telegram-ui-badge--large"));
    }

    #[test]
    fn test_badge_children_large() {
        let badge = Badge::new().children("NEW").large(true);
        let html = badge.render();
        assert!(html.contains("telegram-ui-subheadline"));
        assert!(html.contains("telegram-ui-subheadline--2"));
        assert!(html.contains("NEW"));
    }

    #[test]
    fn test_badge_children_small() {
        let badge = Badge::new().children("OK");
        let html = badge.render();
        assert!(html.contains("telegram-ui-caption"));
        assert!(html.contains("OK"));
    }

    #[test]
    fn test_badge_type_str() {
        let badge = Badge::new().with_type_str("dot");
        assert_eq!(badge.badge_type, BadgeType::Dot);
    }

    #[test]
    fn test_badge_with_mode_str() {
        let badge = Badge::new().with_value(5).with_mode("critical");
        assert_eq!(badge.mode, BadgeMode::Critical);
    }
}
