#[derive(Debug, Clone)]
pub struct Spoiler {
    visible: bool,
    children: String,
}

impl Spoiler {
    pub fn new() -> Self {
        Self {
            visible: false,
            children: String::new(),
        }
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    pub fn render(&self) -> String {
        let visible_class = if self.visible { "spoiler--visible" } else { "" };

        format!(
            "<div class=\"telegram-ui-spoiler {}\">{}</div>",
            visible_class, self.children
        )
    }
}
