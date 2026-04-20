// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Container component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Container {
    children: Vec<String>,
    padding:  u32
}

impl Container {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            padding:  16
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Container")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_default() {
        let container = Container::new();
        assert!(container.children().is_empty());
        assert_eq!(container.padding(), 16);
    }
}
