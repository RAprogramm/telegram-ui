// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Radio button component

#[derive(Debug, Clone)]
pub struct Radio {
    checked: bool,
    disabled: bool,
    label: Option<String>,
    value: Option<String>,
}

impl Radio {
    /// Creates a new Radio with default settings
    pub fn new() -> Self {
        Self {
            checked: false,
            disabled: false,
            label: None,
            value: None,
        }
    }

    /// Sets the checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Renders the radio button to HTML
    pub fn render(&self) -> String {
        let mut html = String::from("<label class=\"telegram-ui-radio\"");

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        html.push_str(
            ">
            <input type=\"radio\"",
        );

        if let Some(ref value) = self.value {
            html.push_str(&format!(" value=\"{}\"", value));
        }

        if self.checked {
            html.push_str(" checked=\"checked\"");
        }

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        html.push_str("/>");

        if let Some(ref label) = self.label {
            html.push_str(&format!(
                "<span class=\"telegram-ui-radio-label\">{}</span>",
                label
            ));
        }

        html.push_str("</label>");
        html
    }
}

impl Default for Radio {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_default() {
        let radio = Radio::new();
        assert!(!radio.checked);
        assert!(!radio.disabled);
        assert!(radio.label.is_none());
    }

    #[test]
    fn test_radio_checked() {
        let radio = Radio::new().checked(true);
        let html = radio.render();
        assert!(html.contains("checked=\"checked\""));
    }

    #[test]
    fn test_radio_with_value() {
        let radio = Radio::new().value("option1");
        let html = radio.render();
        assert!(html.contains("value=\"option1\""));
    }

    #[test]
    fn test_radio_with_label() {
        let radio = Radio::new().label("Option 1");
        let html = radio.render();
        assert!(html.contains("Option 1"));
    }
}
