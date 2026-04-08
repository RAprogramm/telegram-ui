// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Title component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Title {
    text:  String,
    align: String
}

impl Title {
    pub fn new() -> Self {
        Self {
            text:  String::new(),
            align: "left".to_string()
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn with_align(mut self, align: &str) -> Self {
        self.align = align.to_string();
        self
    }

    pub fn align(&self) -> &str {
        &self.align
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_default() {
        let title = Title::new();
        assert_eq!(title.text(), "");
        assert_eq!(title.align(), "left");
    }

    #[test]
    fn test_title_custom() {
        let title = Title::new().with_text("Hello").with_align("center");
        assert_eq!(title.text(), "Hello");
        assert_eq!(title.align(), "center");
    }
}
