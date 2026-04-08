// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Progress component

use std::fmt;

#[derive(Clone, Debug)]
pub struct Progress {
    value: f64,
    max:   f64
}

impl Progress {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            max:   100.0
        }
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", (self.value / self.max) * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_default() {
        let progress = Progress::new();
        assert_eq!(progress.value(), 0.0);
        assert_eq!(progress.max(), 100.0);
    }

    #[test]
    fn test_progress_custom() {
        let progress = Progress::new().with_value(50.0).with_max(200.0);
        assert_eq!(progress.value(), 50.0);
        assert_eq!(progress.max(), 200.0);
    }
}
