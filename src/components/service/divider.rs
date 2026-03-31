// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Divider component

#[derive(Clone, Debug)]
pub struct Divider {
    thickness: u32,
    color: String,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            thickness: 1,
            color: "#e0e0e0".to_string(),
        }
    }

    pub fn thickness(&self) -> u32 {
        self.thickness
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}
