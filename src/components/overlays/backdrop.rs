// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Backdrop component

use std::fmt;

use crate::helpers::escape_html;

/// Backdrop component
#[derive(Clone, Debug)]
pub struct Backdrop {
    visible:  bool,
    children: String
}

impl Backdrop {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            visible:  false,
            children: String::new()
        }
    }

    #[must_use]
    pub const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    #[must_use]
    pub fn render(&self) -> String {
        let class = if self.visible {
            "telegram-ui-backdrop telegram-ui-backdrop--visible"
        } else {
            "telegram-ui-backdrop"
        };

        format!(
            "<div class=\"{}\">{}</div>",
            class,
            escape_html(&self.children)
        )
    }
}

impl Default for Backdrop {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Backdrop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Backdrop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backdrop_default() {
        let backdrop = Backdrop::new();
        assert!(!backdrop.is_visible());
    }

    #[test]
    fn test_backdrop_builder() {
        let backdrop = Backdrop::new().visible(true).children("<div>Content</div>");

        assert!(backdrop.is_visible());
    }

    #[test]
    fn test_backdrop_render() {
        let backdrop = Backdrop::new().visible(true).children("<div>Test</div>");
        let html = backdrop.render();
        assert!(html.contains("telegram-ui-backdrop--visible"));
    }
}
