// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Modal component

#[derive(Clone, Debug)]
pub struct Modal {
    title: String,
    content: String,
    visible: bool,
}

impl Modal {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            visible: false,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
