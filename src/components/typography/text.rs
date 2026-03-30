// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Text component

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
    color: String,
}

impl Text {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            color: "#000000".to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}
