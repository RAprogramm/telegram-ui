// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Row component

#[derive(Clone, Debug)]
pub struct Row {
    children: Vec<String>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}
