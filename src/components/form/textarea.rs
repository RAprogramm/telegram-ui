// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Textarea component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Textarea {
    value: String,
    placeholder: String,
    rows: u32,
}

impl Textarea {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            rows: 3,
        }
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn with_rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }
}

impl Default for Textarea {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Textarea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_default() {
        let textarea = Textarea::new();
        assert_eq!(textarea.value(), "");
        assert_eq!(textarea.placeholder(), "");
        assert_eq!(textarea.rows(), 3);
    }

    #[test]
    fn test_textarea_custom() {
        let textarea = Textarea::new()
            .with_value("text")
            .with_placeholder("Enter text")
            .with_rows(5);
        assert_eq!(textarea.value(), "text");
        assert_eq!(textarea.placeholder(), "Enter text");
        assert_eq!(textarea.rows(), 5);
    }
}
