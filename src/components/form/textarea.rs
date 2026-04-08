// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Textarea component

use std::fmt;

use crate::error::{Result, ValidationError};

#[derive(Clone, Debug)]
pub struct Textarea {
    value: String,
    placeholder: String,
    rows: u32,
    required: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl Textarea {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            rows: 3,
            required: false,
            min_length: None,
            max_length: None,
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

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
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
            errors.push(format!("Minimum length is {}", min_len));
        }

        if let Some(max_len) = self.max_length
            && self.value.len() > max_len
        {
            errors.push(format!("Maximum length is {}", max_len));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let validation_error = ValidationError::with_messages("textarea".to_string(), errors);
            Err(validation_error.into())
        }
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

    #[test]
    fn test_textarea_validation_required() {
        let textarea = Textarea::new().with_value("").with_required(true);
        let result = textarea.validate();
        assert!(result.is_err());

        let textarea = Textarea::new().with_value("   ").with_required(true);
        let result = textarea.validate();
        assert!(result.is_err());

        let textarea = Textarea::new().with_value("valid").with_required(true);
        let result = textarea.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_textarea_validation_min_length() {
        let textarea = Textarea::new().with_value("hi").with_min_length(5);
        let result = textarea.validate();
        assert!(result.is_err());

        let textarea = Textarea::new().with_value("hello").with_min_length(5);
        let result = textarea.validate();
        assert!(result.is_ok());

        let textarea = Textarea::new().with_value("hello world").with_min_length(5);
        let result = textarea.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_textarea_validation_max_length() {
        let textarea = Textarea::new().with_value("hello world").with_max_length(5);
        let result = textarea.validate();
        assert!(result.is_err());

        let textarea = Textarea::new().with_value("hello").with_max_length(5);
        let result = textarea.validate();
        assert!(result.is_ok());

        let textarea = Textarea::new().with_value("hi").with_max_length(5);
        let result = textarea.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_textarea_validation_combined() {
        let textarea = Textarea::new()
            .with_value("hi")
            .with_required(true)
            .with_min_length(5)
            .with_max_length(3);
        let result = textarea.validate();
        assert!(result.is_err());

        let textarea = Textarea::new()
            .with_value("hello")
            .with_required(true)
            .with_min_length(5)
            .with_max_length(10);
        let result = textarea.validate();
        assert!(result.is_ok());
    }
}
