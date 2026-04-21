// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Header component for Telegram UI

use std::fmt;

#[derive(Debug, Clone)]
pub enum HeaderVariant {
    Default,
    Small,
}

impl Default for HeaderVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, Default)]
pub struct Header {
    content: String,
    variant: HeaderVariant,
}

impl Header {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            variant: HeaderVariant::Default,
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn variant(mut self, variant: HeaderVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn render(&self) -> String {
        let mut classes = String::from("section-header");
        match self.variant {
            HeaderVariant::Small => classes.push_str(" section-header--small"),
            HeaderVariant::Default => {}
        }

        format!(
            r#"<div class="{classes}">{content}</div>"#,
            classes = classes,
            content = &self.content
        )
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
