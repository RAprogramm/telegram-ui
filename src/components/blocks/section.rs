// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Section component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Section header variant
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SectionHeader {
    #[default]
    /// Large header
    Large,
    /// Medium header
    Medium,
    /// Small header
    Small,
}

/// Section component - a container with optional header
#[derive(Debug, Clone)]
pub struct Section {
    header: Option<String>,
    header_var: SectionHeader,
    footer: Option<String>,
    content: String,
}

impl Section {
    /// Create a new Section
    pub fn new() -> Self {
        Self {
            header: None,
            header_var: SectionHeader::Large,
            footer: None,
            content: String::new(),
        }
    }

    /// Set the section header
    pub fn header(mut self, header: &str) -> Self {
        self.header = Some(header.to_string());
        self
    }

    /// Set header variant
    pub fn header_var(mut self, header_var: SectionHeader) -> Self {
        self.header_var = header_var;
        self
    }

    /// Set the section footer
    pub fn footer(mut self, footer: &str) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    /// Add content to the section
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Render the section as HTML string
    pub fn render(&self) -> String {
        let header_class = match self.header_var {
            SectionHeader::Large => "section-header--large",
            SectionHeader::Medium => "section-header--medium",
            SectionHeader::Small => "section-header--small",
        };

        let mut html = String::from("<div class=\"telegram-ui-section\">");

        if let Some(ref header) = self.header {
            html.push_str(&format!(
                r#"<div class="section-header {header_class}">{header}</div>"#,
                header_class = header_class,
                header = escape_html(header)
            ));
        }

        html.push_str(&format!(
            r#"<div class="section-content">{content}</div>"#,
            content = &self.content
        ));

        if let Some(ref footer) = self.footer {
            html.push_str(&format!(
                r#"<div class="section-footer">{footer}</div>"#,
                footer = escape_html(footer)
            ));
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
}
