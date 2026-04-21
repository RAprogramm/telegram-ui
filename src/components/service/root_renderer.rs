#[derive(Debug, Clone)]
pub struct RootRenderer {
    children: String
}

impl RootRenderer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            children: String::new()
        }
    }

    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-root-renderer\">{}</div>",
            self.children
        )
    }
}

impl Default for RootRenderer {
    fn default() -> Self {
        Self::new()
    }
}
