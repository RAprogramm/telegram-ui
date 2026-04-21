// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Section header component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Section header variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HeaderVariant {
    #[default]
    /// Large header
    Large,
    /// Medium header
    Medium,
    /// Small header
    Small
}

/// Section header component
#[derive(Debug, Clone)]
pub struct Header {
    content: String,
    variant: HeaderVariant
}

impl Header {
    /// Create a new Header
    #[must_use]
    pub const fn new() -> Self {
        Self {
            content: String::new(),
            variant: HeaderVariant::Large
        }
    }

    /// Set header content
    #[must_use]
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Set header variant
    #[must_use]
    pub const fn variant(mut self, variant: HeaderVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Render the header as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let class = match self.variant {
            HeaderVariant::Large => "section-header section-header--large",
            HeaderVariant::Medium => "section-header section-header--medium",
            HeaderVariant::Small => "section-header section-header--small"
        };

        format!(
            r#"<div class="{class}">{content}</div>"#,
            class = class,
            content = escape_html(&self.content)
        )
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_default() {
        let header = Header::new();
        assert!(header.content.is_empty());
        assert_eq!(header.variant, HeaderVariant::Large);
    }

    #[test]
    fn test_header_render() {
        let header = Header::new().content("Section Title");

        let html = header.render();
        assert!(html.contains("Section Title"));
        assert!(html.contains("section-header--large"));
    }

    #[test]
    fn test_header_with_variant() {
        let header = Header::new().content("Title").variant(HeaderVariant::Small);

        let html = header.render();
        assert!(html.contains("section-header--small"));
    }

    #[test]
    fn test_header_html_escape() {
        let header = Header::new().content("<script>alert('xss')</script>");

        let html = header.render();
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }
}
