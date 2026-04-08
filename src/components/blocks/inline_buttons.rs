#[derive(Debug, Clone)]
pub struct InlineButtons {
    mode: InlineButtonsMode,
    ios: bool,
    items: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub enum InlineButtonsMode {
    #[default]
    Default,
}

impl InlineButtons {
    pub fn new() -> Self {
        Self {
            mode: InlineButtonsMode::Default,
            ios: false,
            items: Vec::new(),
        }
    }

    pub fn mode(mut self, mode: &str) -> Self {
        self.mode = match mode {
            _ => InlineButtonsMode::Default,
        };
        self
    }

    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    pub fn render(&self) -> String {
        let ios_class = if self.ios { "inline-buttons--ios" } else { "" };

        format!(
            "<div class=\"telegram-ui-inline-buttons {}\">{}</div>",
            ios_class,
            self.items.join("")
        )
    }
}
