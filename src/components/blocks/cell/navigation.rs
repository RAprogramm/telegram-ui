// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Navigation component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Navigation component - displays navigation indicator (chevron/right arrow)
#[derive(Debug, Clone)]
pub struct Navigation {
    children: Option<String>
}

impl Navigation {
    /// Creates a new Navigation with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            children: None
        }
    }

    /// Sets the children content (optional navigation text)
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Returns the children content
    #[must_use]
    pub fn get_children(&self) -> Option<&str> {
        self.children.as_deref()
    }

    /// Render the navigation as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut content = String::new();
        let has_children = self.children.is_some();

        if has_children && let Some(ref children) = self.children {
            content.push_str(&format!(
                "<Text class=\"text\">{}</Text>",
                escape_html(children)
            ));
        }

        // Always show chevron if no children or on iOS platform
        // For simplicity, we show chevron when there are no children or show both
        if !has_children {
            content.push_str(r#"<Icon16Chevron class="icon" />"#);
        }

        format!("<div class=\"wrapper\">{content}</div>")
    }
}

impl Default for Navigation {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Navigation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_default() {
        let nav = Navigation::new();
        assert_eq!(nav.get_children(), None);
    }

    #[test]
    fn test_navigation_with_text() {
        let nav = Navigation::new().children("Next Page");
        assert_eq!(nav.get_children(), Some("Next Page"));
    }

    #[test]
    fn test_navigation_render_no_children() {
        let nav = Navigation::new();
        let html = nav.render();
        assert!(html.contains("wrapper"));
        assert!(html.contains("Icon16Chevron"));
    }

    #[test]
    fn test_navigation_render_with_children() {
        let nav = Navigation::new().children("Next");
        let html = nav.render();
        assert!(html.contains("wrapper"));
        assert!(html.contains("Next"));
    }

    #[test]
    fn test_navigation_escape_html() {
        let nav = Navigation::new().children("<script>alert(1)</script>");
        let html = nav.render();
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }
}
