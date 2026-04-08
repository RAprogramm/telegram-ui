// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Skeleton component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Skeleton {
    width:  String,
    height: String
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            width:  "100%".to_string(),
            height: "100px".to_string()
        }
    }

    pub fn width(&self) -> &str {
        &self.width
    }

    pub fn height(&self) -> &str {
        &self.height
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Skeleton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Skeleton")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_default() {
        let skeleton = Skeleton::new();
        assert_eq!(skeleton.width(), "100%");
        assert_eq!(skeleton.height(), "100px");
    }
}
