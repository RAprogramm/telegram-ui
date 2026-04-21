// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Subtitle component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Subtitle {
    text: String
}

impl Subtitle {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: String::new()
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
    pub fn render(&self) -> String {
        format!("<div class=\"telegram-ui-subtitle\">{}</div>", self.text)
    }
}

impl Default for Subtitle {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtitle_default() {
        let subtitle = Subtitle::new();
        assert_eq!(subtitle.text(), "");
    }

    #[test]
    fn test_subtitle_custom() {
        let subtitle = Subtitle::new().with_text("Subtitle");
        assert_eq!(subtitle.text(), "Subtitle");
    }
}
