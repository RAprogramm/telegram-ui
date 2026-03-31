// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Column component

#[derive(Clone, Debug)]
pub struct Column {
    children: Vec<String>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}
