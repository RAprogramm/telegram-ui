// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Alert component

use std::fmt;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AlertKind {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Alert {
    kind: AlertKind,
    message: String,
}

impl Alert {
    pub fn new() -> Self {
        Self {
            kind: AlertKind::Info,
            message: String::new(),
        }
    }

    pub fn kind(&self) -> &AlertKind {
        &self.kind
    }

    pub fn set_kind(mut self, kind: AlertKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn render(&self) -> String {
        let kind_class = match self.kind {
            AlertKind::Info => "telegram-ui-alert--info",
            AlertKind::Success => "telegram-ui-alert--success",
            AlertKind::Warning => "telegram-ui-alert--warning",
            AlertKind::Error => "telegram-ui-alert--error",
        };
        format!(
            "<div class=\"telegram-ui-alert {}\">{}</div>",
            kind_class, self.message
        )
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Alert: {}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_default() {
        let alert = Alert::new();
        assert_eq!(*alert.kind(), AlertKind::Info);
        assert_eq!(alert.message(), "");
    }
}
