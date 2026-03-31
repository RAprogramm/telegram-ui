// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! EmptyState component

use std::fmt;

#[derive(Clone, Debug)]
pub struct EmptyState {
    title: String,
    description: Option<String>,
}

impl EmptyState {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: None,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EmptyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyState: {}", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_default() {
        let empty_state = EmptyState::new();
        assert_eq!(empty_state.title(), "");
        assert!(empty_state.description().is_none());
    }
}
