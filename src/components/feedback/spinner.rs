// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Spinner component

#[derive(Clone, Copy, Debug)]
pub enum SpinnerSize {
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

    pub fn size(&self) -> SpinnerSize {
        self.size
    }
}
