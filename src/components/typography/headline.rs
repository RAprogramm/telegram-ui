// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Headline component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Headline {
    text:  String,
    level: u32
}

impl Headline {
    pub fn new() -> Self {
        Self {
            text:  String::new(),
            level: 1
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn level(&self) -> u32 {
        self.level
    }
}

impl Default for Headline {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Headline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headline_default() {
        let headline = Headline::new();
        assert_eq!(headline.text(), "");
        assert_eq!(headline.level(), 1);
    }

    #[test]
    fn test_headline_custom() {
        let headline = Headline::new().with_text("Headline").with_level(2);
        assert_eq!(headline.text(), "Headline");
        assert_eq!(headline.level(), 2);
    }
}
