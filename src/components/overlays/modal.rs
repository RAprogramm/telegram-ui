// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Modal component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Modal component
#[derive(Clone, Debug)]
pub struct Modal {
    header:    String,
    footer:    String,
    content:   String,
    visible:   bool,
    has_close: bool
}

impl Modal {
    /// Creates a new Modal instance
    pub fn new() -> Self {
        Self {
            header:    String::new(),
            footer:    String::new(),
            content:   String::new(),
            visible:   true,
            has_close: true
        }
    }

    /// Sets the modal header
    pub fn header(mut self, header: &str) -> Self {
        self.header = header.to_string();
        self
    }

    /// Sets the modal footer
    pub fn footer(mut self, footer: &str) -> Self {
        self.footer = footer.to_string();
        self
    }

    /// Sets the modal content
    pub fn children(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    /// Sets whether the modal is visible
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets whether to show the close button
    pub fn close_button(mut self, close: bool) -> Self {
        self.has_close = close;
        self
    }

    /// Get the header
    pub fn header_text(&self) -> &str {
        &self.header
    }

    /// Get the footer
    pub fn footer_text(&self) -> &str {
        &self.footer
    }

    /// Get the content
    pub fn content_text(&self) -> &str {
        &self.content
    }

    /// Render the modal as HTML string
    pub fn render(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"telegram-ui-modal");
        if !self.visible {
            html.push_str(" telegram-ui-modal--hidden");
        }
        html.push_str("\">\n");

        // Close button
        if self.has_close {
            html.push_str("<div class=\"telegram-ui-modal__close\"></div>\n");
        }

        // Header
        if !self.header.is_empty() {
            html.push_str("<div class=\"telegram-ui-modal__header\">\n");
            html.push_str(&escape_html(&self.header));
            html.push_str("</div>\n");
        }

        // Content
        if !self.content.is_empty() {
            html.push_str("<div class=\"telegram-ui-modal__content\">\n");
            html.push_str(&self.content);
            html.push_str("</div>\n");
        }

        // Footer
        if !self.footer.is_empty() {
            html.push_str("<div class=\"telegram-ui-modal__footer\">\n");
            html.push_str(&self.footer);
            html.push_str("</div>\n");
        }

        html.push_str("</div>");

        html
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Modal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_default() {
        let modal = Modal::new();
        assert!(modal.visible);
        assert!(modal.header_text().is_empty());
        assert!(modal.content_text().is_empty());
        assert!(modal.has_close);
    }

    #[test]
    fn test_modal_builder() {
        let modal = Modal::new()
            .header("Title")
            .footer("Footer")
            .children("<p>Content</p>")
            .visible(false);

        assert_eq!(modal.header_text(), "Title");
        assert_eq!(modal.footer_text(), "Footer");
        assert_eq!(modal.content_text(), "<p>Content</p>");
        assert!(!modal.visible);
    }

    #[test]
    fn test_modal_render() {
        let modal = Modal::new()
            .header("Test Modal")
            .children("<p>This is content</p>")
            .footer("<button>OK</button>");

        let html = modal.render();
        assert!(html.contains("telegram-ui-modal"));
        assert!(html.contains("Test Modal"));
        assert!(html.contains("<p>This is content</p>"));
        assert!(html.contains("<button>OK</button>"));
    }
}
