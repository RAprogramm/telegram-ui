//! Feedback components for Telegram UI

use std::fmt;

/// Alert component
#[derive(Debug, Clone)]
pub struct Alert {
    message: String,
    title: Option<String>,
    kind: AlertKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertKind {
    Information,
    Success,
    Warning,
    Error,
}

impl Default for AlertKind {
    fn default() -> Self {
        Self::Information
    }
}

impl AlertKind {
    fn css_class(&self) -> &'static str {
        match self {
            AlertKind::Information => "telegram-ui-alert--info",
            AlertKind::Success => "telegram-ui-alert--success",
            AlertKind::Warning => "telegram-ui-alert--warning",
            AlertKind::Error => "telegram-ui-alert--error",
        }
    }
}

impl Alert {
    /// Creates a new Alert with default settings
    pub fn new() -> Self {
        Self {
            message: String::new(),
            title: None,
            kind: AlertKind::default(),
        }
    }

    /// Sets the alert message
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// Sets the alert title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the alert kind/type
    pub fn kind(mut self, kind: AlertKind) -> Self {
        self.kind = kind;
        self
    }

    /// Returns the alert message
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Returns the alert title
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the alert kind
    pub fn get_kind(&self) -> &AlertKind {
        &self.kind
    }

    /// Render the alert as HTML string
    pub fn render(&self) -> String {
        let kind_class = self.kind.css_class();

        let title_html = self.title
            .as_ref()
            .map(|t| format!("<div class=\"telegram-ui-alert-title\">{}</div>", t))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-alert {}\">\n  {}\n  <div class=\"telegram-ui-alert-message\">{}</div>\n</div>",
            kind_class, title_html, self.message
        )
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Skeleton component
#[derive(Debug, Clone)]
pub struct Skeleton {
    width: Option<String>,
    height: Option<String>,
    rounded: bool,
    animation: bool,
}

impl Skeleton {
    /// Creates a new Skeleton with default settings
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            rounded: false,
            animation: true,
        }
    }

    /// Sets the skeleton width
    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    /// Sets the skeleton height
    pub fn height(mut self, height: &str) -> Self {
        self.height = Some(height.to_string());
        self
    }

    /// Sets whether the skeleton should be rounded
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    /// Sets whether the skeleton should have animation
    pub fn animation(mut self, animation: bool) -> Self {
        self.animation = animation;
        self
    }

    /// Returns the skeleton width
    pub fn get_width(&self) -> Option<&str> {
        self.width.as_deref()
    }

    /// Returns the skeleton height
    pub fn get_height(&self) -> Option<&str> {
        self.height.as_deref()
    }

    /// Returns whether the skeleton is rounded
    pub fn is_rounded(&self) -> bool {
        self.rounded
    }

    /// Returns whether the skeleton has animation
    pub fn has_animation(&self) -> bool {
        self.animation
    }

    /// Render the skeleton as HTML string
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-skeleton"];

        if self.rounded {
            classes.push("--rounded");
        }

        let class_str = classes.join(" ");

        let style = {
            let mut s = Vec::new();

            if let Some(width) = &self.width {
                s.push(format!("width: {};", width));
            }

            if let Some(height) = &self.height {
                s.push(format!("height: {};", height));
            }

            if !self.animation {
                s.push("animation: none;".to_string());
            }

            s.join(" ")
        };

        format!(
            "<div class=\"{}\"{}></div>",
            class_str,
            if style.is_empty() { String::new() } else { format!(" style=\"{}\"", style) }
        )
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Skeleton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Empty state component
#[derive(Debug, Clone)]
pub struct EmptyState {
    icon: Option<String>,
    title: String,
    description: Option<String>,
    action: Option<String>,
}

impl EmptyState {
    /// Creates a new EmptyState with default settings
    pub fn new() -> Self {
        Self {
            icon: None,
            title: String::new(),
            description: None,
            action: None,
        }
    }

    /// Sets the empty state icon
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Sets the empty state title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the empty state description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the empty state action (button HTML)
    pub fn action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    /// Returns the empty state icon
    pub fn get_icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Returns the empty state title
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Returns the empty state description
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the empty state action
    pub fn get_action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    /// Render the empty state as HTML string
    pub fn render(&self) -> String {
        let icon_html = self.icon
            .as_ref()
            .map(|i| format!("<div class=\"telegram-ui-empty-state-icon\">{}</div>", i))
            .unwrap_or_default();

        let description_html = self.description
            .as_ref()
            .map(|d| format!("<div class=\"telegram-ui-empty-state-description\">{}</div>", d))
            .unwrap_or_default();

        let action_html = self.action
            .as_ref()
            .map(|a| format!("<div class=\"telegram-ui-empty-state-action\">{}</div>", a))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-empty-state\">\n  {}\n  <div class=\"telegram-ui-empty-state-title\">{}</div>\n  {}\n  {}\n</div>",
            icon_html, self.title, description_html, action_html
        )
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EmptyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Spinner component
#[derive(Debug, Clone)]
pub struct Spinner {
    size: Option<String>,
    animation: bool,
}

impl Spinner {
    /// Creates a new Spinner with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            animation: true,
        }
    }

    /// Sets the spinner size
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets whether the spinner should have animation
    pub fn animation(mut self, animation: bool) -> Self {
        self.animation = animation;
        self
    }

    /// Returns the spinner size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns whether the spinner has animation
    pub fn has_animation(&self) -> bool {
        self.animation
    }

    /// Render the spinner as HTML string
    pub fn render(&self) -> String {
        let style = {
            let mut s = Vec::new();

            if let Some(size) = &self.size {
                s.push(format!("width: {};", size));
                s.push(format!("height: {};", size));
            }

            if !self.animation {
                s.push("animation: none;".to_string());
            }

            s.join(" ")
        };

        format!(
            "<div class=\"telegram-ui-spinner\"{}></div>",
            if style.is_empty() { String::new() } else { format!(" style=\"{}\"", style) }
        )
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_render() {
        let alert = Alert::new()
            .title("Info")
            .message("This is an information message");

        let html = alert.render();
        assert!(html.contains("telegram-ui-alert"));
        assert!(html.contains("Info"));
        assert!(html.contains("This is an information message"));
    }

    #[test]
    fn test_alert_kind() {
        let alert = Alert::new().kind(AlertKind::Error).message("Error occurred");
        let html = alert.render();
        assert!(html.contains("telegram-ui-alert--error"));
    }

    #[test]
    fn test_skeleton_render() {
        let skeleton = Skeleton::new()
            .width("100px")
            .height("20px")
            .rounded(true);

        let html = skeleton.render();
        assert!(html.contains("telegram-ui-skeleton"));
        assert!(html.contains("--rounded"));
        assert!(html.contains("width: 100px;"));
    }

    #[test]
    fn test_empty_state_render() {
        let empty = EmptyState::new()
            .title("No Data")
            .description("Try again later")
            .action("<button>Retry</button>");

        let html = empty.render();
        assert!(html.contains("telegram-ui-empty-state"));
        assert!(html.contains("No Data"));
        assert!(html.contains("Try again later"));
    }

    #[test]
    fn test_spinner_render() {
        let spinner = Spinner::new()
            .size("24px")
            .animation(false);

        let html = spinner.render();
        assert!(html.contains("telegram-ui-spinner"));
        assert!(html.contains("width: 24px;"));
        assert!(html.contains("height: 24px;"));
        assert!(html.contains("animation: none;"));
    }
}
