// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Section component for Telegram UI

use std::fmt;

mod footer;
mod header;

pub use footer::Footer;
pub use header::{Header, HeaderVariant};

/// Section component - a container with optional header and footer
#[derive(Debug, Clone)]
pub struct Section {
    header:  Option<Header>,
    footer:  Option<Footer>,
    content: String
}

impl Section {
    /// Create a new Section
    #[must_use]
    pub const fn new() -> Self {
        Self {
            header:  None,
            footer:  None,
            content: String::new()
        }
    }

    /// Add content to the section
    #[must_use]
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Add header to the section using Header builder
    #[must_use]
    pub fn header_builder(mut self, header: Header) -> Self {
        self.header = Some(header);
        self
    }

    /// Convenience method to add a simple header with text
    #[must_use]
    pub fn header(mut self, content: &str) -> Self {
        self.header = Some(Header::new().content(content));
        self
    }

    /// Add footer to the section using Footer builder
    #[must_use]
    pub fn footer_builder(mut self, footer: Footer) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Convenience method to add a simple footer with text
    #[must_use]
    pub fn footer(mut self, content: &str) -> Self {
        self.footer = Some(Footer::new().content(content));
        self
    }

    /// Render the section as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut html = String::from("<div class=\"telegram-ui-section\">");

        if let Some(ref header) = self.header {
            html.push_str(&header.render());
        }

        html.push_str(&format!(
            r#"<div class="section-content">{content}</div>"#,
            content = &self.content
        ));

        if let Some(ref footer) = self.footer {
            html.push_str(&footer.render());
        }

        html.push_str("</div>");
        html
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_default() {
        let section = Section::new();
        assert!(section.header.is_none());
        assert!(section.footer.is_none());
    }

    #[test]
    fn test_section_render() {
        let section = Section::new()
            .header("Section Title")
            .content("Section content here");

        let html = section.render();
        assert!(html.contains("Section Title"));
        assert!(html.contains("Section content here"));
    }

    #[test]
    fn test_section_with_footer() {
        let section = Section::new()
            .header("Title")
            .content("Content")
            .footer("Footer text");

        let html = section.render();
        assert!(html.contains("Title"));
        assert!(html.contains("Content"));
        assert!(html.contains("Footer text"));
    }

    #[test]
    fn test_section_with_header_builder() {
        let section = Section::new()
            .header_builder(Header::new().content("Title").variant(HeaderVariant::Small))
            .content("Content");

        let html = section.render();
        assert!(html.contains("section-header--small"));
    }

    #[test]
    fn test_section_with_footer_builder() {
        let section =
            Section::new().footer_builder(Footer::new().content("Footer").centered(true));

        let html = section.render();
        assert!(html.contains("section-footer--centered"));
    }
}
