#[derive(Debug, Clone)]
pub struct FileInput {
    label: String,
}

impl FileInput {
    pub fn new() -> Self {
        Self {
            label: "Attach file".to_string(),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn render(&self) -> String {
        format!(
            r#"<div class="telegram-ui-file-input">
  <label class="file-input-label">
    <input type="file" />
    <span class="file-input-icon">📎</span>
    <span class="file-input-text">{}</span>
  </label>
</div>"#,
            self.label
        )
    }
}
