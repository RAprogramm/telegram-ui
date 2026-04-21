#[derive(Debug, Clone)]
pub struct Spoiler {
    visible:  bool,
    children: String
}

impl Spoiler {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            visible:  false,
            children: String::new()
        }
    }

    #[must_use]
    pub const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let visible_class = if self.visible { "spoiler--visible" } else { "" };

        format!(
            "<div class=\"telegram-ui-spoiler {}\">{}</div>",
            visible_class, self.children
        )
    }
}

impl Default for Spoiler {
    fn default() -> Self {
        Self::new()
    }
}
