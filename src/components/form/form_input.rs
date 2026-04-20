#[derive(Debug, Clone)]
pub struct FormInput {
    header:   Option<String>,
    status:   String,
    before:   Option<String>,
    after:    Option<String>,
    disabled: bool,
    children: String
}

impl FormInput {
    pub fn new() -> Self {
        Self {
            header:   None,
            status:   "default".to_string(),
            before:   None,
            after:    None,
            disabled: false,
            children: String::new()
        }
    }

    pub fn header(mut self, header: &str) -> Self {
        self.header = Some(header.to_string());
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn before(mut self, before: &str) -> Self {
        self.before = Some(before.to_string());
        self
    }

    pub fn after(mut self, after: &str) -> Self {
        self.after = Some(after.to_string());
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
        let status_class = format!("form-input--{}", self.status);
        let disabled_class = if self.disabled {
            "form-input--disabled"
        } else {
            ""
        };

        let before_html = self
            .before
            .as_ref()
            .map(|b| format!("<span class=\"form-input-before\">{}</span>", b))
            .unwrap_or_default();

        let after_html = self
            .after
            .as_ref()
            .map(|a| format!("<span class=\"form-input-after\">{}</span>", a))
            .unwrap_or_default();

        let header_html = self
            .header
            .as_ref()
            .map(|h| format!("<div class=\"form-input-header\">{}</div>", h))
            .unwrap_or_default();

        format!(
            "<div class=\"telegram-ui-form-input {} {}\">{}<label class=\"form-input-body\">{}{}{}</label></div>",
            status_class, disabled_class, header_html, before_html, self.children, after_html
        )
    }
}

impl Default for FormInput {
    fn default() -> Self {
        Self::new()
    }
}
