#[derive(Debug, Clone)]
pub struct HorizontalScroll {
    children: String,
}

impl HorizontalScroll {
    pub fn new() -> Self {
        Self {
            children: String::new(),
        }
    }

    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-horizontal-scroll\">{}</div>",
            self.children
        )
    }
}
