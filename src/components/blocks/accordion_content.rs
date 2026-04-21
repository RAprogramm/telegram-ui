// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `AccordionContent` component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Accordion content component
#[derive(Debug, Clone)]
pub struct AccordionContent {
    id:              Option<String>,
    aria_labelledby: Option<String>,
    aria_hidden:     Option<bool>,
    class:           Option<String>,
    children:        String
}

impl AccordionContent {
    /// Create a new `AccordionContent`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id:              None,
            aria_labelledby: None,
            aria_hidden:     None,
            class:           None,
            children:        String::new()
        }
    }

    /// Set the element id
    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Set aria-labelledby attribute
    #[must_use]
    pub fn aria_labelledby(mut self, labelledby: &str) -> Self {
        self.aria_labelledby = Some(labelledby.to_string());
        self
    }

    /// Set aria-hidden attribute
    #[must_use]
    pub const fn aria_hidden(mut self, hidden: bool) -> Self {
        self.aria_hidden = Some(hidden);
        self
    }

    /// Set custom CSS class
    #[must_use]
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Set the children content
    #[must_use]
    pub fn children(mut self, content: &str) -> Self {
        self.children = content.to_string();
        self
    }

    /// Render the accordion content as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-accordion-content"];

        if let Some(custom_class) = &self.class {
            classes.push(custom_class);
        }

        let class_str = classes.join(" ");

        let mut attrs = vec![];
        if let Some(id) = &self.id {
            attrs.push(format!(" id=\"{}\"", escape_html(id)));
        }
        if let Some(labelledby) = &self.aria_labelledby {
            attrs.push(format!(" aria-labelledby=\"{}\"", escape_html(labelledby)));
        }
        if let Some(hidden) = self.aria_hidden {
            attrs.push(format!(" aria-hidden=\"{hidden}\""));
        }

        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        };

        format!(
            r#"<div class="{}"{}>
                <div class="telegram-ui-accordion-content-body">
                    {}
                </div>
            </div>"#,
            escape_html(&class_str),
            attr_str,
            escape_html(&self.children)
        )
    }
}

impl Default for AccordionContent {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AccordionContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_content_default() {
        let content = AccordionContent::new();
        assert!(content.id.is_none());
        assert!(content.aria_labelledby.is_none());
        assert!(content.aria_hidden.is_none());
        assert!(content.children.is_empty());
    }

    #[test]
    fn test_accordion_content_custom() {
        let content = AccordionContent::new()
            .id("content-1")
            .aria_labelledby("summary-1")
            .aria_hidden(false)
            .class("custom-class")
            .children("Content text");

        assert_eq!(content.id, Some("content-1".to_string()));
        assert_eq!(content.aria_labelledby, Some("summary-1".to_string()));
        assert_eq!(content.aria_hidden, Some(false));
        assert_eq!(content.class, Some("custom-class".to_string()));
        assert_eq!(content.children, "Content text");
    }

    #[test]
    fn test_accordion_content_render() {
        let content = AccordionContent::new()
            .id("content-1")
            .aria_labelledby("summary-1")
            .aria_hidden(true)
            .children("Hidden content");

        let html = content.render();
        assert!(html.contains("content-1"));
        assert!(html.contains("summary-1"));
        assert!(html.contains("aria-hidden=\"true\""));
        assert!(html.contains("Hidden content"));
    }

    #[test]
    fn test_accordion_content_escape_html() {
        let content = AccordionContent::new()
            .id("<script>")
            .children("<script>alert(1)</script>");

        let html = content.render();
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }
}
