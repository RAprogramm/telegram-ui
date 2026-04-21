#[derive(Debug, Clone)]
pub struct ColorInput {
    value: String,
    text:  Option<String>
}

impl ColorInput {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: String::new(),
            text:  None
        }
    }

    #[must_use]
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    #[must_use]
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let text_html = self
            .text
            .as_ref()
            .map(|t| format!("<span class=\"color-input-text\">{t}</span>"))
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

impl Default for ColorInput {
    fn default() -> Self {
        Self::new()
    }
}
