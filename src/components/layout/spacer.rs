// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Spacer component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Spacer {
    size: u32
}

impl Spacer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: 16
        }
    }

    #[must_use]
    pub const fn size(&self) -> u32 {
        self.size
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Spacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Spacer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacer_default() {
        let spacer = Spacer::new();
        assert_eq!(spacer.size(), 16);
    }
}
