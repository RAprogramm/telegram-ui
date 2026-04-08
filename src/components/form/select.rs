// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Select component

use std::fmt;

use crate::error::{Result, ValidationError};

#[derive(Clone, Debug)]
pub struct Select {
    value:    String,
    options:  Vec<String>,
    required: bool
}

impl Select {
    pub fn new() -> Self {
        Self {
            value:    String::new(),
            options:  Vec::new(),
            required: false
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

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.required && self.value.trim().is_empty() {
            errors.push("Field is required".to_string());
        }

        if !self.value.is_empty() && !self.options.contains(&self.value) {
            errors.push("Invalid option selected".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let validation_error = ValidationError::with_messages("select".to_string(), errors);
            Err(validation_error.into())
        }
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

    #[test]
    fn test_select_validation_required() {
        let select = Select::new().with_value("").with_required(true);
        let result = select.validate();
        assert!(result.is_err());

        let select = Select::new().with_value("   ").with_required(true);
        let result = select.validate();
        assert!(result.is_err());

        let select = Select::new()
            .with_value("option1")
            .with_options(vec!["option1".to_string()])
            .with_required(true);
        let result = select.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_select_validation_option_exists() {
        let select = Select::new()
            .with_value("invalid")
            .with_options(vec!["option1".to_string(), "option2".to_string()]);
        let result = select.validate();
        assert!(result.is_err());

        let select = Select::new()
            .with_value("option1")
            .with_options(vec!["option1".to_string(), "option2".to_string()]);
        let result = select.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_select_validation_combined() {
        let select = Select::new()
            .with_value("")
            .with_options(vec!["option1".to_string(), "option2".to_string()])
            .with_required(true);
        let result = select.validate();
        assert!(result.is_err());

        let select = Select::new()
            .with_value("option1")
            .with_options(vec!["option1".to_string(), "option2".to_string()])
            .with_required(true);
        let result = select.validate();
        assert!(result.is_ok());
    }
}
