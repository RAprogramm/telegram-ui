// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Card component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Card component
#[derive(Debug, Clone)]
pub struct Card {
    ambient:  bool,
    children: String
}

impl Card {
    /// Creates a new Card with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ambient:  false,
            children: String::new()
        }
    }

    /// Sets whether the card should use ambient style
    #[must_use]
    pub const fn ambient(mut self, ambient: bool) -> Self {
        self.ambient = ambient;
        self
    }

    /// Sets the card children content
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns whether the card uses ambient style
    #[must_use]
    pub const fn is_ambient(&self) -> bool {
        self.ambient
    }

    /// Returns the card children content
    #[must_use]
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the card as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let class = if self.ambient {
            "telegram-ui-card telegram-ui-card--ambient"
        } else {
            "telegram-ui-card"
        };

        format!(
            "<div class=\"{}\">{}</div>",
            class,
            escape_html(&self.children)
        )
    }
}

/// `CardCell` component - displays cell content in a Card
#[derive(Debug, Clone)]
pub struct CardCell {
    title:       String,
    description: String
}

impl CardCell {
    /// Creates a new `CardCell` with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            title:       String::new(),
            description: String::new()
        }
    }

    /// Sets the cell title
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the cell description
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Render the card cell as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-card-cell\">\n  <h3>{}</h3>\n  <p>{}</p>\n</div>",
            escape_html(&self.title),
            escape_html(&self.description)
        )
    }
}

impl Default for CardCell {
    fn default() -> Self {
        Self::new()
    }
}

/// `CardChip` component - displays a small badge/label in a Card
#[derive(Debug, Clone)]
pub struct CardChip {
    text: String
}

impl CardChip {
    /// Creates a new `CardChip` with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: String::new()
        }
    }

    /// Sets the chip text
    #[must_use]
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Render the card chip as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<span class=\"telegram-ui-card-chip\">{}</span>",
            escape_html(&self.text)
        )
    }
}

impl Default for CardChip {
    fn default() -> Self {
        Self::new()
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
        assert!(html.contains("&lt;p&gt;Hello&lt;/p&gt;"));
    }

    #[test]
    fn test_card_ambient() {
        let card = Card::new().ambient(true);
        let html = card.render();
        assert!(html.contains("telegram-ui-card--ambient"));
    }
}
