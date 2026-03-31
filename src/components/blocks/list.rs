// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! List component for Telegram UI

use std::fmt;

/// List component
#[derive(Debug, Clone)]
pub struct List {
    ios: bool,
    cells: Vec<String>,
}

impl List {
    /// Creates a new List with default settings
    pub fn new() -> Self {
        Self {
            ios: false,
            cells: Vec::new(),
        }
    }

    /// Sets whether the list should use iOS styling
    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    /// Adds a cell to the list
    pub fn add_cell(mut self, cell: &str) -> Self {
        self.cells.push(cell.to_string());
        self
    }

    /// Adds multiple cells to the list
    pub fn add_cells(mut self, cells: &[&str]) -> Self {
        for cell in cells {
            self.cells.push(cell.to_string());
        }
        self
    }

    /// Returns whether the list uses iOS styling
    pub fn is_ios(&self) -> bool {
        self.ios
    }

    /// Returns the number of cells in the list
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Returns whether the list is empty
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Returns the cells in the list
    pub fn get_cells(&self) -> &[String] {
        &self.cells
    }

    /// Render the list as HTML string
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-list"];

        if self.ios {
            classes.push("--ios");
        }

        let class_str = classes.join(" ");

        let cells_html = self.cells.join("");

        format!(
            "<div class=\"{}\">{}</div>",
            class_str, cells_html
        )
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_default() {
        let list = List::new();
        assert!(!list.is_ios());
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_with_cells() {
        let list = List::new()
            .add_cell("<div>Cell 1</div>")
            .add_cell("<div>Cell 2</div>");

        assert_eq!(list.len(), 2);
        assert_eq!(list.get_cells()[0], "<div>Cell 1</div>");
        assert_eq!(list.get_cells()[1], "<div>Cell 2</div>");
    }

    #[test]
    fn test_list_add_cells() {
        let list = List::new().add_cells(&["Cell 1", "Cell 2", "Cell 3"]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_list_render() {
        let list = List::new().add_cell("<div>Content</div>");
        let html = list.render();
        assert!(html.contains("telegram-ui-list"));
        assert!(html.contains("<div>Content</div>"));
    }

    #[test]
    fn test_list_ios() {
        let list = List::new().ios(true);
        let html = list.render();
        assert!(html.contains("--ios"));
    }
}
