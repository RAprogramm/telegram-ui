// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `AvatarAcronym` component for displaying text acronyms in avatars

use crate::components::typography::{Caption, Headline, LargeTitle, Title};

/// `AvatarAcronym` component that renders text with appropriate typography
/// based on size
#[derive(Debug, Clone)]
pub struct AvatarAcronym {
    text: String,
    size: u32
}

impl AvatarAcronym {
    /// Create a new `AvatarAcronym`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            size: 48
        }
    }

    /// Create with text
    #[must_use]
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Create with size
    #[must_use]
    pub const fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    /// Get text
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get size
    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }

    /// Render the component
    #[must_use]
    pub fn render(&self) -> String {
        if self.size == 0 {
            return String::new();
        }

        if self.size <= 28 {
            return Caption::new()
                .with_text(&self.text)
                .with_bold(true)
                .render();
        }

        if self.size == 40 {
            return Headline::new().with_text(&self.text).with_level(1).render();
        }

        if self.size == 48 {
            return Title::new()
                .with_text(&self.text)
                .with_align("center")
                .render();
        }

        LargeTitle::new().with_text(&self.text).render()
    }
}

impl Default for AvatarAcronym {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_acronym_default() {
        let acronym = AvatarAcronym::new();
        assert_eq!(acronym.text(), "");
        assert_eq!(acronym.size(), 48);
    }

    #[test]
    fn test_avatar_acronym_with_text() {
        let acronym = AvatarAcronym::new().with_text("JD");
        assert_eq!(acronym.text(), "JD");
    }

    #[test]
    fn test_avatar_acronym_with_size() {
        let acronym = AvatarAcronym::new().with_size(24);
        assert_eq!(acronym.size(), 24);
    }

    #[test]
    fn test_avatar_acronym_render_s24() {
        let acronym = AvatarAcronym::new().with_text("J").with_size(24);
        let html = acronym.render();
        assert!(html.contains("telegram-ui-caption"));
        assert!(html.contains("font-weight: bold"));
        assert!(html.contains('J'));
    }

    #[test]
    fn test_avatar_acronym_render_s28() {
        let acronym = AvatarAcronym::new().with_text("J").with_size(28);
        let html = acronym.render();
        assert!(html.contains("telegram-ui-caption"));
        assert!(html.contains("font-weight: bold"));
        assert!(html.contains('J'));
    }

    #[test]
    fn test_avatar_acronym_render_s40() {
        let acronym = AvatarAcronym::new().with_text("JD").with_size(40);
        let html = acronym.render();
        assert!(html.contains("telegram-ui-headline"));
        assert!(html.contains("JD"));
    }

    #[test]
    fn test_avatar_acronym_render_s48() {
        let acronym = AvatarAcronym::new().with_text("JD").with_size(48);
        let html = acronym.render();
        assert!(html.contains("telegram-ui-title"));
        assert!(html.contains("text-align: center"));
        assert!(html.contains("JD"));
    }

    #[test]
    fn test_avatar_acronym_render_l() {
        let acronym = AvatarAcronym::new().with_text("Telegram").with_size(64);
        let html = acronym.render();
        assert!(html.contains("telegram-ui-largetitle"));
        assert!(html.contains("Telegram"));
    }

    #[test]
    fn test_avatar_acronym_empty_size() {
        let acronym = AvatarAcronym::new().with_size(0);
        let html = acronym.render();
        assert_eq!(html, "");
    }
}
