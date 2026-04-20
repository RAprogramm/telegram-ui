// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! AvatarStack component for Telegram UI

use std::fmt;

/// AvatarStack component - displays overlapping avatars
#[derive(Debug, Clone)]
pub struct AvatarStack {
    avatars:        Vec<AvatarItem>,
    max_display:    usize,
    #[expect(dead_code)]
    overflow_count: Option<usize>
}

#[derive(Debug, Clone)]
struct AvatarItem {
    src:      Option<String>,
    initials: String
}

impl AvatarStack {
    /// Create a new AvatarStack
    pub fn new() -> Self {
        Self {
            avatars:        Vec::new(),
            max_display:    5,
            overflow_count: None
        }
    }

    /// Add an avatar (by initials)
    pub fn add_avatar(mut self, initials: &str) -> Self {
        self.avatars.push(AvatarItem {
            src:      None,
            initials: initials.to_string()
        });
        self
    }

    /// Add an avatar with image
    pub fn add_avatar_with_image(mut self, initials: &str, src: &str) -> Self {
        self.avatars.push(AvatarItem {
            src:      Some(src.to_string()),
            initials: initials.to_string()
        });
        self
    }

    /// Set maximum avatars to display
    pub fn max_display(mut self, max: usize) -> Self {
        self.max_display = max;
        self
    }

    /// Render the avatar stack as HTML string
    pub fn render(&self) -> String {
        let display_count = self.max_display.min(self.avatars.len());
        let overflow = self.avatars.len().saturating_sub(self.max_display);

        let mut html = String::from("<div class=\"telegram-ui-avatar-stack\">");

        for (i, avatar) in self.avatars.iter().take(display_count).enumerate() {
            let offset = i * 10;

            if let Some(ref src) = avatar.src {
                html.push_str(&format!(
                    r#"<div class="avatar-stack-item" style="margin-left: -{offset}px; z-index: {z};">
                        <img src="{src}" alt="{initials}">
                    </div>"#,
                    offset = offset,
                    z = display_count - i,
                    src = src,
                    initials = &avatar.initials
                ));
            } else {
                html.push_str(&format!(
                    r#"<div class="avatar-stack-item" style="margin-left: -{offset}px; z-index: {z};">
                        <div class="avatar-initials">{initials}</div>
                    </div>"#,
                    offset = offset,
                    z = display_count - i,
                    initials = &avatar.initials
                ));
            }
        }

        if overflow > 0 {
            html.push_str(&format!(
                r#"<div class="avatar-stack-overflow" style="margin-left: -{offset}px; z-index: 0;">
                    +{overflow}
                </div>"#,
                offset = display_count * 10
            ));
        }

        html.push_str("</div>");
        html
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
        assert!(stack.avatars.is_empty());
    }

    #[test]
    fn test_avatar_stack_render() {
        let stack = AvatarStack::new()
            .add_avatar("JD")
            .add_avatar("AS")
            .max_display(3);

        let html = stack.render();
        assert!(html.contains("avatar-stack"));
        assert!(html.contains("JD"));
        assert!(html.contains("AS"));
    }
}
