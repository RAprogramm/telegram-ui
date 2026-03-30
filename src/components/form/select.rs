// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Select component

#[derive(Clone, Debug)]
pub struct Select {
    options: Vec<String>,
    value: Option<String>,
}

impl Select {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            value: None,
        }
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}
