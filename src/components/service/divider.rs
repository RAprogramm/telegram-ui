// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Divider component

use std::fmt;

/// Divider component
#[derive(Clone, Debug)]
pub struct Divider {
    inset: bool
}

impl Divider {
    pub fn new() -> Self {
        Self {
            inset: false
        }
    }

    pub fn with_inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn is_inset(&self) -> bool {
        self.inset
    }

    pub fn render(&self) -> String {
        let class = if self.inset {
            "telegram-ui-divider telegram-ui-divider--inset"
        } else {
            "telegram-ui-divider"
        };

        format!("<div class=\"{}\"></div>", class)
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Divider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Divider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divider_default() {
        let divider = Divider::new();
        assert!(!divider.is_inset());
    }

    #[test]
    fn test_divider_custom() {
        let divider = Divider::new().with_inset(true);
        assert!(divider.is_inset());
    }

    #[test]
    fn test_divider_render() {
        let divider = Divider::new();
        let html = divider.render();
        assert!(html.contains("telegram-ui-divider"));
        assert!(html.contains("</div>"));
    }
}
