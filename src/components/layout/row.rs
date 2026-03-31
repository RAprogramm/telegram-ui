// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Row component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Row {
    children: Vec<String>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Row")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_default() {
        let row = Row::new();
        assert!(row.children().is_empty());
    }
}
