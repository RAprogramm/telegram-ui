//! Overlay components for Telegram UI

use std::fmt;

/// Backdrop component
#[derive(Debug, Clone)]
pub struct Backdrop {
    visible: bool,
    on_click: Option<String>,
}

impl Backdrop {
    /// Creates a new Backdrop with default settings
    pub fn new() -> Self {
        Self {
            visible: false,
            on_click: None,
        }
    }

    /// Sets whether the backdrop is visible
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets the click handler
    pub fn on_click(mut self, handler: &str) -> Self {
        self.on_click = Some(handler.to_string());
        self
    }

    /// Returns whether the backdrop is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the click handler
    pub fn get_on_click(&self) -> Option<&str> {
        self.on_click.as_deref()
    }

    /// Render the backdrop as HTML string
    pub fn render(&self) -> String {
        let style = if self.visible {
            "display: block;"
        } else {
            "display: none;"
        };

        let on_click_attr = self.on_click
            .as_ref()
            .map(|h| format!("onclick=\"{}\"", h))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-backdrop\" style=\"{}\"{}></div>",
            style, on_click_attr
        )
    }
}

impl Default for Backdrop {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Backdrop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Modal component
#[derive(Debug, Clone)]
pub struct Modal {
    visible: bool,
    title: Option<String>,
    children: String,
    footer: Option<String>,
}

impl Modal {
    /// Creates a new Modal with default settings
    pub fn new() -> Self {
        Self {
            visible: false,
            title: None,
            children: String::new(),
            footer: None,
        }
    }

    /// Sets whether the modal is visible
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets the modal title
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Sets the modal children content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Sets the modal footer content
    pub fn footer(mut self, footer: &str) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    /// Returns whether the modal is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the modal title
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the modal children content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Returns the modal footer content
    pub fn get_footer(&self) -> Option<&str> {
        self.footer.as_deref()
    }

    /// Render the modal as HTML string
    pub fn render(&self) -> String {
        let style = if self.visible {
            "display: block;"
        } else {
            "display: none;"
        };

        let title_html = self.title
            .as_ref()
            .map(|t| format!("<div class=\"telegram-ui-modal-title\">{}</div>", t))
            .unwrap_or_default();

        let footer_html = self.footer
            .as_ref()
            .map(|f| format!("<div class=\"telegram-ui-modal-footer\">{}</div>", f))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-modal\" style=\"{}\">\n  <div class=\"telegram-ui-modal-content\">\n    {}\n    {}\n    {}\n  </div>\n</div>",
            style, title_html, self.children, footer_html
        )
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Modal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Toast component
#[derive(Debug, Clone)]
pub struct Toast {
    visible: bool,
    message: String,
    position: String,
    duration: Option<u32>,
}

impl Toast {
    /// Creates a new Toast with default settings
    pub fn new() -> Self {
        Self {
            visible: false,
            message: String::new(),
            position: "bottom".to_string(),
            duration: None,
        }
    }

    /// Sets whether the toast is visible
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets the toast message
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// Sets the toast position
    pub fn position(mut self, position: &str) -> Self {
        self.position = position.to_string();
        self
    }

    /// Sets the toast duration in milliseconds
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Returns whether the toast is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns the toast message
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Returns the toast position
    pub fn get_position(&self) -> &str {
        &self.position
    }

    /// Returns the toast duration
    pub fn get_duration(&self) -> Option<u32> {
        self.duration
    }

    /// Render the toast as HTML string
    pub fn render(&self) -> String {
        let style = if self.visible {
            "display: block;"
        } else {
            "display: none;"
        };

        let position_style = match self.position.as_str() {
            "top" => "top: 20px;",
            "bottom" => "bottom: 20px;",
            "center" => "top: 50%; transform: translateY(-50%);",
            _ => "bottom: 20px;",
        };

        let duration_attr = self.duration
            .map(|d| format!("data-duration=\"{}\"", d))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-toast\" style=\"{} {}\"{}>{}</div>",
            style, position_style, duration_attr, self.message
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
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backdrop_render() {
        let backdrop = Backdrop::new().visible(true);
        let html = backdrop.render();
        assert!(html.contains("telegram-ui-backdrop"));
        assert!(html.contains("display: block;"));
    }

    #[test]
    fn test_modal_render() {
        let modal = Modal::new()
            .visible(true)
            .title("Confirm")
            .children("Are you sure?")
            .footer("<button>OK</button>");

        let html = modal.render();
        assert!(html.contains("telegram-ui-modal"));
        assert!(html.contains("telegram-ui-modal-title"));
        assert!(html.contains("Are you sure?"));
    }

    #[test]
    fn test_toast_render() {
        let toast = Toast::new()
            .visible(true)
            .message("Saved successfully!")
            .position("top")
            .duration(3000);

        let html = toast.render();
        assert!(html.contains("telegram-ui-toast"));
        assert!(html.contains("Saved successfully!"));
        assert!(html.contains("top: 20px;"));
        assert!(html.contains("data-duration=\"3000\""));
    }
}
