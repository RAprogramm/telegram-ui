// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Link component

#[derive(Clone, Debug)]
pub struct Link {
    href: String,
    text: String,
}

impl Link {
    pub fn new() -> Self {
        Self {
            href: String::new(),
            text: String::new(),
        }
    }

    pub fn href(&self) -> &str {
        &self.href
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
