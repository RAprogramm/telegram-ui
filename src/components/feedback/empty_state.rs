// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! EmptyState component

use std::fmt;

use crate::helpers::escape_html;

/// Empty state component
#[derive(Clone, Debug)]
pub struct EmptyState {
    header:      String,
    description: String,
    action:      String
}

impl EmptyState {
    pub fn new() -> Self {
        Self {
            header:      String::new(),
            description: String::new(),
            action:      String::new()
        }
    }

    pub fn header(mut self, header: &str) -> Self {
        self.header = header.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.action = action.to_string();
        self
    }

    pub fn header_text(&self) -> &str {
        &self.header
    }

    pub fn description_text(&self) -> &str {
        &self.description
    }

    pub fn action_text(&self) -> &str {
        &self.action
    }

    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-empty-state\">\n  <h3>{}</h3>\n  <p>{}</p>\n  <p>{}</p>\n</div>",
            escape_html(&self.header),
            escape_html(&self.description),
            escape_html(&self.action)
        )
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EmptyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyState")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_default() {
        let empty = EmptyState::new();
        assert!(empty.header_text().is_empty());
        assert!(empty.description_text().is_empty());
        assert!(empty.action_text().is_empty());
    }

    #[test]
    fn test_empty_state_builder() {
        let empty = EmptyState::new()
            .header("No items")
            .description("Add your first item")
            .action("Add");

        assert_eq!(empty.header_text(), "No items");
        assert_eq!(empty.description_text(), "Add your first item");
        assert_eq!(empty.action_text(), "Add");
    }

    #[test]
    fn test_empty_state_render() {
        let empty = EmptyState::new()
            .header("Title")
            .description("Description")
            .action("Action");

        let html = empty.render();
        assert!(html.contains("telegram-ui-empty-state"));
        assert!(html.contains("Title"));
    }
}
