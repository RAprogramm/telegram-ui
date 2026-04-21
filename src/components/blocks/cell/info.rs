// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Info component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Info component type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InfoType {
    /// Text content with optional subtitle
    #[default]
    Text,
    /// Avatar stack display
    AvatarStack
}

impl InfoType {
    /// Convert to CSS class suffix
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Text => "wrapper--text",
            Self::AvatarStack => "wrapper--avatarStack"
        }
    }
}

/// Info component - displays additional info text or avatar stack
#[derive(Debug, Clone)]
pub struct Info {
    info_type: InfoType,
    subtitle:  Option<String>,
    children:  Option<String>
}

impl Info {
    /// Creates a new Info with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            info_type: InfoType::Text,
            subtitle:  None,
            children:  None
        }
    }

    /// Sets the info type (text or avatar stack)
    #[must_use]
    pub const fn info_type(mut self, info_type: InfoType) -> Self {
        self.info_type = info_type;
        self
    }

    /// Sets the info type from string
    #[must_use]
    pub fn info_type_str(mut self, info_type: &str) -> Self {
        self.info_type = match info_type {
            "avatarStack" => InfoType::AvatarStack,
            _ => InfoType::Text
        };
        self
    }

    /// Sets the subtitle text
    #[must_use]
    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    /// Sets the children content
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Returns the info type
    #[must_use]
    pub const fn get_info_type(&self) -> &InfoType {
        &self.info_type
    }

    /// Returns the subtitle
    #[must_use]
    pub fn get_subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    /// Returns the children content
    #[must_use]
    pub fn get_children(&self) -> Option<&str> {
        self.children.as_deref()
    }

    /// Render the info as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["wrapper"];

        classes.push(self.info_type.css_class());

        let class_str = classes.join(" ");

        let mut content = String::new();

        if self.info_type == InfoType::AvatarStack {
            // Avatar stack would be rendered here
            // For now, leave empty as avatar stack is complex
        }

        if let Some(ref children) = self.children {
            content.push_str(&format!("<Text>{}</Text>", escape_html(children)));
        }

        if self.info_type == InfoType::Text
            && let Some(ref subtitle) = self.subtitle
        {
            content.push_str(&format!(
                r#"<Subheadline class="subtitle" level="2" plain="false">{}</Subheadline>"#,
                escape_html(subtitle)
            ));
        }

        format!("<div class=\"{class_str}\">{content}</div>")
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_default() {
        let info = Info::new();
        assert_eq!(info.get_info_type(), &InfoType::Text);
        assert_eq!(info.get_subtitle(), None);
        assert_eq!(info.get_children(), None);
    }

    #[test]
    fn test_info_text() {
        let info = Info::new()
            .info_type(InfoType::Text)
            .subtitle("Additional info")
            .children("Main text");

        assert_eq!(info.get_info_type(), &InfoType::Text);
        assert_eq!(info.get_subtitle(), Some("Additional info"));
        assert_eq!(info.get_children(), Some("Main text"));
    }

    #[test]
    fn test_info_render_text() {
        let info = Info::new().subtitle("Subtitle").children("Content");

        let html = info.render();
        assert!(html.contains("wrapper"));
        assert!(html.contains("wrapper--text"));
        assert!(html.contains("Subtitle"));
        assert!(html.contains("Content"));
    }

    #[test]
    fn test_info_render_avatar_stack() {
        let info = Info::new()
            .info_type(InfoType::AvatarStack)
            .children("AvatarStack content");

        let html = info.render();
        assert!(html.contains("wrapper"));
        assert!(html.contains("wrapper--avatarStack"));
    }

    #[test]
    fn test_info_escape_html() {
        let info = Info::new().subtitle("<script>alert('xss')</script>");
        let html = info.render();
        assert!(html.contains("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"));
    }
}
