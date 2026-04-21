// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `LargeTitle` component

use std::fmt;

#[derive(Clone, Debug)]
pub struct LargeTitle {
    text: String
}

impl LargeTitle {
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
        format!("<div class=\"telegram-ui-largetitle\">{}</div>", self.text)
    }
}

impl Default for LargeTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LargeTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largetitle_default() {
        let largetitle = LargeTitle::new();
        assert_eq!(largetitle.text(), "");
    }

    #[test]
    fn test_largetitle_custom() {
        let largetitle = LargeTitle::new().with_text("LargeTitle");
        assert_eq!(largetitle.text(), "LargeTitle");
    }
}
