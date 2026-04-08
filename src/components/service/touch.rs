#[derive(Debug, Clone)]
pub struct Touch {
    children: String,
}

impl Touch {
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
        format!("<div class=\"telegram-ui-touch\">{}</div>", self.children)
    }
}
