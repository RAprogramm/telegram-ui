#[derive(Debug, Clone)]
pub struct PinInput {
    length: usize,
    title:  Option<String>,
    ios:    bool
}

impl PinInput {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            length: 4,
            title:  None,
            ios:    false
        }
    }

    #[must_use]
    pub const fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    #[must_use]
    pub const fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let ios_class = if self.ios { "pin-input--ios" } else { "" };

        let cells: String = (0..self.length)
            .map(|_| "<span class=\"pin-input-cell\"></span>".to_string())
            .collect();

        let title_html = self
            .title
            .as_ref()
            .map(|t| format!("<div class=\"pin-input-title\">{t}</div>"))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-pin-input {ios_class}\"><div class=\"pin-input-header\">{title_html}</div><div class=\"pin-input-cells\">{cells}</div></div>"
        )
    }
}

impl Default for PinInput {
    fn default() -> Self {
        Self::new()
    }
}
