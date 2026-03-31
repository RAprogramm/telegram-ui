// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Badge component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Badge {
    value: Option<i32>,
    dot: bool,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            value: None,
            dot: false,
        }
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<i32> {
        self.value
    }

    pub fn with_dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    pub fn is_dot(&self) -> bool {
        self.dot
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(v) => write!(f, "{}", v),
            None => write!(f, "Badge"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_default() {
        let badge = Badge::new();
        assert!(badge.value().is_none());
        assert!(!badge.is_dot());
    }

    #[test]
    fn test_badge_custom() {
        let badge = Badge::new().with_value(5).with_dot(true);
        assert_eq!(badge.value(), Some(5));
        assert!(badge.is_dot());
    }
}
