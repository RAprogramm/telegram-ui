#[derive(Debug, Clone)]
pub struct Chip {
    mode:     ChipMode,
    value:    String,
    selected: bool,
    before:   Option<String>,
    after:    Option<String>
}

#[derive(Debug, Clone, Default)]
pub enum ChipMode {
    #[default]
    Elevated,
    Mono,
    Outline
}

impl Chip {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self {
            mode:     ChipMode::Elevated,
            value:    value.to_string(),
            selected: false,
            before:   None,
            after:    None
        }
    }

    #[must_use]
    pub fn mode(mut self, mode: &str) -> Self {
        self.mode = match mode {
            "mono" => ChipMode::Mono,
            "outline" => ChipMode::Outline,
            _ => ChipMode::Elevated
        };
        self
    }

    #[must_use]
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    #[must_use]
    pub fn before(mut self, before: &str) -> Self {
        self.before = Some(before.to_string());
        self
    }

    #[must_use]
    pub fn after(mut self, after: &str) -> Self {
        self.after = Some(after.to_string());
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let mode_class = match self.mode {
            ChipMode::Elevated => "chip--elevated",
            ChipMode::Mono => "chip--mono",
            ChipMode::Outline => "chip--outline"
        };

        let before_html = self
            .before
            .as_ref()
            .map(|b| format!("<span class=\"chip-before\">{b}</span>"))
            .unwrap_or_default();

        let after_html = self
            .after
            .as_ref()
            .map(|a| format!("<span class=\"chip-after\">{a}</span>"))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-chip {} {}\">{}<span class=\"chip-text\">{}</span>{}</div>",
            mode_class,
            if self.selected { "chip--selected" } else { "" },
            before_html,
            self.value,
            after_html
        )
    }
}
