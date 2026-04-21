// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Timeout management hook for deferred execution
//!
//! Provides a professional wrapper around JavaScript setTimeout/clearTimeout
//! for Rust WebAssembly applications.

use std::cell::RefCell;

use wasm_bindgen::prelude::*;

/// Hook that manages timeout execution
#[expect(dead_code)]
pub struct TimeoutHook {
    active_handle: RefCell<Option<i32>>,
    closure_holder: RefCell<Option<Closure<dyn FnMut()>>>,
}

impl TimeoutHook {
    /// Check if there's an active timeout
    #[expect(dead_code)]
    pub fn has_active(&self) -> bool {
        self.active_handle.borrow().is_some()
    }

    /// Set a timeout that will execute after the specified delay
    #[expect(dead_code)]
    pub fn set(&self, callback: Closure<dyn FnMut()>, delay: u32) -> i32 {
        self.clear_current();

        let window = web_sys::window().expect("no global `window` exists");
        let handle_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                delay as i32,
            )
            .expect("failed to set timeout");

        *self.active_handle.borrow_mut() = Some(handle_id);
        *self.closure_holder.borrow_mut() = Some(callback);

        handle_id
    }

    /// Clear the currently active timeout
    pub fn clear_current(&self) {
        if let Some(handle) = self.active_handle.borrow_mut().take()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(handle);
        }
        *self.closure_holder.borrow_mut() = None;
    }

    /// Clear a specific timeout by ID
    #[expect(dead_code)]
    pub fn clear(&self, timeout_id: i32) {
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(timeout_id);
        }
    }

    /// Clear all timeouts (including current)
    #[expect(dead_code)]
    pub fn clear_all(&self) {
        self.clear_current();
    }

    /// Check if a specific timeout ID is currently active
    #[expect(dead_code)]
    pub fn is_active(&self, timeout_id: i32) -> bool {
        let current = self.active_handle.borrow();
        current.is_some_and(|h| h == timeout_id)
    }

    /// Get the currently active timeout ID
    #[expect(dead_code)]
    pub fn active_id(&self) -> i32 {
        (*self.active_handle.borrow()).unwrap_or(-1)
    }
}

impl TimeoutHook {
    /// Creates a new timeout manager
    #[expect(dead_code)]
    pub fn new() -> Self {
        Self {
            active_handle: RefCell::new(None),
            closure_holder: RefCell::new(None),
        }
    }
}

impl Default for TimeoutHook {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a new timeout manager
#[expect(dead_code)]
pub fn use_timeout() -> TimeoutHook {
    TimeoutHook::new()
}
