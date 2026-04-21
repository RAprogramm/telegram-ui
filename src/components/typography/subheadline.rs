// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Subheadline component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Subheadline {
    text:      String,
    level:     String,
    weight:    String,
    component: String,
    plain:     bool,
    caps:      bool
}

impl Subheadline {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text:      String::new(),
            level:     "1".to_string(),
            weight:    "3".to_string(),
            component: "h6".to_string(),
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

impl Default for Subheadline {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Subheadline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subheadline_default() {
        let subheadline = Subheadline::new();
        assert_eq!(subheadline.text(), "");
        assert_eq!(subheadline.level(), "1");
        assert_eq!(subheadline.weight(), "3");
        assert_eq!(subheadline.component(), "h6");
        assert!(subheadline.plain());
        assert!(!subheadline.caps());
    }

    #[test]
    fn test_subheadline_custom() {
        let subheadline = Subheadline::new()
            .with_text("Hello")
            .with_level("2")
            .with_weight("1")
            .with_component("h5")
            .with_plain(false)
            .with_caps(true);

        assert_eq!(subheadline.text(), "Hello");
        assert_eq!(subheadline.level(), "2");
        assert_eq!(subheadline.weight(), "1");
        assert_eq!(subheadline.component(), "h5");
        assert!(!subheadline.plain());
        assert!(subheadline.caps());
    }

    #[test]
    fn test_subheadline_render() {
        let subheadline = Subheadline::new()
            .with_text("Test")
            .with_level("1")
            .with_weight("2");

        let rendered = subheadline.render();
        assert!(rendered.contains("telegram-ui-subheadline"));
        assert!(rendered.contains("telegram-ui-subheadline--1"));
        assert!(rendered.contains("telegram-ui-subheadline--weight-2"));
        assert!(rendered.contains("Test"));
    }
}
