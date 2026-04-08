// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Checkbox component

#[derive(Debug, Clone)]
pub struct Checkbox {
    checked: bool,
    disabled: bool,
    label: Option<String>,
}

impl Checkbox {
    /// Creates a new Checkbox with default settings
    pub fn new() -> Self {
        Self {
            checked: false,
            disabled: false,
            label: None,
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

    /// Renders the checkbox to HTML
    pub fn render(&self) -> String {
        let mut html = String::from("<label class=\"telegram-ui-checkbox\"");

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        html.push_str(
            ">
            <input type=\"checkbox\"",
        );

        if self.checked {
            html.push_str(" checked=\"checked\"");
        }

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        html.push_str("/>");

        if let Some(ref label) = self.label {
            html.push_str(&format!(
                "<span class=\"telegram-ui-checkbox-label\">{}</span>",
                label
            ));
        }

        html.push_str("</label>");
        html
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_default() {
        let checkbox = Checkbox::new();
        assert!(!checkbox.checked);
        assert!(!checkbox.disabled);
        assert!(checkbox.label.is_none());
    }

    #[test]
    fn test_checkbox_checked() {
        let checkbox = Checkbox::new().checked(true);
        let html = checkbox.render();
        assert!(html.contains("checked=\"checked\""));
    }

    #[test]
    fn test_checkbox_disabled() {
        let checkbox = Checkbox::new().disabled(true);
        let html = checkbox.render();
        assert!(html.contains("disabled=\"disabled\""));
    }

    #[test]
    fn test_checkbox_with_label() {
        let checkbox = Checkbox::new().label("Accept terms");
        let html = checkbox.render();
        assert!(html.contains("Accept terms"));
    }
}
