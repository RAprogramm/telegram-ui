// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Placeholder component for Telegram UI

use std::fmt;

/// Placeholder component
#[derive(Debug, Clone)]
pub struct Placeholder {
    title: String,
    description: Option<String>,
}

impl Placeholder {
    /// Creates a new Placeholder with default settings
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: None,
        }
    }

    /// Sets the placeholder title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the placeholder description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Returns the placeholder title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Returns the placeholder description
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Render the placeholder as HTML string
    pub fn render(&self) -> String {
        let description_html = self
            .description
            .as_ref()
            .map(|d| {
                format!(
                    "<div class=\"telegram-ui-placeholder-description\">{}</div>",
                    d
                )
            })
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-placeholder\">\n  <div class=\"telegram-ui-placeholder-title\">{}</div>\n  {}\n</div>",
            self.title, description_html
        )
    }
}

impl Default for Placeholder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_default() {
        let placeholder = Placeholder::new();
        assert_eq!(placeholder.get_title(), "");
        assert_eq!(placeholder.get_description(), None);
    }

    #[test]
    fn test_placeholder_custom() {
        let placeholder = Placeholder::new()
            .title("No Data")
            .description("Please try again later");

        assert_eq!(placeholder.get_title(), "No Data");
        assert_eq!(
            placeholder.get_description(),
            Some("Please try again later")
        );
    }

    #[test]
    fn test_placeholder_render() {
        let placeholder = Placeholder::new()
            .title("Loading...")
            .description("Please wait");

        let html = placeholder.render();
        assert!(html.contains("telegram-ui-placeholder"));
        assert!(html.contains("telegram-ui-placeholder-title"));
        assert!(html.contains("Loading..."));
        assert!(html.contains("Please wait"));
    }

    #[test]
    fn test_placeholder_without_description() {
        let placeholder = Placeholder::new().title("Title");
        let html = placeholder.render();
        assert!(html.contains("Title"));
        assert!(!html.contains("telegram-ui-placeholder-description"));
    }
}
