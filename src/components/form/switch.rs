// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Switch (toggle) component

#[derive(Debug, Clone)]
pub struct Switch {
    checked: bool,
    disabled: bool,
}

impl Switch {
    /// Creates a new Switch with default settings
    pub fn new() -> Self {
        Self {
            checked: false,
            disabled: false,
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

    /// Renders the switch to HTML
    pub fn render(&self) -> String {
        let mut html = String::from("<div class=\"telegram-ui-switch\"");

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        if self.checked {
            html.push_str(" active=\"active\"");
        }

        html.push_str(
            ">
            <input type=\"checkbox\" class=\"telegram-ui-switch-input\"",
        );

        if self.checked {
            html.push_str(" checked=\"checked\"");
        }

        if self.disabled {
            html.push_str(" disabled=\"disabled\"");
        }

        html.push_str(
            "/>
            <span class=\"telegram-ui-switch-track\"></span>
            <span class=\"telegram-ui-switch-thumb\"></span>
        </div>",
        );
        html
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_default() {
        let switch = Switch::new();
        assert!(!switch.checked);
        assert!(!switch.disabled);
    }

    #[test]
    fn test_switch_checked() {
        let switch = Switch::new().checked(true);
        let html = switch.render();
        assert!(html.contains("active=\"active\""));
        assert!(html.contains("checked=\"checked\""));
    }

    #[test]
    fn test_switch_disabled() {
        let switch = Switch::new().disabled(true);
        let html = switch.render();
        assert!(html.contains("disabled=\"disabled\""));
    }
}
