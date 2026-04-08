// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Backdrop component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Backdrop {
    visible: bool
}

impl Backdrop {
    pub fn new() -> Self {
        Self {
            visible: false
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl Default for Backdrop {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Backdrop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Backdrop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backdrop_default() {
        let backdrop = Backdrop::new();
        assert!(!backdrop.visible());
    }
}
