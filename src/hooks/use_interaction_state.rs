// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Interaction state tracking for UI elements
//!
//! Provides a state machine for tracking hover, press, and idle states
//! of interactive elements.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

/// Represents the current interaction state of an element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[wasm_bindgen]
pub enum InteractionState {
    /// Element is idle (no interaction)
    #[default]
    Idle,
    /// Element is being hovered over
    Hovered,
    /// Element is currently pressed (mousedown/touchstart)
    Pressed,
}

/// Hooks for managing interaction states
#[wasm_bindgen]
pub struct InteractionHook {
    state: Rc<RefCell<InteractionState>>,
}

#[wasm_bindgen]
impl InteractionHook {
    /// Get the current interaction state
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> InteractionState {
        *self.state.borrow()
    }

    /// Update the interaction state
    pub fn set_state(&self, new_state: InteractionState) {
        *self.state.borrow_mut() = new_state;
    }

    /// Check if the element is currently pressed
    pub fn is_pressed(&self) -> bool {
        matches!(*self.state.borrow(), InteractionState::Pressed)
    }

    /// Check if the element is currently hovered
    pub fn is_hovered(&self) -> bool {
        matches!(*self.state.borrow(), InteractionState::Hovered)
    }

    /// Check if the element is idle
    pub fn is_idle(&self) -> bool {
        matches!(*self.state.borrow(), InteractionState::Idle)
    }
}

#[wasm_bindgen]
impl InteractionHook {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(InteractionState::Idle)),
        }
    }
}

/// Creates a new interaction state manager
///
/// Returns an [`InteractionHook`] that manages hover, press, and idle states.
///
/// # Examples
///
/// ```ignore
/// use telegram_ui::hooks::use_interaction_state;
///
/// let interaction = use_interaction_state();
///
/// // Later in event handlers:
/// interaction.set_state(InteractionState::Hovered);
/// interaction.set_state(InteractionState::Pressed);
/// interaction.set_state(InteractionState::Idle);
/// ```
#[expect(dead_code)]
pub fn use_interaction_state() -> InteractionHook {
    InteractionHook::new()
}
