// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Container component

#[derive(Clone, Debug)]
pub struct Container {
    padding: String,
}

impl Container {
    pub fn new() -> Self {
        Self {
            padding: "16px".to_string(),
        }
    }

    pub fn padding(&self) -> &str {
        &self.padding
    }
}
