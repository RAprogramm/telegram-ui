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

    pub fn message(&self) -> &str {
        &self.message
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
