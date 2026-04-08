#[derive(Debug, Clone)]
pub struct Snackbar {
    message: String,
    action: Option<String>,
    before: Option<String>,
    after: Option<String>,
    ios: bool,
}

impl Snackbar {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            action: None,
            before: None,
            after: None,
            ios: false,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
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

    pub fn ios(mut self, ios: bool) -> Self {
        self.ios = ios;
        self
    }

    pub fn render(&self) -> String {
        let ios_class = if self.ios { "snackbar--ios" } else { "" };

        let before_html = self
            .before
            .as_ref()
            .map(|b| format!("<span class=\"snackbar-before\">{}</span>", b))
            .unwrap_or_default();

        let after_html = self
            .after
            .as_ref()
            .map(|a| format!("<span class=\"snackbar-after\">{}</span>", a))
            .unwrap_or_default();

        let action_html = self
            .action
            .as_ref()
            .map(|a| format!("<span class=\"snackbar-action\">{}</span>", a))
            .unwrap_or_default();

        format!(
            r#"<div class="telegram-ui-snackbar {}">
  <div class="snackbar-body">
    {}<span class="snackbar-message">{}{}</span>{}
  </div>
</div>"#,
            ios_class, before_html, self.message, action_html, after_html
        )
    }
}
