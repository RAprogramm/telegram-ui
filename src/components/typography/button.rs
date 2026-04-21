// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `ButtonTypography` component

use std::fmt;

#[derive(Clone, Debug)]
pub struct ButtonTypography {
    text:      String,
    size:      String,
    weight:    String,
    component: String,
    plain:     bool,
    caps:      bool
}

impl ButtonTypography {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text:      String::new(),
            size:      "m".to_string(),
            weight:    "2".to_string(),
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
    pub fn with_size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    #[must_use]
    pub fn size(&self) -> &str {
        &self.size
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
        let mut classes = vec!["telegram-ui-subheadline".to_string()];

        if self.size == "l" {
            let classes = ["telegram-ui-text".to_string()];
            let class_str = classes.join(" ");
            format!(
                "<{} class=\"{}\">{}</{}>",
                self.component, class_str, self.text, self.component
            )
        } else {
            match self.size.as_str() {
                "s" => classes.push("telegram-ui-subheadline--s".to_string()),
                "m" => classes.push("telegram-ui-subheadline--m".to_string()),
                "l" => classes.push("telegram-ui-subheadline--l".to_string()),
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

impl Default for ButtonTypography {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ButtonTypography {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_typography_default() {
        let button_typography = ButtonTypography::new();
        assert_eq!(button_typography.text(), "");
        assert_eq!(button_typography.size(), "m");
        assert_eq!(button_typography.weight(), "2");
        assert_eq!(button_typography.component(), "span");
        assert!(button_typography.plain());
        assert!(!button_typography.caps());
    }

    #[test]
    fn test_button_typography_custom() {
        let button_typography = ButtonTypography::new()
            .with_text("Button")
            .with_size("l")
            .with_weight("1")
            .with_component("h6")
            .with_plain(false)
            .with_caps(true);

        assert_eq!(button_typography.text(), "Button");
        assert_eq!(button_typography.size(), "l");
        assert_eq!(button_typography.weight(), "1");
        assert_eq!(button_typography.component(), "h6");
        assert!(!button_typography.plain());
        assert!(button_typography.caps());
    }

    #[test]
    fn test_button_typography_render_small() {
        let button_typography = ButtonTypography::new()
            .with_text("Test")
            .with_size("s")
            .with_weight("2");

        let rendered = button_typography.render();
        assert!(rendered.contains("telegram-ui-subheadline"));
        assert!(rendered.contains("telegram-ui-subheadline--s"));
        assert!(rendered.contains("telegram-ui-subheadline--weight-2"));
        assert!(rendered.contains("Test"));
    }

    #[test]
    fn test_button_typography_render_large() {
        let button_typography = ButtonTypography::new()
            .with_text("Test")
            .with_size("l")
            .with_weight("2");

        let rendered = button_typography.render();
        assert!(rendered.contains("telegram-ui-text"));
        assert!(rendered.contains("Test"));
    }
}
