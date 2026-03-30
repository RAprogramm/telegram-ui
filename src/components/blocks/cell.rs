// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Cell component for Telegram UI

use std::fmt;

/// Cell component
#[derive(Debug, Clone)]
pub struct Cell {
    ios: bool,
    hovered: bool,
    before: Option<String>,
    after: Option<String>,
    middle: String,
}

impl Cell {
    /// Creates a new Cell with default settings
    pub fn new() -> Self {
        Self {
            ios: false,
            hovered: false,
            before: None,
            after: None,
            middle: String::new(),
        }
    }

    /// Sets whether the cell should use iOS styling
    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    /// Sets whether the cell should show hover state
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Sets content to show before the cell content
    pub fn before(mut self, content: &str) -> Self {
        self.before = Some(content.to_string());
        self
    }

    /// Sets content to show after the cell content
    pub fn after(mut self, content: &str) -> Self {
        self.after = Some(content.to_string());
        self
    }

    /// Sets the middle content of the cell
    pub fn middle(mut self, content: &str) -> Self {
        self.middle = content.to_string();
        self
    }

    /// Returns whether the cell uses iOS styling
    pub fn is_ios(&self) -> bool {
        self.ios
    }

    /// Returns whether the cell shows hover state
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Returns the cell before content
    pub fn get_before(&self) -> Option<&str> {
        self.before.as_deref()
    }

    /// Returns the cell after content
    pub fn get_after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    /// Returns the cell middle content
    pub fn get_middle(&self) -> &str {
        &self.middle
    }

    /// Render the cell as HTML string
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-cell"];

        if self.ios {
            classes.push("--ios");
        }

        if self.hovered {
            classes.push("--hovered");
        }

        let class_str = classes.join(" ");

        let mut content = String::new();

        if let Some(before) = &self.before {
            content.push_str(&format!("<div class=\"before\">{}</div>", before));
        }

        content.push_str(&format!("<div class=\"middle\">{}</div>", self.middle));

        if let Some(after) = &self.after {
            content.push_str(&format!("<div class=\"after\">{}</div>", after));
        }

        format!(
            "<div class=\"{}\">{}</div>",
            class_str, content
        )
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::new();
        assert!(!cell.is_ios());
        assert!(!cell.is_hovered());
        assert_eq!(cell.get_middle(), "");
    }

    #[test]
    fn test_cell_custom() {
        let cell = Cell::new()
            .ios(true)
            .hovered(true)
            .before("🔍")
            .after("➡")
            .middle("Search");

        assert!(cell.is_ios());
        assert!(cell.is_hovered());
        assert_eq!(cell.get_before(), Some("🔍"));
        assert_eq!(cell.get_after(), Some("➡"));
        assert_eq!(cell.get_middle(), "Search");
    }

    #[test]
    fn test_cell_render() {
        let cell = Cell::new().middle("Content");
        let html = cell.render();
        assert!(html.contains("telegram-ui-cell"));
        assert!(html.contains("<div class=\"middle\">Content</div>"));
    }

    #[test]
    fn test_cell_with_before_after() {
        let cell = Cell::new()
            .before("Icon")
            .after("Arrow")
            .middle("Text");

        let html = cell.render();
        assert!(html.contains("<div class=\"before\">Icon</div>"));
        assert!(html.contains("<div class=\"after\">Arrow</div>"));
    }

    #[test]
    fn test_cell_ios() {
        let cell = Cell::new().ios(true);
        let html = cell.render();
        assert!(html.contains("--ios"));
    }

    #[test]
    fn test_cell_hovered() {
        let cell = Cell::new().hovered(true);
        let html = cell.render();
        assert!(html.contains("--hovered"));
    }
}
