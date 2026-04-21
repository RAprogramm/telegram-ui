// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Input component

use std::fmt;

use crate::error::{Result, ValidationError};

#[derive(Clone, Debug)]
pub struct Input {
    value:       String,
    placeholder: String,
    required:    bool,
    min_length:  Option<usize>,
    max_length:  Option<usize>
}

impl Input {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value:       String::new(),
            placeholder: String::new(),
            required:    false,
            min_length:  None,
            max_length:  None
        }
    }

    #[must_use]
    pub fn with_value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    #[must_use]
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    #[must_use]
    pub const fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    #[must_use]
    pub const fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    #[must_use]
    pub const fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.required && self.value.trim().is_empty() {
            errors.push("Field is required".to_string());
        }

        if let Some(min_len) = self.min_length
            && self.value.len() < min_len
        {
            errors.push(format!("Minimum length is {min_len}"));
        }

        if let Some(max_len) = self.max_length
            && self.value.len() > max_len
        {
            errors.push(format!("Maximum length is {max_len}"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let validation_error = ValidationError::with_messages("input".to_string(), errors);
            Err(validation_error.into())
        }
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

    #[test]
    fn test_input_validation_required() {
        let input = Input::new().with_value("").with_required(true);
        let result = input.validate();
        assert!(result.is_err());

        let input = Input::new().with_value("   ").with_required(true);
        let result = input.validate();
        assert!(result.is_err());

        let input = Input::new().with_value("valid").with_required(true);
        let result = input.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_validation_min_length() {
        let input = Input::new().with_value("hi").with_min_length(5);
        let result = input.validate();
        assert!(result.is_err());

        let input = Input::new().with_value("hello").with_min_length(5);
        let result = input.validate();
        assert!(result.is_ok());

        let input = Input::new().with_value("hello world").with_min_length(5);
        let result = input.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_validation_max_length() {
        let input = Input::new().with_value("hello world").with_max_length(5);
        let result = input.validate();
        assert!(result.is_err());

        let input = Input::new().with_value("hello").with_max_length(5);
        let result = input.validate();
        assert!(result.is_ok());

        let input = Input::new().with_value("hi").with_max_length(5);
        let result = input.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_validation_combined() {
        let input = Input::new()
            .with_value("hi")
            .with_required(true)
            .with_min_length(5)
            .with_max_length(3);
        let result = input.validate();
        assert!(result.is_err());

        let input = Input::new()
            .with_value("hello")
            .with_required(true)
            .with_min_length(5)
            .with_max_length(10);
        let result = input.validate();
        assert!(result.is_ok());
    }
}
