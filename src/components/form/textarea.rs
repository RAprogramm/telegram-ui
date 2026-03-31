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

    pub fn value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_string();
        self
    }

    pub fn value_value(&self) -> &str {
        &self.value
    }

    pub fn placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn placeholder_value(&self) -> &str {
        &self.placeholder
    }

    pub fn rows(&mut self, rows: u32) -> &mut Self {
        self.rows = rows;
        self
    }

    pub fn rows_value(&self) -> u32 {
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
        assert_eq!(textarea.value_value(), "");
        assert_eq!(textarea.placeholder_value(), "");
        assert_eq!(textarea.rows_value(), 3);
    }

    #[test]
    fn test_textarea_custom() {
        let textarea = Textarea::new()
            .value("text")
            .placeholder("Enter text")
            .rows(5);
        assert_eq!(textarea.value_value(), "text");
        assert_eq!(textarea.placeholder_value(), "Enter text");
        assert_eq!(textarea.rows_value(), 5);
    }
}
