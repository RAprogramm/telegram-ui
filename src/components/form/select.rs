// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Select component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Select {
    value: String,
    options: Vec<String>,
}

impl Select {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            options: Vec::new(),
        }
    }

    pub fn value(&mut self, value: &str) -> &mut Self {
        self.value = value.to_string();
        self
    }

    pub fn value_value(&self) -> &str {
        &self.value
    }

    pub fn options(&mut self, options: Vec<String>) -> &mut Self {
        self.options = options;
        self
    }

    pub fn options_value(&self) -> &[String] {
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
        assert_eq!(select.value_value(), "");
        assert!(select.options_value().is_empty());
    }

    #[test]
    fn test_select_custom() {
        let select = Select::new()
            .value("option1")
            .options(vec!["option1".to_string(), "option2".to_string()]);
        assert_eq!(select.value_value(), "option1");
        assert_eq!(select.options_value().len(), 2);
    }
}
