// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Input component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Input {
    value: String,
    placeholder: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
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
        assert_eq!(input.value_value(), "");
        assert_eq!(input.placeholder_value(), "");
    }

    #[test]
    fn test_input_custom() {
        let input = Input::new().value("test").placeholder("Enter text");
        assert_eq!(input.value_value(), "test");
        assert_eq!(input.placeholder_value(), "Enter text");
    }
}
