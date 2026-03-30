// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Caption component

#[derive(Clone, Debug)]
pub struct Caption {
    text: String,
    bold: bool,
}

impl Caption {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            bold: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn bold(&self) -> bool {
        self.bold
    }
}
