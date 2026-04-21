// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `AvatarStack` component for Telegram UI

use std::fmt;

/// `AvatarStack` component - simple container for avatar elements
#[derive(Debug, Clone)]
pub struct AvatarStack {
    children: String
}

impl AvatarStack {
    /// Creates a new `AvatarStack`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            children: String::new()
        }
    }

    /// Sets the children content
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Render the avatar stack as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-avatar-stack\">{}</div>",
            self.children
        )
    }
}

impl Default for AvatarStack {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AvatarStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_stack_default() {
        let stack = AvatarStack::new();
        assert!(stack.children.is_empty());
    }

    #[test]
    fn test_avatar_stack_with_children() {
        let stack = AvatarStack::new().children("<div>Avatar 1</div><div>Avatar 2</div>");

        let html = stack.render();
        assert!(html.contains("telegram-ui-avatar-stack"));
        assert!(html.contains("<div>Avatar 1</div><div>Avatar 2</div>"));
    }

    #[test]
    fn test_avatar_stack_display() {
        let stack = AvatarStack::new().children("<div>Test</div>");

        let html = format!("{stack}");
        assert!(html.contains("telegram-ui-avatar-stack"));
        assert!(html.contains("<div>Test</div>"));
    }
}
