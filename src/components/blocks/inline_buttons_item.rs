// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `InlineButtonsItem` component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Inline buttons item mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InlineButtonsItemMode {
    /// Default plain mode (no background fill)
    #[default]
    Plain,
    /// Bezeled mode (3D bezeled style)
    Bezeled,
    /// Gray mode (gray background)
    Gray
}

impl InlineButtonsItemMode {
    /// Get CSS class name for this mode
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Bezeled => "telegram-ui-inline-buttons-item--bezeled",
            Self::Gray => "telegram-ui-inline-buttons-item--gray"
        }
    }
}

/// Inline buttons item component
#[derive(Debug, Clone)]
pub struct InlineButtonsItem {
    platform: String,
    mode:     InlineButtonsItemMode,
    class:    Option<String>,
    text:     Option<String>,
    children: Option<String>,
    active:   bool,
    disabled: bool
}

impl InlineButtonsItem {
    /// Create a new `InlineButtonsItem`
    #[must_use]
    pub fn new() -> Self {
        Self {
            platform: "base".to_string(),
            mode:     InlineButtonsItemMode::default(),
            class:    None,
            text:     None,
            children: None,
            active:   false,
            disabled: false
        }
    }

    /// Set platform (ios, android, base)
    #[must_use]
    pub fn platform(mut self, platform: &str) -> Self {
        self.platform = platform.to_string();
        self
    }

    /// Set the button mode
    #[must_use]
    pub const fn mode(mut self, mode: InlineButtonsItemMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set custom CSS class
    #[must_use]
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Set the button text
    #[must_use]
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    /// Set the button children (icon)
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Set whether the button is active
    #[must_use]
    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set whether the button is disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Render the inline buttons item as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-inline-buttons-item"];

        // Platform-specific class
        if self.platform == "ios" {
            classes.push("telegram-ui-inline-buttons-item--ios");
        }

        // Mode classes
        match self.mode {
            InlineButtonsItemMode::Bezeled => {
                classes.push("telegram-ui-inline-buttons-item--bezeled");
            }
            InlineButtonsItemMode::Gray => {
                classes.push("telegram-ui-inline-buttons-item--gray");
            }
            _ => {}
        }

        // Custom class
        if let Some(custom_class) = &self.class {
            classes.push(custom_class);
        }

        let class_str = classes.join(" ");

        // Build attributes
        let mut attrs = vec![];
        if self.active {
            attrs.push(" aria-pressed=\"true\"");
        }
        if self.disabled {
            attrs.push(" disabled");
        }

        let attr_str = attrs.join("");

        // Build content
        let mut content = String::new();
        if let Some(children) = &self.children {
            content.push_str(&format!(
                "<div class=\"telegram-ui-inline-buttons-item-children\">{}</div>",
                escape_html(children)
            ));
        }
        if let Some(text) = &self.text {
            content.push_str(&format!(
                "<span class=\"telegram-ui-inline-buttons-item-text\">{}</span>",
                escape_html(text)
            ));
        }

        format!(
            r#"<button class="{}"{}>{}</button>"#,
            escape_html(&class_str),
            attr_str,
            content
        )
    }
}

impl Default for InlineButtonsItem {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InlineButtonsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_buttons_item_default() {
        let item = InlineButtonsItem::new();
        assert_eq!(item.platform, "base");
        assert_eq!(item.mode, InlineButtonsItemMode::Plain);
        assert!(item.text.is_none());
        assert!(item.children.is_none());
    }

    #[test]
    fn test_inline_buttons_item_custom() {
        let item = InlineButtonsItem::new()
            .platform("ios")
            .mode(InlineButtonsItemMode::Bezeled)
            .class("custom")
            .text("Button")
            .children("🔍");

        assert_eq!(item.platform, "ios");
        assert_eq!(item.mode, InlineButtonsItemMode::Bezeled);
        assert_eq!(item.class, Some("custom".to_string()));
        assert_eq!(item.text, Some("Button".to_string()));
        assert_eq!(item.children, Some("🔍".to_string()));
    }

    #[test]
    fn test_inline_buttons_item_render() {
        let item = InlineButtonsItem::new().text("Button").children("🔍");

        let html = item.render();
        assert!(html.contains("telegram-ui-inline-buttons-item"));
        assert!(html.contains("Button"));
        assert!(html.contains("🔍"));
    }

    #[test]
    fn test_inline_buttons_item_modes() {
        let bezeled = InlineButtonsItem::new()
            .mode(InlineButtonsItemMode::Bezeled)
            .text("Bezeled");

        let gray = InlineButtonsItem::new()
            .mode(InlineButtonsItemMode::Gray)
            .text("Gray");

        assert!(
            bezeled
                .render()
                .contains("telegram-ui-inline-buttons-item--bezeled")
        );
        assert!(
            gray.render()
                .contains("telegram-ui-inline-buttons-item--gray")
        );
    }

    #[test]
    fn test_inline_buttons_item_ios() {
        let item = InlineButtonsItem::new().platform("ios").text("Button");

        let html = item.render();
        assert!(html.contains("telegram-ui-inline-buttons-item--ios"));
    }

    #[test]
    fn test_inline_buttons_item_escape_html() {
        let item = InlineButtonsItem::new().text("<script>alert(1)</script>");

        let html = item.render();
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }
}
