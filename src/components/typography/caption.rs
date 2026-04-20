// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Caption component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Caption {
    text: String,
    bold: bool
}

impl Caption {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            bold: false
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_mut(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn is_bold(&self) -> bool {
        self.bold
    }

    pub fn render(&self) -> String {
        let bold_style = if self.bold { "font-weight: bold;" } else { "" };
        format!(
            "<div class=\"telegram-ui-caption\" style=\"{}\">{}</div>",
            bold_style, self.text
        )
    }
}

impl Default for Caption {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Caption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caption_default() {
        let caption = Caption::new();
        assert_eq!(caption.text(), "");
        assert!(!caption.is_bold());
    }

    #[test]
    fn test_caption_custom() {
        let caption = Caption::new().with_text("Caption").with_bold(true);
        assert_eq!(caption.text(), "Caption");
        assert!(caption.is_bold());
    }
}
