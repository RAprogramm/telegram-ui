#[derive(Debug, Clone)]
pub struct IconButton {
    size:     IconButtonSize,
    mode:     IconButtonMode,
    children: String
}

#[derive(Debug, Clone, Default)]
pub enum IconButtonSize {
    #[default]
    M,
    S,
    L
}

#[derive(Debug, Clone, Default)]
pub enum IconButtonMode {
    #[default]
    Bezeled,
    Plain,
    Gray,
    Outline
}

impl IconButton {
    pub fn new() -> Self {
        Self {
            size:     IconButtonSize::M,
            mode:     IconButtonMode::Bezeled,
            children: String::new()
        }
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = match size {
            "s" => IconButtonSize::S,
            "l" => IconButtonSize::L,
            _ => IconButtonSize::M
        };
        self
    }

    pub fn mode(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "plain" => IconButtonMode::Plain,
            "gray" => IconButtonMode::Gray,
            "outline" => IconButtonMode::Outline,
            _ => IconButtonMode::Bezeled
        };
        self
    }

    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    pub fn render(&self) -> String {
        let size_class = match self.size {
            IconButtonSize::S => "icon-button--s",
            IconButtonSize::M => "icon-button--m",
            IconButtonSize::L => "icon-button--l"
        };

        let mode_class = match self.mode {
            IconButtonMode::Bezeled => "icon-button--bezeled",
            IconButtonMode::Plain => "icon-button--plain",
            IconButtonMode::Gray => "icon-button--gray",
            IconButtonMode::Outline => "icon-button--outline"
        };

        format!(
            "<button class=\"telegram-ui-icon-button {} {}\">{}</button>",
            size_class, mode_class, self.children
        )
    }
}

impl Default for IconButton {
    fn default() -> Self {
        Self::new()
    }
}
