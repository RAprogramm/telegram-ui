// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Modal component

use std::fmt;

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

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Modal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Modal: {}", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_default() {
        let modal = Modal::new();
        assert_eq!(modal.title(), "");
        assert_eq!(modal.content(), "");
        assert!(!modal.visible());
    }
}
