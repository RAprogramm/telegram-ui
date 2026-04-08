// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Link component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Link {
    href: String,
    text: String,
}

impl Link {
    pub fn new() -> Self {
        Self {
            href: String::new(),
            text: String::new(),
        }
    }

    pub fn href(&self) -> &str {
        &self.href
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Default for Link {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Link: {}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_default() {
        let link = Link::new();
        assert_eq!(link.href(), "");
        assert_eq!(link.text(), "");
    }
}
