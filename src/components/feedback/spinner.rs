// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Spinner component

use std::fmt;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum SpinnerSize {
    #[default]
    S,
    M,
    L,
}

#[derive(Clone, Debug)]
pub struct Spinner {
    size: SpinnerSize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: SpinnerSize::M,
        }
    }

    pub fn size(&self) -> &SpinnerSize {
        &self.size
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Spinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Spinner")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_default() {
        let spinner = Spinner::new();
        assert_eq!(*spinner.size(), SpinnerSize::M);
    }
}
