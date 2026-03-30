// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Title component

#[derive(Clone, Debug)]
pub struct Title {
    text: String,
    align: String,
}

impl Title {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            align: "left".to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn align(&self) -> &str {
        &self.align
    }
}
