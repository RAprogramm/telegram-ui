// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Empty state component

#[derive(Clone, Debug)]
pub struct EmptyState {
    title: String,
    description: String,
    action: Option<String>,
}

impl EmptyState {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            action: None,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }
}
