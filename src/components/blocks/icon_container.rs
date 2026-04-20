#[derive(Debug, Clone)]
pub struct IconContainer {
    children: String
}

impl IconContainer {
    pub fn new() -> Self {
        Self {
            children: String::new()
        }
    }

    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-icon-container\">{}</div>",
            self.children
        )
    }
}

impl Default for IconContainer {
    fn default() -> Self {
        Self::new()
    }
}
