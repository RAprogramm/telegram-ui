// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Error handling for Telegram UI library
//!
//! This module provides a comprehensive error handling system for the library,
//! including custom error types, error conversion, and utilities.

use std::fmt;

/// Result type alias for Telegram UI operations
pub type Result<T> = std::result::Result<T, UiError>;

/// Error types for Telegram UI components
#[derive(Debug, Clone, PartialEq)]
pub enum UiError {
    /// Invalid component configuration
    InvalidConfig {
        component: String,
        details:   String
    },
    /// Invalid platform specification
    InvalidPlatform(String),
    /// Missing required property
    MissingProperty {
        component: String,
        property:  String
    },
    /// Invalid property value
    InvalidPropertyValue {
        component: String,
        property:  String,
        value:     String,
        expected:  String
    },
    /// Rendering error
    RenderError(String),
    /// Platform detection error
    PlatformDetectionError(String),
    /// Generic error
    Generic(String)
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::InvalidConfig {
                component,
                details
            } => {
                write!(f, "Invalid configuration for '{}': {}", component, details)
            }
            UiError::InvalidPlatform(platform) => {
                write!(f, "Invalid platform: {}", platform)
            }
            UiError::MissingProperty {
                component,
                property
            } => {
                write!(
                    f,
                    "Missing required property '{}' for '{}'",
                    property, component
                )
            }
            UiError::InvalidPropertyValue {
                component,
                property,
                value,
                expected
            } => {
                write!(
                    f,
                    "Invalid value '{}' for property '{}' in '{}'. Expected: {}",
                    value, property, component, expected
                )
            }
            UiError::RenderError(msg) => {
                write!(f, "Render error: {}", msg)
            }
            UiError::PlatformDetectionError(msg) => {
                write!(f, "Platform detection error: {}", msg)
            }
            UiError::Generic(msg) => {
                write!(f, "Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for UiError {}

impl UiError {
    /// Create a new invalid configuration error
    pub fn invalid_config(component: impl Into<String>, details: impl Into<String>) -> Self {
        UiError::InvalidConfig {
            component: component.into(),
            details:   details.into()
        }
    }

    /// Create a new missing property error
    pub fn missing_property(component: impl Into<String>, property: impl Into<String>) -> Self {
        UiError::MissingProperty {
            component: component.into(),
            property:  property.into()
        }
    }

    /// Create a new invalid property value error
    pub fn invalid_value(
        component: impl Into<String>,
        property: impl Into<String>,
        value: impl Into<String>,
        expected: impl Into<String>
    ) -> Self {
        UiError::InvalidPropertyValue {
            component: component.into(),
            property:  property.into(),
            value:     value.into(),
            expected:  expected.into()
        }
    }

    /// Create a new render error
    pub fn render(msg: impl Into<String>) -> Self {
        UiError::RenderError(msg.into())
    }

    /// Get the error code for this error
    pub fn code(&self) -> &'static str {
        match self {
            UiError::InvalidConfig {
                ..
            } => "INVALID_CONFIG",
            UiError::InvalidPlatform(_) => "INVALID_PLATFORM",
            UiError::MissingProperty {
                ..
            } => "MISSING_PROPERTY",
            UiError::InvalidPropertyValue {
                ..
            } => "INVALID_PROPERTY_VALUE",
            UiError::RenderError(_) => "RENDER_ERROR",
            UiError::PlatformDetectionError(_) => "PLATFORM_DETECTION_ERROR",
            UiError::Generic(_) => "GENERIC_ERROR"
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            UiError::InvalidConfig { .. }
                | UiError::MissingProperty { .. }
                | UiError::InvalidPropertyValue { .. }
        )
    }
}

impl From<String> for UiError {
    fn from(msg: String) -> Self {
        UiError::Generic(msg)
    }
}

impl From<&str> for UiError {
    fn from(msg: &str) -> Self {
        UiError::Generic(msg.to_string())
    }
}

/// Validation error for component properties
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// The component that failed validation
    pub component: String,
    /// The property that failed validation
    pub property:  Option<String>,
    /// The error messages
    pub messages:  Vec<String>
}

impl ValidationError {
    /// Create a new validation error with a single message
    pub fn new(component: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            property:  None,
            messages:  vec![message.into()]
        }
    }

    /// Create a new validation error with multiple messages
    pub fn with_messages(component: impl Into<String>, messages: Vec<String>) -> Self {
        Self {
            component: component.into(),
            property: None,
            messages
        }
    }

    /// Specify the property that failed validation
    pub fn with_property(mut self, property: impl Into<String>) -> Self {
        self.property = Some(property.into());
        self
    }

    /// Get all error messages as a combined string
    pub fn combined_message(&self) -> String {
        self.messages.join(", ")
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prop) = &self.property {
            write!(
                f,
                "[{}::{}] {}",
                self.component,
                prop,
                self.combined_message()
            )
        } else {
            write!(f, "[{}] {}", self.component, self.combined_message())
        }
    }
}

impl From<ValidationError> for UiError {
    fn from(err: ValidationError) -> Self {
        UiError::Generic(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let errors = vec![
            (
                UiError::invalid_config("Button", "Invalid type"),
                "Invalid configuration for 'Button': Invalid type"
            ),
            (
                UiError::InvalidPlatform("unknown".to_string()),
                "Invalid platform: unknown"
            ),
            (
                UiError::missing_property("Card", "title"),
                "Missing required property 'title' for 'Card'"
            ),
            (
                UiError::invalid_value("Input", "type", "invalid", "text, number, password"),
                "Invalid value 'invalid' for property 'type' in 'Input'. Expected: text, number, password"
            ),
            (
                UiError::render("Failed to render HTML"),
                "Render error: Failed to render HTML"
            ),
            (
                UiError::Generic("Something went wrong".to_string()),
                "Error: Something went wrong"
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(format!("{}", error), expected);
        }
    }

    #[test]
    fn test_error_code() {
        assert_eq!(UiError::invalid_config("", "").code(), "INVALID_CONFIG");
        assert_eq!(
            UiError::InvalidPlatform("".to_string()).code(),
            "INVALID_PLATFORM"
        );
        assert_eq!(UiError::missing_property("", "").code(), "MISSING_PROPERTY");
        assert_eq!(
            UiError::invalid_value("", "", "", "").code(),
            "INVALID_PROPERTY_VALUE"
        );
        assert_eq!(UiError::render("").code(), "RENDER_ERROR");
        assert_eq!(UiError::Generic("".to_string()).code(), "GENERIC_ERROR");
    }

    #[test]
    fn test_is_recoverable() {
        assert!(UiError::invalid_config("", "").is_recoverable());
        assert!(UiError::missing_property("", "").is_recoverable());
        assert!(UiError::invalid_value("", "", "", "").is_recoverable());
        assert!(!UiError::render("").is_recoverable());
        assert!(!UiError::Generic("".to_string()).is_recoverable());
    }

    #[test]
    fn test_validation_error() {
        let error = ValidationError::new("Button", "Invalid type").with_property("type");
        assert_eq!(format!("{}", error), "[Button::type] Invalid type");
    }

    #[test]
    fn test_from_str_to_error() {
        let error: UiError = "test error".into();
        assert_eq!(error.code(), "GENERIC_ERROR");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(
            UiError::invalid_config("A", "B"),
            UiError::invalid_config("A", "B")
        );
        assert_ne!(
            UiError::invalid_config("A", "B"),
            UiError::invalid_config("A", "C")
        );
    }
}
