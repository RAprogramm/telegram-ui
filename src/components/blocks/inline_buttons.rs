// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `InlineButtons` component for Telegram UI

use std::fmt;

use crate::components::blocks::inline_buttons_item::{InlineButtonsItem, InlineButtonsItemMode};

/// Inline buttons component
#[derive(Debug, Clone)]
pub struct InlineButtons {
    mode:  InlineButtonsItemMode,
    ios:   bool,
    items: Vec<InlineButtonsItem>
}

impl InlineButtons {
    /// Create a new `InlineButtons`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode:  InlineButtonsItemMode::Plain,
            ios:   false,
            items: Vec::new()
        }
    }

    /// Set the default mode for items
    #[must_use]
    pub const fn mode(mut self, mode: InlineButtonsItemMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set whether the buttons should use iOS styling
    #[must_use]
    pub const fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    /// Add an item to the inline buttons
    #[must_use]
    pub fn add_item(mut self, item: InlineButtonsItem) -> Self {
        self.items.push(item);
        self
    }

    /// Render the inline buttons as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let ios_class = if self.ios {
            "telegram-ui-inline-buttons--ios"
        } else {
            ""
        };

        let items_html = self
            .items
            .iter()
            .map(|item| {
                let mut item_rendered = item.clone();
                if self.ios {
                    item_rendered = item_rendered.platform("ios");
                }
                item_rendered.render()
            })
            .collect::<String>();

        format!(
            r#"<div class="telegram-ui-inline-buttons {ios_class}">
                {items_html}
            </div>"#
        )
    }
}

impl Default for InlineButtons {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InlineButtons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_buttons_default() {
        let buttons = InlineButtons::new();
        assert_eq!(buttons.mode, InlineButtonsItemMode::Plain);
        assert!(!buttons.ios);
        assert!(buttons.items.is_empty());
    }

    #[test]
    fn test_inline_buttons_custom() {
        let buttons = InlineButtons::new()
            .mode(InlineButtonsItemMode::Gray)
            .ios(true)
            .add_item(InlineButtonsItem::new().text("Button 1"))
            .add_item(InlineButtonsItem::new().text("Button 2"));

        assert_eq!(buttons.mode, InlineButtonsItemMode::Gray);
        assert!(buttons.ios);
        assert_eq!(buttons.items.len(), 2);
    }

    #[test]
    fn test_inline_buttons_render() {
        let buttons = InlineButtons::new()
            .add_item(InlineButtonsItem::new().text("Button 1"))
            .add_item(InlineButtonsItem::new().text("Button 2"));

        let html = buttons.render();
        assert!(html.contains("telegram-ui-inline-buttons"));
        assert!(html.contains("Button 1"));
        assert!(html.contains("Button 2"));
    }

    #[test]
    fn test_inline_buttons_ios() {
        let buttons = InlineButtons::new()
            .ios(true)
            .add_item(InlineButtonsItem::new().text("Button"));

        let html = buttons.render();
        assert!(html.contains("telegram-ui-inline-buttons--ios"));
    }

    #[test]
    fn test_inline_buttons_mode() {
        let buttons = InlineButtons::new()
            .mode(InlineButtonsItemMode::Bezeled)
            .add_item(
                InlineButtonsItem::new()
                    .mode(InlineButtonsItemMode::Bezeled)
                    .text("Button")
            );

        let html = buttons.render();
        assert!(html.contains("telegram-ui-inline-buttons-item--bezeled"));
    }
}
