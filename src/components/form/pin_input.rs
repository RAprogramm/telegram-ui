#[derive(Debug, Clone)]
pub struct PinInput {
    length: usize,
    title: Option<String>,
    ios: bool,
}

impl PinInput {
    pub fn new() -> Self {
        Self {
            length: 4,
            title: None,
            ios: false,
        }
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    pub fn render(&self) -> String {
        let ios_class = if self.ios { "pin-input--ios" } else { "" };

        let cells: String = (0..self.length)
            .map(|_| "<span class=\"pin-input-cell\"></span>".to_string())
            .collect();

        let title_html = self
            .title
            .as_ref()
            .map(|t| format!("<div class=\"pin-input-title\">{}</div>", t))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-pin-input {}\"><div class=\"pin-input-header\">{}</div><div class=\"pin-input-cells\">{}</div></div>",
            ios_class, title_html, cells
        )
    }
}
