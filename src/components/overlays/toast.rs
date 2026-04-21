// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Toast component

use std::fmt;

use crate::helpers::escape_html;

/// Toast component
#[derive(Clone, Debug)]
pub struct Toast {
    message:  String,
    duration: u64,
    action:   String
}

impl Toast {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            message:  String::new(),
            duration: 3000,
            action:   String::new()
        }
    }

    #[must_use]
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    #[must_use]
    pub const fn duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }

    #[must_use]
    pub fn action(mut self, action: &str) -> Self {
        self.action = action.to_string();
        self
    }

    #[must_use]
    pub fn message_text(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn duration_time(&self) -> u64 {
        self.duration
    }

    #[must_use]
    pub fn action_text(&self) -> &str {
        &self.action
    }

    #[must_use]
    pub fn render(&self) -> String {
        let _action_html = if self.action.is_empty() {
            String::new()
        } else {
            format!("<button>{}</button>", escape_html(&self.action))
        };

        format!(
            "<div class=\"telegram-ui-toast\" style=\"--toast-duration: {}ms;\">{}</div>",
            self.duration,
            escape_html(&self.message)
        )
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Toast")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_default() {
        let toast = Toast::new();
        assert!(toast.message_text().is_empty());
        assert_eq!(toast.duration_time(), 3000);
    }

    #[test]
    fn test_toast_builder() {
        let toast = Toast::new().message("Hello").duration(5000).action("Undo");

        assert_eq!(toast.message_text(), "Hello");
        assert_eq!(toast.duration_time(), 5000);
        assert_eq!(toast.action_text(), "Undo");
    }

    #[test]
    fn test_toast_render() {
        let toast = Toast::new().message("Test");
        let html = toast.render();
        assert!(html.contains("telegram-ui-toast"));
    }
}
