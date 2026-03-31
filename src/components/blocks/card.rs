// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Card component for Telegram UI

use std::fmt;

/// Card component
#[derive(Debug, Clone)]
pub struct Card {
    ambient: bool,
    children: String,
}

impl Card {
    /// Creates a new Card with default settings
    pub fn new() -> Self {
        Self {
            ambient: false,
            children: String::new(),
        }
    }

    /// Sets whether the card should use ambient style
    pub fn ambient(mut self, ambient: bool) -> Self {
        self.ambient = ambient;
        self
    }

    /// Sets the card children content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns whether the card uses ambient style
    pub fn is_ambient(&self) -> bool {
        self.ambient
    }

    /// Returns the card children content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the card as HTML string
    pub fn render(&self) -> String {
        let class = if self.ambient {
            "telegram-ui-card telegram-ui-card--ambient"
        } else {
            "telegram-ui-card"
        };

        format!(
            "<div class=\"{}\">{}</div>",
            class, self.children
        )
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_default() {
        let card = Card::new();
        assert!(!card.is_ambient());
        assert_eq!(card.get_children(), "");
    }

    #[test]
    fn test_card_custom() {
        let card = Card::new().ambient(true).children("Content");
        assert!(card.is_ambient());
        assert_eq!(card.get_children(), "Content");
    }

    #[test]
    fn test_card_render() {
        let card = Card::new().children("<p>Hello</p>");
        let html = card.render();
        assert!(html.contains("telegram-ui-card"));
        assert!(html.contains("<p>Hello</p>"));
    }

    #[test]
    fn test_card_ambient() {
        let card = Card::new().ambient(true);
        let html = card.render();
        assert!(html.contains("telegram-ui-card--ambient"));
    }
}
