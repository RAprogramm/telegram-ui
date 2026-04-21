// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Button component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Button mode/style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonMode {
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

impl ButtonMode {
    /// Convert to CSS class suffix
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::Filled => "telegram-ui-button--filled",
            Self::Bezeled => "telegram-ui-button--bezeled",
            Self::Plain => "telegram-ui-button--plain",
            Self::Gray => "telegram-ui-button--gray",
            Self::Outline => "telegram-ui-button--outline",
            Self::White => "telegram-ui-button--white"
        }
    }
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Small size
    S,
    /// Medium size (default)
    #[default]
    M,
    /// Large size
    L
}

impl ButtonSize {
    /// Convert to CSS class suffix
    #[must_use]
    pub const fn css_class(&self) -> &'static str {
        match self {
            Self::S => "telegram-ui-button--s",
            Self::M => "telegram-ui-button--m",
            Self::L => "telegram-ui-button--l"
        }
    }
}

/// Button component
#[derive(Debug, Clone)]
pub struct Button {
    size:      ButtonSize,
    mode:      ButtonMode,
    children:  String,
    stretched: bool,
    disabled:  bool,
    loading:   bool,
    before:    Option<String>,
    after:     Option<String>
}

impl Button {
    /// Creates a new Button with default settings
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size:      ButtonSize::M,
            mode:      ButtonMode::Filled,
            children:  String::new(),
            stretched: false,
            disabled:  false,
            loading:   false,
            before:    None,
            after:     None
        }
    }

    /// Sets the button size
    #[must_use]
    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button size from string
    #[must_use]
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "s" => ButtonSize::S,
            "l" => ButtonSize::L,
            _ => ButtonSize::M
        };
        self
    }

    /// Sets the button mode/style
    #[must_use]
    pub const fn mode(mut self, mode: ButtonMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the button mode from string
    #[must_use]
    pub fn mode_str(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "bezeled" => ButtonMode::Bezeled,
            "plain" => ButtonMode::Plain,
            "gray" => ButtonMode::Gray,
            "outline" => ButtonMode::Outline,
            "white" => ButtonMode::White,
            _ => ButtonMode::Filled
        };
        self
    }

    /// Sets the button children content
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Sets whether the button should be stretched to full width
    #[must_use]
    pub const fn stretched(mut self, stretched: bool) -> Self {
        self.stretched = stretched;
        self
    }

    /// Sets whether the button should be disabled
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the button should show loading state
    #[must_use]
    pub const fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets content to show before the button text
    #[must_use]
    pub fn before(mut self, content: &str) -> Self {
        self.before = Some(content.to_string());
        self
    }

    /// Sets content to show after the button text
    #[must_use]
    pub fn after(mut self, content: &str) -> Self {
        self.after = Some(content.to_string());
        self
    }

    /// Returns the button size
    #[must_use]
    pub const fn get_size(&self) -> &ButtonSize {
        &self.size
    }

    /// Returns the button mode
    #[must_use]
    pub const fn get_mode(&self) -> &ButtonMode {
        &self.mode
    }

    /// Returns the button children content
    #[must_use]
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the button as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-button"];

        // Add size class
        classes.push(self.size.css_class());

        // Add mode class
        classes.push(self.mode.css_class());

        // Add platform class
        classes.push("telegram-ui-button--ios");

        // Add stretched class
        if self.stretched {
            classes.push("telegram-ui-button--stretched");
        }

        // Add loading class
        if self.loading {
            classes.push("telegram-ui-button--loading");
        }

        // Add disabled class
        if self.disabled {
            classes.push("telegram-ui-button--disabled");
        }

        let class_str = classes.join(" ");

        let mut html = String::new();

        if self.loading {
            html.push_str(&self.render_spinner());
        }

        if let Some(before) = &self.before {
            html.push_str(&format!(
                "<div class=\"before\">{}</div>",
                escape_html(before)
            ));
        }

        html.push_str(&format!(
            "<span class=\"content\">{}</span>",
            escape_html(&self.children)
        ));

        if let Some(after) = &self.after {
            html.push_str(&format!(
                "<div class=\"after\">{}</div>",
                escape_html(after)
            ));
        }

        format!(
            "<button class=\"{}\"{}>{}</button>",
            class_str,
            if self.disabled { " disabled" } else { "" },
            html
        )
    }

    fn render_spinner(&self) -> String {
        let size = match self.size {
            ButtonSize::S => "s",
            ButtonSize::M => "m",
            ButtonSize::L => "l"
        };
        format!("<div class=\"telegram-ui-spinner telegram-ui-spinner--{size}\"></div>")
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_default() {
        let button = Button::new();
        assert_eq!(button.get_size(), &ButtonSize::M);
        assert_eq!(button.get_mode(), &ButtonMode::Filled);
        assert_eq!(button.get_children(), "");
    }

    #[test]
    fn test_button_customization() {
        let button = Button::new()
            .size(ButtonSize::L)
            .mode(ButtonMode::Outline)
            .children("Click me");

        assert_eq!(button.get_size(), &ButtonSize::L);
        assert_eq!(button.get_mode(), &ButtonMode::Outline);
        assert_eq!(button.get_children(), "Click me");
    }

    #[test]
    fn test_button_render() {
        let button = Button::new()
            .size(ButtonSize::M)
            .mode(ButtonMode::Filled)
            .children("Submit");

        let html = button.render();
        assert!(html.contains("telegram-ui-button"));
        assert!(html.contains("telegram-ui-button--m"));
        assert!(html.contains("telegram-ui-button--filled"));
        assert!(html.contains("Submit"));
    }

    #[test]
    fn test_button_with_before_after() {
        let button = Button::new().before("🔍").after("➡").children("Search");

        let html = button.render();
        assert!(html.contains("<div class=\"before\">🔍</div>"));
        assert!(html.contains("<div class=\"after\">➡</div>"));
    }

    #[test]
    fn test_button_disabled() {
        let button = Button::new().disabled(true);
        let html = button.render();
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_button_loading() {
        let button = Button::new().loading(true);
        let html = button.render();
        assert!(html.contains("telegram-ui-spinner"));
    }
}
