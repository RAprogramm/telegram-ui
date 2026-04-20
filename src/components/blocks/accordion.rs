// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Accordion component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Accordion component
#[derive(Debug, Clone)]
pub struct Accordion {
    expanded: bool,
    summary:  String,
    content:  String
}

impl Accordion {
    /// Create a new Accordion
    pub fn new() -> Self {
        Self {
            expanded: false,
            summary:  String::new(),
            content:  String::new()
        }
    }

    /// Set whether the accordion is expanded
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set the summary/header text
    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = summary.to_string();
        self
    }

    /// Set the content text
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Render the accordion as HTML string
    pub fn render(&self) -> String {
        let expanded = if self.expanded { " expanded" } else { "" };

        format!(
            r#"<div class="telegram-ui-accordion{expanded}">
                <div class="accordion-summary">
                    <span>{summary}</span>
                    <span class="accordion-arrow">{arrow}</span>
                </div>
                <div class="accordion-content">{content}</div>
            </div>"#,
            summary = escape_html(&self.summary),
            content = escape_html(&self.content),
            arrow = if self.expanded { "▼" } else { "▶" }
        )
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Accordion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_default() {
        let accordion = Accordion::new();
        assert!(!accordion.expanded);
        assert!(accordion.summary.is_empty());
    }

    #[test]
    fn test_accordion_render() {
        let accordion = Accordion::new()
            .expanded(true)
            .summary("Click me")
            .content("Hidden content");

        let html = accordion.render();
        assert!(html.contains("expanded"));
        assert!(html.contains("Click me"));
        assert!(html.contains("Hidden content"));
    }
}
