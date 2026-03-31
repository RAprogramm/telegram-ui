// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Button component (navigation)

#[derive(Clone, Copy, Debug)]
pub enum ButtonSize {
    S,
    M,
    L,
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonMode {
    Filled,
    Outline,
    Plain,
    Gray,
    White,
}

#[derive(Clone, Debug)]
pub struct Button {
    size: ButtonSize,
    mode: ButtonMode,
    label: String,
}

impl Button {
    pub fn new() -> Self {
        Self {
            size: ButtonSize::M,
            mode: ButtonMode::Filled,
            label: String::new(),
        }
    }

    pub fn size(&self) -> ButtonSize {
        self.size
    }

    pub fn mode(&self) -> ButtonMode {
        self.mode
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
