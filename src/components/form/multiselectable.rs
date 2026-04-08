#[derive(Debug, Clone)]
pub struct Multiselectable {
    value: String,
    label: String,
    checked: bool,
    disabled: bool,
}

impl Multiselectable {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            label: String::new(),
            checked: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn render(&self) -> String {
        let disabled_class = if self.disabled {
            "multiselectable--disabled"
        } else {
            ""
        };

        format!(
            r#"<label class="telegram-ui-multiselectable {}">
  <input type="checkbox" class="multiselectable-input" value="{}" {} {} />
  <span class="multiselectable-icon">☐</span>
  <span class="multiselectable-checked-icon">☑</span>
  <span class="multiselectable-label">{}</span>
</label>"#,
            disabled_class,
            self.value,
            if self.checked { "checked" } else { "" },
            if self.disabled { "disabled" } else { "" },
            self.label
        )
    }
}
