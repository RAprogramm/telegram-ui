// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Button component for navigation

use std::fmt;

#[derive(Clone, Debug)]
pub struct Button {
    text:   String,
    active: bool
}

impl Button {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text:   String::new(),
            active: false
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Button: {}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_default() {
        let button = Button::new();
        assert_eq!(button.text(), "");
        assert!(!button.active());
    }
}
