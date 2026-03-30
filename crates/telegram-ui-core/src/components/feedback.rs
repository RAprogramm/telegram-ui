//! Spinner component for Telegram UI

use std::fmt;

/// Spinner size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    /// Small size
    S,
    /// Medium size (default)
    M,
    /// Large size
    L,
}

impl Default for SpinnerSize {
    fn default() -> Self {
        Self::M
    }
}

impl SpinnerSize {
    /// Convert to CSS class suffix
    pub fn css_class(&self) -> &'static str {
        match self {
            SpinnerSize::S => "--s",
            SpinnerSize::M => "--m",
            SpinnerSize::L => "--l",
        }
    }
}

/// Spinner component
#[derive(Debug, Clone)]
pub struct Spinner {
    size: SpinnerSize,
}

impl Spinner {
    /// Creates a new Spinner with default size
    pub fn new() -> Self {
        Self {
            size: SpinnerSize::M,
        }
    }

    /// Sets the spinner size
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the spinner size from string
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "s" => SpinnerSize::S,
            "l" => SpinnerSize::L,
            _ => SpinnerSize::M,
        };
        self
    }

    /// Returns the spinner size
    pub fn get_size(&self) -> &SpinnerSize {
        &self.size
    }

    /// Render the spinner as HTML string
    pub fn render(&self) -> String {
        let size_class = self.size.css_class();
        format!(
            "<div class=\"telegram-ui-spinner telegram-ui-spinner--{}\">\n  <div class=\"telegram-ui-spinner-circle\"></div>\n</div>",
            size_class.trim_start_matches("--")
        )
    }

    /// Render as full component with circle
    pub fn render_full(&self) -> String {
        let size = match self.size {
            SpinnerSize::S => "16px",
            SpinnerSize::M => "20px",
            SpinnerSize::L => "24px",
        };
        let border_width = match self.size {
            SpinnerSize::S => "1.5px",
            SpinnerSize::M => "2px",
            SpinnerSize::L => "3px",
        };

        format!(
            "<div class=\"telegram-ui-spinner telegram-ui-spinner--{}\" style=\"width: {}; height: {};\"><div class=\"telegram-ui-spinner-circle\" style=\"border-width: {};\"></div></div>",
            size.trim_start_matches("--"),
            size, size, border_width
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
    fn test_spinner_default() {
        let spinner = Spinner::new();
        assert_eq!(spinner.get_size(), &SpinnerSize::M);
    }

    #[test]
    fn test_spinner_custom_size() {
        let spinner = Spinner::new().size(SpinnerSize::S);
        assert_eq!(spinner.get_size(), &SpinnerSize::S);
    }

    #[test]
    fn test_spinner_render() {
        let spinner = Spinner::new().size(SpinnerSize::M);
        let html = spinner.render();
        assert!(html.contains("telegram-ui-spinner"));
        assert!(html.contains("telegram-ui-spinner--m"));
        assert!(html.contains("telegram-ui-spinner-circle"));
    }

    #[test]
    fn test_spinner_large() {
        let spinner = Spinner::new().size(SpinnerSize::L);
        let html = spinner.render_full();
        assert!(html.contains("width: 24px"));
        assert!(html.contains("height: 24px"));
        assert!(html.contains("border-width: 3px"));
    }
}
