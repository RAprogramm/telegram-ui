#[derive(Debug, Clone)]
pub struct Timeline {
    horizontal: bool,
    items:      Vec<String>
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            horizontal: false,
            items:      Vec::new()
        }
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    pub fn render(&self) -> String {
        let orientation_class = if self.horizontal {
            "timeline--horizontal"
        } else {
            ""
        };

        format!(
            "<div class=\"telegram-ui-timeline {}\">{}</div>",
            orientation_class,
            self.items.join("")
        )
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
