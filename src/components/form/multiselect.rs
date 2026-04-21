#[derive(Debug, Clone)]
pub struct Multiselect {
    placeholder: String,
    options:     Vec<(String, String)>,
    selected:    Vec<String>
}

impl Multiselect {
    #[must_use]
    pub fn new() -> Self {
        Self {
            placeholder: "Select options".to_string(),
            options:     Vec::new(),
            selected:    Vec::new()
        }
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    #[must_use]
    pub fn add_option(mut self, value: &str, label: &str) -> Self {
        self.options.push((value.to_string(), label.to_string()));
        self
    }

    #[must_use]
    pub fn selected(mut self, selected: Vec<&str>) -> Self {
        self.selected = selected
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let options_html: String = self
            .options
            .iter()
            .map(|(value, label)| {
                let _selected = if self.selected.contains(value) {
                    "selected"
                } else {
                    ""
                };
                format!("<option class=\"multiselect-option\" value=\"{value}\">{label}</option>")
            })
            .collect();

        let placeholder = if self.selected.is_empty() {
            format!(
                "<span class=\"multiselect-placeholder\">{}</span>",
                self.placeholder
            )
        } else {
            String::new()
        };

        format!(
            "<div class=\"telegram-ui-multiselect\">{placeholder}<select class=\"multiselect-select\">{options_html}</select></div>"
        )
    }
}

impl Default for Multiselect {
    fn default() -> Self {
        Self::new()
    }
}
