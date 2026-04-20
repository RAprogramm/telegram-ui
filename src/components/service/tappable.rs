#[derive(Debug, Clone)]
pub struct Tappable {
    interactive: bool,
    disabled:    bool,
    children:    String
}

impl Tappable {
    pub fn new() -> Self {
        Self {
            interactive: true,
            disabled:    false,
            children:    String::new()
        }
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    pub fn render(&self) -> String {
        let interactive_class = if self.interactive {
            "tappable--interactive"
        } else {
            ""
        };
        let disabled_class = if self.disabled {
            "tappable--disabled"
        } else {
            ""
        };

        format!(
            "<div class=\"telegram-ui-tappable {} {}\">{}</div>",
            interactive_class, disabled_class, self.children
        )
    }
}

impl Default for Tappable {
    fn default() -> Self {
        Self::new()
    }
}
