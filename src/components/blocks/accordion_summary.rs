// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `AccordionSummary` component for Telegram UI

use std::fmt;

use crate::components::blocks::cell::Cell;

/// Accordion summary component
#[derive(Debug, Clone)]
pub struct AccordionSummary {
    id:            Option<String>,
    aria_expanded: Option<bool>,
    aria_controls: Option<String>,
    after:         Option<String>,
    children:      String,
    on_click:      bool
}

impl AccordionSummary {
    /// Create a new `AccordionSummary`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id:            None,
            aria_expanded: None,
            aria_controls: None,
            after:         None,
            children:      String::new(),
            on_click:      true
        }
    }

    /// Set the element id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Set aria-expanded attribute
    #[must_use]
    pub const fn aria_expanded(mut self, expanded: bool) -> Self {
        self.aria_expanded = Some(expanded);
        self
    }

    /// Set aria-controls attribute
    #[must_use]
    pub fn aria_controls(mut self, controls: &str) -> Self {
        self.aria_controls = Some(controls.to_string());
        self
    }

    /// Set the content after the summary (typically an icon)
    #[must_use]
    pub fn after(mut self, content: &str) -> Self {
        self.after = Some(content.to_string());
        self
    }

    /// Set the children content (summary text)
    #[must_use]
    pub fn children(mut self, content: &str) -> Self {
        self.children = content.to_string();
        self
    }

    /// Set whether to handle click event
    #[must_use]
    pub const fn on_click(mut self, enabled: bool) -> Self {
        self.on_click = enabled;
        self
    }

    /// Render the accordion summary as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut cell = Cell::new();

        if let Some(id) = &self.id {
            cell = cell.id(id);
        }

        if let Some(expanded) = self.aria_expanded {
            cell = cell.aria_expanded(expanded);
        }

        if let Some(controls) = &self.aria_controls {
            cell = cell.aria_controls(controls);
        }

        if let Some(after) = &self.after {
            cell = cell.after(after);
        }

        cell = cell.middle(&self.children);

        cell.render()
    }
}

impl Default for AccordionSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AccordionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_summary_default() {
        let summary = AccordionSummary::new();
        assert!(summary.id.is_none());
        assert!(summary.aria_expanded.is_none());
        assert!(summary.aria_controls.is_none());
        assert!(summary.children.is_empty());
    }

    #[test]
    fn test_accordion_summary_custom() {
        let summary = AccordionSummary::new()
            .id("summary-1")
            .aria_expanded(true)
            .aria_controls("content-1")
            .children("Click to expand");

        assert_eq!(summary.id, Some("summary-1".to_string()));
        assert_eq!(summary.aria_expanded, Some(true));
        assert_eq!(summary.aria_controls, Some("content-1".to_string()));
        assert_eq!(summary.children, "Click to expand");
    }

    #[test]
    fn test_accordion_summary_render() {
        let summary = AccordionSummary::new()
            .id("summary-1")
            .aria_expanded(true)
            .aria_controls("content-1")
            .children("Section 1");

        let html = summary.render();
        assert!(html.contains("summary-1"));
        assert!(html.contains("Section 1"));
        assert!(html.contains("aria-expanded=\"true\""));
        assert!(html.contains("aria-controls=\"content-1\""));
    }

    #[test]
    fn test_accordion_summary_escape_html() {
        let summary = AccordionSummary::new().children("<script>alert(1)</script>");
        let html = summary.render();
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }

    #[test]
    fn test_accordion_summary_with_after() {
        let summary = AccordionSummary::new().children("Section 1").after("▶");

        let html = summary.render();
        assert!(html.contains("▶"));
    }
}
