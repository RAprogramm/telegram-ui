#[derive(Debug, Clone)]
pub struct ColorInput {
    value: String,
    text: Option<String>,
}

impl ColorInput {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            text: None,
        }
    }

    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn render(&self) -> String {
        let text_html = self
            .text
            .as_ref()
            .map(|t| format!("<span class=\"color-input-text\">{}</span>", t))
            .unwrap_or_default();

        format!(
            r#"<div class="telegram-ui-color-input">
  <div class="color-input-circle">
    <div class="color-input-circle-inner" style="background: {}"></div>
  </div>
  {}
</div>"#,
            self.value, text_html
        )
    }
}
