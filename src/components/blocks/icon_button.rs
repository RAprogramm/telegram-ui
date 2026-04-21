// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Icon Button component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Icon button mode/style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconButtonMode {
    /// Filled button with solid background
    #[default]
    Filled,
    /// Bezeled button with border
    Bezeled,
    /// Plain button with text only
    Plain,
    /// Gray button for secondary actions
    Gray,
    /// Outline button
    Outline,
    /// White button
    White
}

impl IconButtonMode {
    /// Convert to CSS class suffix
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Filled => "telegram-ui-icon-button--filled",
            Self::Bezeled => "telegram-ui-icon-button--bezeled",
            Self::Plain => "telegram-ui-icon-button--plain",
            Self::Gray => "telegram-ui-icon-button--gray",
            Self::Outline => "telegram-ui-icon-button--outline",
            Self::White => "telegram-ui-icon-button--white"
        }
    }
}

/// Icon button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconButtonSize {
    /// Small size
    S,
    /// Medium size (default)
    #[default]
    M,
    /// Large size
    L
}

impl IconButtonSize {
    /// Convert to CSS class suffix
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::S => "telegram-ui-icon-button--s",
            Self::M => "telegram-ui-icon-button--m",
            Self::L => "telegram-ui-icon-button--l"
        }
    }
}

/// Icon button component
#[derive(Debug, Clone)]
pub struct IconButton {
    size:     IconButtonSize,
    mode:     IconButtonMode,
    icon:     String,
    disabled: bool,
    loading:  bool
}

impl IconButton {
    /// Creates a new `IconButton` with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size:     IconButtonSize::M,
            mode:     IconButtonMode::Filled,
            icon:     String::new(),
            disabled: false,
            loading:  false
        }
    }

    /// Sets the icon button size
    #[must_use]
    pub const fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the icon button size from string
    #[must_use]
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "s" => IconButtonSize::S,
            "l" => IconButtonSize::L,
            _ => IconButtonSize::M
        };
        self
    }

    /// Sets the icon button mode/style
    #[must_use]
    pub const fn mode(mut self, mode: IconButtonMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the icon button mode from string
    #[must_use]
    pub fn mode_str(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "bezeled" => IconButtonMode::Bezeled,
            "plain" => IconButtonMode::Plain,
            "gray" => IconButtonMode::Gray,
            "outline" => IconButtonMode::Outline,
            "white" => IconButtonMode::White,
            _ => IconButtonMode::Filled
        };
        self
    }

    /// Sets the icon content
    #[must_use]
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }

    /// Sets whether the icon button should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the icon button should show loading state
    #[must_use]
    pub const fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Returns the icon button size
    #[must_use]
    pub const fn get_size(&self) -> &IconButtonSize {
        &self.size
    }

    /// Returns the icon button mode
    #[must_use]
    pub const fn get_mode(&self) -> &IconButtonMode {
        &self.mode
    }

    /// Returns the icon content
    #[must_use]
    pub fn get_icon(&self) -> &str {
        &self.icon
    }

    /// Render the icon button as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-icon-button"];

        classes.push(self.size.css_class());
        classes.push(self.mode.css_class());
        classes.push("telegram-ui-icon-button--ios");

        if self.loading {
            classes.push("telegram-ui-icon-button--loading");
        }

        if self.disabled {
            classes.push("telegram-ui-icon-button--disabled");
        }

        let class_str = classes.join(" ");

        let mut html = String::new();

        if self.loading {
            html.push_str(&self.render_spinner());
        }

        html.push_str(&format!(
            "<span class=\"icon\">{}</span>",
            escape_html(&self.icon)
        ));

        format!(
            "<button class=\"{}\"{}>{}</button>",
            class_str,
            if self.disabled { " disabled" } else { "" },
            html
        )
    }

    fn render_spinner(&self) -> String {
        let size = match self.size {
            IconButtonSize::S => "s",
            IconButtonSize::M => "m",
            IconButtonSize::L => "l"
        };
        format!("<div class=\"telegram-ui-spinner telegram-ui-spinner--{size}\"></div>")
    }
}

impl Default for IconButton {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for IconButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_button_default() {
        let icon_button = IconButton::new();
        assert_eq!(icon_button.get_size(), &IconButtonSize::M);
        assert_eq!(icon_button.get_mode(), &IconButtonMode::Filled);
        assert_eq!(icon_button.get_icon(), "");
    }

    #[test]
    fn test_icon_button_customization() {
        let icon_button = IconButton::new()
            .size(IconButtonSize::L)
            .mode(IconButtonMode::Outline)
            .icon("icon=settings");

        assert_eq!(icon_button.get_size(), &IconButtonSize::L);
        assert_eq!(icon_button.get_mode(), &IconButtonMode::Outline);
        assert_eq!(icon_button.get_icon(), "icon=settings");
    }

    #[test]
    fn test_icon_button_render() {
        let icon_button = IconButton::new()
            .size(IconButtonSize::M)
            .mode(IconButtonMode::Filled)
            .icon("icon=arrow_up");

        let html = icon_button.render();
        assert!(html.contains("telegram-ui-icon-button"));
        assert!(html.contains("telegram-ui-icon-button--m"));
        assert!(html.contains("telegram-ui-icon-button--filled"));
        assert!(html.contains("icon=arrow_up"));
    }

    #[test]
    fn test_icon_button_disabled() {
        let icon_button = IconButton::new().disabled(true);
        let html = icon_button.render();
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_icon_button_loading() {
        let icon_button = IconButton::new().loading(true);
        let html = icon_button.render();
        assert!(html.contains("telegram-ui-spinner"));
    }
}
