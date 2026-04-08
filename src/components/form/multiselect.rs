#[derive(Debug, Clone)]
pub struct Multiselect {
    placeholder: String,
    options: Vec<(String, String)>,
    selected: Vec<String>,
}

impl Multiselect {
    pub fn new() -> Self {
        Self {
            placeholder: "Select options".to_string(),
            options: Vec::new(),
            selected: Vec::new(),
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn add_option(mut self, value: &str, label: &str) -> Self {
        self.options.push((value.to_string(), label.to_string()));
        self
    }

    pub fn selected(mut self, selected: Vec<&str>) -> Self {
        self.selected = selected.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn render(&self) -> String {
        let options_html: String = self
            .options
            .iter()
            .map(|(value, label)| {
                let selected = if self.selected.contains(value) {
                    "selected"
                } else {
                    ""
                };
                format!(
                    "<option class=\"multiselect-option\" value=\"{}\">{}</option>",
                    value, label
                )
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
            "<div class=\"telegram-ui-multiselect\">{}<select class=\"multiselect-select\">{}</select></div>",
            placeholder, options_html
        )
    }
}
