//! Form components for Telegram UI

use std::fmt;

/// Input component
#[derive(Debug, Clone)]
pub struct Input {
    placeholder: Option<String>,
    value: Option<String>,
    disabled: bool,
    readonly: bool,
    required: bool,
    input_type: String,
}

impl Input {
    /// Creates a new Input with default settings
    pub fn new() -> Self {
        Self {
            placeholder: None,
            value: None,
            disabled: false,
            readonly: false,
            required: false,
            input_type: "text".to_string(),
        }
    }

    /// Sets the input placeholder
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }

    /// Sets the input value
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Sets whether the input is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the input is readonly
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Sets whether the input is required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets the input type
    pub fn input_type(mut self, input_type: &str) -> Self {
        self.input_type = input_type.to_string();
        self
    }

    /// Returns the input placeholder
    pub fn get_placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    /// Returns the input value
    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns whether the input is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns whether the input is readonly
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Returns whether the input is required
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the input type
    pub fn get_input_type(&self) -> &str {
        &self.input_type
    }

    /// Render the input as HTML string
    pub fn render(&self) -> String {
        let mut attributes = Vec::new();

        if let Some(placeholder) = &self.placeholder {
            attributes.push(format!("placeholder=\"{}\"", placeholder));
        }

        if let Some(value) = &self.value {
            attributes.push(format!("value=\"{}\"", value));
        }

        if self.disabled {
            attributes.push("disabled".to_string());
        }

        if self.readonly {
            attributes.push("readonly".to_string());
        }

        if self.required {
            attributes.push("required".to_string());
        }

        let attributes_str = attributes.join(" ");

        format!(
            "<input type=\"{}\" class=\"telegram-ui-input\"{}>",
            self.input_type, attributes_str
        )
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Textarea component
#[derive(Debug, Clone)]
pub struct Textarea {
    placeholder: Option<String>,
    value: Option<String>,
    rows: Option<u32>,
    disabled: bool,
    readonly: bool,
    required: bool,
}

impl Textarea {
    /// Creates a new Textarea with default settings
    pub fn new() -> Self {
        Self {
            placeholder: None,
            value: None,
            rows: None,
            disabled: false,
            readonly: false,
            required: false,
        }
    }

    /// Sets the textarea placeholder
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }

    /// Sets the textarea value
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Sets the number of rows
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    /// Sets whether the textarea is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the textarea is readonly
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Sets whether the textarea is required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Returns the textarea placeholder
    pub fn get_placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    /// Returns the textarea value
    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns the number of rows
    pub fn get_rows(&self) -> Option<u32> {
        self.rows
    }

    /// Returns whether the textarea is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns whether the textarea is readonly
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Returns whether the textarea is required
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Render the textarea as HTML string
    pub fn render(&self) -> String {
        let mut attributes = Vec::new();

        if let Some(placeholder) = &self.placeholder {
            attributes.push(format!("placeholder=\"{}\"", placeholder));
        }

        if let Some(value) = &self.value {
            attributes.push(format!("value=\"{}\"", value));
        }

        if let Some(rows) = self.rows {
            attributes.push(format!("rows=\"{}\"", rows));
        }

        if self.disabled {
            attributes.push("disabled".to_string());
        }

        if self.readonly {
            attributes.push("readonly".to_string());
        }

        if self.required {
            attributes.push("required".to_string());
        }

        let attributes_str = attributes.join(" ");

        format!(
            "<textarea class=\"telegram-ui-textarea\"{}></textarea>",
            attributes_str
        )
    }
}

impl Default for Textarea {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Textarea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Select component
#[derive(Debug, Clone)]
pub struct Select {
    options: Vec<(String, String)>, // (value, label)
    placeholder: Option<String>,
    value: Option<String>,
    disabled: bool,
    multiple: bool,
}

impl Select {
    /// Creates a new Select with default settings
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            placeholder: None,
            value: None,
            disabled: false,
            multiple: false,
        }
    }

    /// Adds an option to the select
    pub fn add_option(mut self, value: &str, label: &str) -> Self {
        self.options.push((value.to_string(), label.to_string()));
        self
    }

    /// Adds multiple options to the select
    pub fn add_options(mut self, options: &[(&str, &str)]) -> Self {
        for (value, label) in options {
            self.options.push((value.to_string(), label.to_string()));
        }
        self
    }

    /// Sets the select placeholder
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self
    }

    /// Sets the selected value
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Sets whether the select is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the select supports multiple selection
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Returns the select options
    pub fn get_options(&self) -> &[(String, String)] {
        &self.options
    }

    /// Returns the select placeholder
    pub fn get_placeholder(&self) -> Option<&str> {
        self.placeholder.as_deref()
    }

    /// Returns the selected value
    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns whether the select is disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Returns whether the select supports multiple selection
    pub fn is_multiple(&self) -> bool {
        self.multiple
    }

    /// Render the select as HTML string
    pub fn render(&self) -> String {
        let mut attributes = Vec::new();

        if let Some(placeholder) = &self.placeholder {
            attributes.push(format!("placeholder=\"{}\"", placeholder));
        }

        if self.disabled {
            attributes.push("disabled".to_string());
        }

        if self.multiple {
            attributes.push("multiple".to_string());
        }

        let attributes_str = attributes.join(" ");

        let options_html: String = self.options
            .iter()
            .map(|(value, label)| {
                let selected = self.value
                    .as_ref()
                    .map(|v| v == value)
                    .unwrap_or_default();

                let selected_attr = if selected { " selected" } else { "" };
                format!(
                    "<option value=\"{}\"{}>{}</option>",
                    value, selected_attr, label
                )
            })
            .collect();

        format!(
            "<select class=\"telegram-ui-select\"{}>{}</select>",
            attributes_str, options_html
        )
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_render() {
        let input = Input::new()
            .placeholder("Enter text")
            .value("default")
            .disabled(true);

        let html = input.render();
        assert!(html.contains("telegram-ui-input"));
        assert!(html.contains("placeholder=\"Enter text\""));
        assert!(html.contains("value=\"default\""));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_textarea_render() {
        let textarea = Textarea::new()
            .placeholder("Enter text")
            .value("default")
            .rows(5);

        let html = textarea.render();
        assert!(html.contains("telegram-ui-textarea"));
        assert!(html.contains("placeholder=\"Enter text\""));
        assert!(html.contains("rows=\"5\""));
    }

    #[test]
    fn test_select_render() {
        let select = Select::new()
            .add_option("1", "Option 1")
            .add_option("2", "Option 2")
            .value("1");

        let html = select.render();
        assert!(html.contains("telegram-ui-select"));
        assert!(html.contains("value=\"1\""));
        assert!(html.contains("Option 1"));
        assert!(html.contains("Option 2"));
    }
}
