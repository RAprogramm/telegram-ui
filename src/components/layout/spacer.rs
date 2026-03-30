// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Spacer component

#[derive(Clone, Debug)]
pub struct Spacer {
    size: u32,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            size: 16,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
