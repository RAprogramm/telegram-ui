// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Input component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Input {
    value:       String,
    placeholder: String
}

impl Input {
    pub fn new() -> Self {
        Self {
            value:       String::new(),
            placeholder: String::new()
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
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_default() {
        let input = Input::new();
        assert_eq!(input.value(), "");
        assert_eq!(input.placeholder(), "");
    }

    #[test]
    fn test_input_custom() {
        let input = Input::new()
            .with_value("test")
            .with_placeholder("Enter text");
        assert_eq!(input.value(), "test");
        assert_eq!(input.placeholder(), "Enter text");
    }
}
