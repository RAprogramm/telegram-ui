// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Avatar component for displaying user avatars

#[derive(Debug, Clone)]
pub struct Avatar {
    src: Option<String>,
    initials: Option<String>,
    size: String,
    bg_color: Option<String>,
    text_color: Option<String>,
}

impl Avatar {
    /// Creates a new Avatar with default settings
    pub fn new() -> Self {
        Self {
            src: None,
            initials: None,
            size: "48px".to_string(),
            bg_color: None,
            text_color: None,
        }
    }

    /// Sets the image source
    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    /// Sets the initials (shown when no image)
    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into().to_uppercase());
        self
    }

    /// Sets the avatar size
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = size.into();
        self
    }

    /// Sets the background color (for initials mode)
    pub fn bg_color(mut self, color: impl Into<String>) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    /// Sets the text color (for initials mode)
    pub fn text_color(mut self, color: impl Into<String>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    /// Renders the avatar to HTML
    pub fn render(&self) -> String {
        let mut html = String::from("<div class=\"telegram-ui-avatar\"");

        // Size style
        html.push_str(&format!(
            " style=\"width: {}; height: {}; ",
            self.size, self.size
        ));

        // Background color
        if let Some(ref bg) = self.bg_color {
            html.push_str(&format!("background-color: {}; ", bg));
        } else {
            html.push_str("background-color: var(--tg-theme-button-color, #2481cc); ");
        }

        // Text color
        if let Some(ref text) = self.text_color {
            html.push_str(&format!("color: {}; ", text));
        } else {
            html.push_str("color: var(--tg-theme-button-text-color, #ffffff); ");
        }

        html.push_str(&format!(
            "border-radius: 50%; display: flex; align-items: center; justify-content: center; \
            font-weight: 600; font-size: calc({} * 0.5); overflow: hidden;\"",
            self.size
        ));

        if let Some(ref src) = self.src {
            html.push_str(&format!(
                "<img src=\"{}\" alt=\"Avatar\" style=\"width: 100%; height: 100%; object-fit: cover;\"/>",
                src
            ));
        } else if let Some(ref initials) = self.initials {
            html.push_str(initials);
        }

        html.push_str("</div>");
        html
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_default() {
        let avatar = Avatar::new();
        assert_eq!(avatar.size, "48px");
        assert!(avatar.src.is_none());
        assert!(avatar.initials.is_none());
    }

    #[test]
    fn test_avatar_with_image() {
        let avatar = Avatar::new().src("https://example.com/avatar.jpg");
        let html = avatar.render();
        assert!(html.contains("https://example.com/avatar.jpg"));
        assert!(html.contains("<img"));
    }

    #[test]
    fn test_avatar_with_initials() {
        let avatar = Avatar::new().initials("JD");
        let html = avatar.render();
        assert!(html.contains("JD"));
        assert!(!html.contains("<img"));
    }

    #[test]
    fn test_avatar_custom_size() {
        let avatar = Avatar::new().size("64px");
        let html = avatar.render();
        assert!(html.contains("width: 64px"));
        assert!(html.contains("height: 64px"));
    }

    #[test]
    fn test_avatar_custom_colors() {
        let avatar = Avatar::new()
            .initials("A")
            .bg_color("#ff5722")
            .text_color("#ffffff");
        let html = avatar.render();
        assert!(html.contains("#ff5722"));
        assert!(html.contains("#ffffff"));
    }
}
