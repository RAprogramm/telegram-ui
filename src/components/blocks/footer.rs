// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Footer component for Telegram UI

use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct Footer {
    content: String,
    centered: bool,
}

impl Footer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            centered: false,
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    pub fn render(&self) -> String {
        let mut classes = String::from("section-footer");
        if self.centered {
            classes.push_str(" section-footer--centered");
        }

        format!(
            r#"<div class="{classes}">{content}</div>"#,
            classes = classes,
            content = &self.content
        )
    }
}

impl fmt::Display for Footer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
