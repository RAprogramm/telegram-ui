#[derive(Debug, Clone)]
pub struct RootRenderer {
    children: String,
}

impl RootRenderer {
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
            "<div class=\"telegram-ui-root-renderer\">{}</div>",
            self.children
        )
    }
}
