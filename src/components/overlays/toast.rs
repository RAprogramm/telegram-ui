// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Toast component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Toast {
    message:  String,
    duration: u64
}

impl Toast {
    pub fn new() -> Self {
        Self {
            message:  String::new(),
            duration: 3000
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Toast: {}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_default() {
        let toast = Toast::new();
        assert_eq!(toast.message(), "");
        assert_eq!(toast.duration(), 3000);
    }
}
