// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Column component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Column {
    children: Vec<String>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Column")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_default() {
        let column = Column::new();
        assert!(column.children().is_empty());
    }
}
