// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Text component

use std::fmt;

use crate::helpers::escape_html;

#[derive(Clone, Debug)]
pub struct Text {
    text:  String,
    color: String
}

impl Text {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text:  String::new(),
            color: "#000000".to_string()
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
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    #[must_use]
    pub fn color(&self) -> &str {
        &self.color
    }

    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-text\" style=\"color: {}\">{}</div>",
            self.color,
            escape_html(&self.text)
        )
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", escape_html(&self.text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_default() {
        let text = Text::new();
        assert_eq!(text.text(), "");
        assert_eq!(text.color(), "#000000");
    }

    #[test]
    fn test_text_custom() {
        let text = Text::new().with_text("Hello").with_color("#ff0000");
        assert_eq!(text.text(), "Hello");
        assert_eq!(text.color(), "#ff0000");
    }
}
