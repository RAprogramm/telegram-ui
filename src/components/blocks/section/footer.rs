// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Section footer component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Section footer component
#[derive(Debug, Clone)]
pub struct Footer {
    content:  String,
    centered: bool
}

impl Footer {
    /// Create a new Footer
    #[must_use]
    pub const fn new() -> Self {
        Self {
            content:  String::new(),
            centered: false
        }
    }

    /// Set footer content
    #[must_use]
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Set centered alignment
    #[must_use]
    pub const fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Render the footer as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let centered_class = if self.centered {
            "section-footer--centered"
        } else {
            ""
        };

        format!(
            r#"<div class="section-footer {centered_class}">{content}</div>"#,
            centered_class = centered_class,
            content = escape_html(&self.content)
        )
    }
}

impl Default for Footer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Footer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_default() {
        let footer = Footer::new();
        assert!(footer.content.is_empty());
        assert!(!footer.centered);
    }

    #[test]
    fn test_footer_render() {
        let footer = Footer::new().content("Footer text");

        let html = footer.render();
        assert!(html.contains("Footer text"));
        assert!(html.contains("section-footer"));
    }

    #[test]
    fn test_footer_with_centered() {
        let footer = Footer::new().content("Text").centered(true);

        let html = footer.render();
        assert!(html.contains("section-footer--centered"));
    }

    #[test]
    fn test_footer_html_escape() {
        let footer = Footer::new().content("<script>alert('xss')</script>");

        let html = footer.render();
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }
}
