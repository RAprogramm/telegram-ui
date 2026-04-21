// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Spinner component

use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    #[default]
    S,
    M,
    L
}

#[derive(Clone, Debug)]
pub struct Spinner {
    size: SpinnerSize
}

impl Spinner {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: SpinnerSize::M
        }
    }

    #[must_use]
    pub const fn size(&self) -> &SpinnerSize {
        &self.size
    }

    #[must_use]
    pub const fn set_size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-spinner telegram-ui-spinner--{}\"></div>",
            match self.size {
                SpinnerSize::S => "s",
                SpinnerSize::M => "m",
                SpinnerSize::L => "l"
            }
        )
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
