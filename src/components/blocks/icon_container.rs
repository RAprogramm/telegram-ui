// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Icon Container component for Telegram UI

use std::fmt;

/// Icon container component
#[derive(Debug, Clone)]
pub struct IconContainer {
    icon:       String,
    size:       Option<String>,
    class_name: Option<String>
}

impl IconContainer {
    /// Creates a new `IconContainer`
    #[must_use]
    pub const fn new() -> Self {
        Self {
            icon:       String::new(),
            size:       None,
            class_name: None
        }
    }

    /// Sets the icon content (required)
    #[must_use]
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }

    /// Sets the size (optional)
    #[must_use]
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the class name (optional)
    #[must_use]
    pub fn class_name(mut self, class_name: &str) -> Self {
        self.class_name = Some(class_name.to_string());
        self
    }

    /// Renders the icon container as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-icon-container".to_string()];

        if let Some(ref size) = self.size {
            classes.push(format!("telegram-ui-icon-container--{size}"));
        }

        if let Some(ref class_name) = self.class_name {
            classes.push(class_name.clone());
        }

        let class_str = classes.join(" ");

        format!("<div class=\"{}\">{}</div>", class_str, self.icon)
    }
}

impl Default for IconContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for IconContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_container_default() {
        let icon_container = IconContainer::new();
        assert_eq!(icon_container.icon, "");
        assert!(icon_container.size.is_none());
        assert!(icon_container.class_name.is_none());
    }

    #[test]
    fn test_icon_container_required_icon() {
        let icon_container = IconContainer::new().icon("icon=arrow_up");
        let html = icon_container.render();
        assert!(html.contains("icon=arrow_up"));
    }

    #[test]
    fn test_icon_container_with_size() {
        let icon_container = IconContainer::new().icon("icon=arrow_up").size("24px");
        let html = icon_container.render();
        assert!(html.contains("telegram-ui-icon-container--24px"));
    }

    #[test]
    fn test_icon_container_with_class_name() {
        let icon_container = IconContainer::new()
            .icon("icon=arrow_up")
            .class_name("custom-class");
        let html = icon_container.render();
        assert!(html.contains("custom-class"));
    }

    #[test]
    fn test_icon_container_full() {
        let icon_container = IconContainer::new()
            .icon("icon=arrow_up")
            .size("24px")
            .class_name("custom-class");
        let html = icon_container.render();
        assert!(html.contains("telegram-ui-icon-container"));
        assert!(html.contains("telegram-ui-icon-container--24px"));
        assert!(html.contains("custom-class"));
        assert!(html.contains("icon=arrow_up"));
    }
}
