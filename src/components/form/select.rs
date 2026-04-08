// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Select component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Select {
    value:   String,
    options: Vec<String>
}

impl Select {
    pub fn new() -> Self {
        Self {
            value:   String::new(),
            options: Vec::new()
        }
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_default() {
        let select = Select::new();
        assert_eq!(select.value(), "");
        assert!(select.options().is_empty());
    }

    #[test]
    fn test_select_custom() {
        let select = Select::new()
            .with_value("option1")
            .with_options(vec!["option1".to_string(), "option2".to_string()]);
        assert_eq!(select.value(), "option1");
        assert_eq!(select.options().len(), 2);
    }
}
