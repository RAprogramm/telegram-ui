//! Service components for Telegram UI

use std::fmt;

/// Divider component
#[derive(Debug, Clone)]
pub struct Divider {
    inset: bool,
}

impl Divider {
    /// Creates a new Divider with default settings
    pub fn new() -> Self {
        Self { inset: false }
    }

    /// Sets whether the divider should be inset
    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    /// Returns whether the divider is inset
    pub fn is_inset(&self) -> bool {
        self.inset
    }

    /// Render the divider as HTML string
    pub fn render(&self) -> String {
        let class = if self.inset {
            "telegram-ui-divider telegram-ui-divider--inset"
        } else {
            "telegram-ui-divider"
        };

        format!("<div class=\"{}\"></div>", class)
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Divider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Badge component
#[derive(Debug, Clone)]
pub struct Badge {
    text: String,
    primary: bool,
    critical: bool,
    dot: bool,
}

impl Badge {
    /// Creates a new Badge with default settings
    pub fn new() -> Self {
        Self {
            text: String::new(),
            primary: true,
            critical: false,
            dot: false,
        }
    }

    /// Sets the badge text
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Sets whether the badge should use primary style
    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }

    /// Sets whether the badge should use critical style
    pub fn critical(mut self, critical: bool) -> Self {
        self.critical = critical;
        self
    }

    /// Sets whether the badge should be a dot
    pub fn dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    /// Returns the badge text
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Returns whether the badge uses primary style
    pub fn is_primary(&self) -> bool {
        self.primary
    }

    /// Returns whether the badge uses critical style
    pub fn is_critical(&self) -> bool {
        self.critical
    }

    /// Returns whether the badge is a dot
    pub fn is_dot(&self) -> bool {
        self.dot
    }

    /// Render the badge as HTML string
    pub fn render(&self) -> String {
        if self.dot {
            format!("<div class=\"telegram-ui-badge telegram-ui-badge--dot\"></div>")
        } else {
            let color_class = if self.critical {
                "telegram-ui-badge--critical"
            } else if self.primary {
                "telegram-ui-badge--primary"
            } else {
                "telegram-ui-badge"
            };

            format!(
                "<div class=\"{}\">{}</div>",
                color_class, self.text
            )
        }
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Progress component
#[derive(Debug, Clone)]
pub struct Progress {
    value: f32,
    max: f32,
    size: String,
}

impl Progress {
    /// Creates a new Progress with default settings
    pub fn new() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
            size: "m".to_string(),
        }
    }

    /// Sets the current value
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.max(0.0).min(self.max);
        self
    }

    /// Sets the maximum value
    pub fn max(mut self, max: f32) -> Self {
        self.max = max.max(0.1);
        self
    }

    /// Sets the progress size
    pub fn size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    /// Returns the current value
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Returns the maximum value
    pub fn get_max(&self) -> f32 {
        self.max
    }

    /// Returns the progress size
    pub fn get_size(&self) -> &str {
        &self.size
    }

    /// Returns the progress percentage
    pub fn get_percentage(&self) -> f32 {
        if self.max == 0.0 {
            0.0
        } else {
            (self.value / self.max) * 100.0
        }
    }

    /// Render the progress as HTML string
    pub fn render(&self) -> String {
        let width = self.get_percentage();
        let size_class = match self.size.as_str() {
            "s" => "s",
            "l" => "l",
            _ => "m",
        };

        format!(
            "<div class=\"telegram-ui-progress telegram-ui-progress--{}\">\n  <div class=\"telegram-ui-progress-bar\" style=\"width: {}%;\"></div>\n</div>",
            size_class, width
        )
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divider_render() {
        let divider = Divider::new();
        let html = divider.render();
        assert!(html.contains("telegram-ui-divider"));
    }

    #[test]
    fn test_divider_inset() {
        let divider = Divider::new().inset(true);
        let html = divider.render();
        assert!(html.contains("telegram-ui-divider--inset"));
    }

    #[test]
    fn test_badge_render() {
        let badge = Badge::new().text("5");
        let html = badge.render();
        assert!(html.contains("telegram-ui-badge"));
        assert!(html.contains("5"));
    }

    #[test]
    fn test_badge_critical() {
        let badge = Badge::new().critical(true).text("Error");
        let html = badge.render();
        assert!(html.contains("telegram-ui-badge--critical"));
    }

    #[test]
    fn test_badge_dot() {
        let badge = Badge::new().dot(true);
        let html = badge.render();
        assert!(html.contains("telegram-ui-badge--dot"));
    }

    #[test]
    fn test_progress_render() {
        let progress = Progress::new().value(50.0).max(100.0);
        let html = progress.render();
        assert!(html.contains("telegram-ui-progress"));
        assert!(html.contains("width: 50%;"));
    }
}
