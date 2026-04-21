// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! `ButtonCell` component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// `ButtonCell` mode variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonCellMode {
    /// Default button cell style
    #[default]
    Default,
    /// Destructive button cell style (red/attention-grabbing)
    Destructive
}

impl ButtonCellMode {
    /// Convert to CSS class suffix
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Destructive => "wrapper--destructive"
        }
    }
}

/// `ButtonCell` component - a cell that acts as a button
#[derive(Debug, Clone)]
pub struct ButtonCell {
    mode:     ButtonCellMode,
    ios:      bool,
    before:   Option<String>,
    after:    Option<String>,
    children: Option<String>
}

impl ButtonCell {
    /// Creates a new `ButtonCell` with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode:     ButtonCellMode::Default,
            ios:      false,
            before:   None,
            after:    None,
            children: None
        }
    }

    /// Sets the button cell mode
    #[must_use]
    pub const fn mode(mut self, mode: ButtonCellMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the button cell mode from string
    #[must_use]
    pub fn mode_str(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "destructive" => ButtonCellMode::Destructive,
            _ => ButtonCellMode::Default
        };
        self
    }

    /// Sets whether the cell should use iOS styling
    #[must_use]
    pub const fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    /// Sets content to show before the button cell content
    #[must_use]
    pub fn before(mut self, content: &str) -> Self {
        self.before = Some(content.to_string());
        self
    }

    /// Sets content to show after the button cell content
    #[must_use]
    pub fn after(mut self, content: &str) -> Self {
        self.after = Some(content.to_string());
        self
    }

    /// Sets the children content (button text)
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Returns the button cell mode
    #[must_use]
    pub const fn get_mode(&self) -> &ButtonCellMode {
        &self.mode
    }

    /// Returns whether the cell uses iOS styling
    #[must_use]
    pub const fn is_ios(&self) -> bool {
        self.ios
    }

    /// Returns the button cell before content
    #[must_use]
    pub fn get_before(&self) -> Option<&str> {
        self.before.as_deref()
    }

    /// Returns the button cell after content
    #[must_use]
    pub fn get_after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    /// Returns the button cell children content
    #[must_use]
    pub fn get_children(&self) -> Option<&str> {
        self.children.as_deref()
    }

    /// Render the button cell as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["wrapper"];

        if !self.mode.css_class().is_empty() {
            classes.push(self.mode.css_class());
        }

        if self.ios {
            classes.push("wrapper--ios");
        }

        let class_str = classes.join(" ");

        let mut content = String::new();

        if let Some(ref before) = self.before {
            content.push_str(&format!(
                "<div class=\"before\">{}</div>",
                escape_html(before)
            ));
        }

        if let Some(ref children) = self.children {
            let typography_tag = if self.ios { "Text" } else { "Subheadline" };
            content.push_str(&format!(
                "<{} class=\"content\">{}</{}>",
                typography_tag,
                escape_html(children),
                typography_tag
            ));
        }

        if let Some(ref after) = self.after {
            content.push_str(&format!(
                "<div class=\"after\">{}</div>",
                escape_html(after)
            ));
        }

        format!("<button class=\"{class_str}\">{content}</button>")
    }
}

impl Default for ButtonCell {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ButtonCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_cell_default() {
        let button_cell = ButtonCell::new();
        assert_eq!(button_cell.get_mode(), &ButtonCellMode::Default);
        assert!(!button_cell.is_ios());
        assert_eq!(button_cell.get_children(), None);
    }

    #[test]
    fn test_button_cell_custom() {
        let button_cell = ButtonCell::new()
            .mode(ButtonCellMode::Destructive)
            .ios(true)
            .before("🔍")
            .after("➡")
            .children("Search");

        assert_eq!(button_cell.get_mode(), &ButtonCellMode::Destructive);
        assert!(button_cell.is_ios());
        assert_eq!(button_cell.get_before(), Some("🔍"));
        assert_eq!(button_cell.get_after(), Some("➡"));
        assert_eq!(button_cell.get_children(), Some("Search"));
    }

    #[test]
    fn test_button_cell_render() {
        let button_cell = ButtonCell::new().children("Submit");
        let html = button_cell.render();
        assert!(html.contains("wrapper"));
        assert!(html.contains("Submit"));
    }

    #[test]
    fn test_button_cell_render_ios() {
        let button_cell = ButtonCell::new().ios(true).children("Text");
        let html = button_cell.render();
        assert!(html.contains("wrapper--ios"));
        assert!(html.contains("<Text"));
    }

    #[test]
    fn test_button_cell_render_ddestructive() {
        let button_cell = ButtonCell::new().mode(ButtonCellMode::Destructive);
        let html = button_cell.render();
        assert!(html.contains("wrapper--destructive"));
    }

    #[test]
    fn test_button_cell_with_before_after() {
        let button_cell = ButtonCell::new()
            .before("Icon")
            .after("Arrow")
            .children("Text");

        let html = button_cell.render();
        assert!(html.contains("<div class=\"before\">Icon</div>"));
        assert!(html.contains("<div class=\"after\">Arrow</div>"));
    }

    #[test]
    fn test_button_cell_escape_html() {
        let button_cell = ButtonCell::new().children("<script>alert(1)</script>");
        let html = button_cell.render();
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }
}
