// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Badge component for Telegram UI

use std::fmt;

/// Badge type: number (shows numeric value) or dot (simple dot indicator)
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeType {
    Number,
    Dot
}

/// Badge visual modes matching Telegram UI design system
#[derive(Clone, Debug, PartialEq, Default)]
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
    pub fn new() -> Self {
        Self {
            badge_type: BadgeType::Number,
            mode:       BadgeMode::default(),
            large:      false,
            value:      None,
            children:   None
        }
    }

    /// Set badge type: number or dot
    pub fn badge_type(mut self, badge_type: BadgeType) -> Self {
        self.badge_type = badge_type;
        self
    }

    /// Set number value (for type=number)
    pub fn with_value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    /// Set dot mode
    pub fn with_dot(mut self) -> Self {
        self.badge_type = BadgeType::Dot;
        self
    }

    /// Set mode: primary, critical, secondary, gray, white
    pub fn mode(mut self, mode: BadgeMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set mode by string
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
    pub fn large(mut self, large: bool) -> Self {
        self.large = large;
        self
    }

    /// Set custom children text
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Render the badge as HTML string
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

        let class = format!(
            "telegram-ui-badge {} {} {}",
            type_class, mode_class, large_class
        )
        .trim()
        .to_string();

        if self.badge_type == BadgeType::Dot {
            return format!("<span class=\"{}\"></span>", class);
        }

        let content = self
            .children
            .clone()
            .or(self.value.map(|v| v.to_string()))
            .unwrap_or_default();

        format!("<span class=\"{}\">{}</span>", class, content)
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
                write!(f, "Badge({:?})", val)
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
    fn test_badge_children() {
        let badge = Badge::new().children("NEW");
        assert_eq!(
            badge.render(),
            "<span class=\"telegram-ui-badge telegram-ui-badge--number telegram-ui-badge--primary\">NEW</span>"
        );
    }
}
