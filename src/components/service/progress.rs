// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Progress component

#[derive(Clone, Debug)]
pub struct Progress {
    value: f64,
    max: f64,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            max: 100.0,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}
