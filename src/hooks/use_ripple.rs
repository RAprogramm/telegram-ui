// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Ripple effect management for interactive elements
//!
//! Provides sophisticated state management for Material Design-style
//! ripple effects on click/tap interactions.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{closure::Closure, prelude::*};

/// Represents the position and state of a ripple effect
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RippleState {
    active: bool,
    x: f64,
    y: f64,
    radius: f64,
}

#[wasm_bindgen]
impl RippleState {
    /// Check if ripple is currently active
    #[wasm_bindgen(getter)]
    pub fn active(&self) -> bool {
        self.active
    }

    /// Get the X position of the ripple center
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the Y position of the ripple center
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the ripple radius
    #[wasm_bindgen(getter)]
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for RippleState {
    fn default() -> Self {
        RippleState {
            active: false,
            x: 0.0,
            y: 0.0,
            radius: 0.0,
        }
    }
}

/// Handles the ripple effect state and cleanup
#[wasm_bindgen]
pub struct RippleEffect {
    state: Rc<RefCell<RippleState>>,
    cleanup_timer: RefCell<Option<i32>>,
    ripples: Rc<RefCell<Vec<RippleState>>>,
    cleanup_closures: RefCell<Vec<Closure<dyn FnMut()>>>,
}

#[wasm_bindgen]
impl RippleEffect {
    /// Get the current ripple state
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> RippleState {
        self.state.borrow().clone()
    }

    /// Start a ripple effect at the given coordinates
    pub fn start(&self, x: f64, y: f64, element: &web_sys::Element) -> RippleState {
        let width = element.client_width() as f64;
        let height = element.client_height() as f64;
        let radius = (width.powi(2) + height.powi(2)).sqrt();

        let new_state = RippleState {
            active: true,
            x,
            y,
            radius,
        };

        *self.state.borrow_mut() = new_state.clone();

        // Schedule ripple removal after animation completes
        let window = web_sys::window().expect("no global `window` exists");

        let cleanup_closure = Closure::new(|| {});
        let cleanup_timer_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cleanup_closure.as_ref().unchecked_ref(),
                500,
            )
            .expect("failed to set timeout");

        self.cleanup_closures.borrow_mut().push(cleanup_closure);

        *self.cleanup_timer.borrow_mut() = Some(cleanup_timer_id);

        new_state
    }

    /// End the current ripple effect
    pub fn end(&self) {
        *self.state.borrow_mut() = RippleState::default();
        if let Some(timer_id) = self.cleanup_timer.borrow_mut().take()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(timer_id);
        }
    }

    /// Clear all active ripples
    pub fn clear(&self) {
        *self.state.borrow_mut() = RippleState::default();
        self.ripples.borrow_mut().clear();
        if let Some(timer_id) = self.cleanup_timer.borrow_mut().take()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(timer_id);
        }
        self.cleanup_closures.borrow_mut().clear();
    }

    /// Get list of active ripples
    pub fn active_ripples(&self) -> Vec<RippleState> {
        self.ripples.borrow().clone()
    }

    /// Add a ripple to the list (for multiple ripples)
    pub fn add_ripple(&self, ripple: RippleState) {
        self.ripples.borrow_mut().push(ripple);
    }
}

#[wasm_bindgen]
impl RippleEffect {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(RippleState::default())),
            cleanup_timer: RefCell::new(None),
            ripples: Rc::new(RefCell::new(Vec::new())),
            cleanup_closures: RefCell::new(Vec::new()),
        }
    }
}

/// Creates a new ripple effect manager
#[expect(dead_code)]
pub fn use_ripple() -> RippleEffect {
    RippleEffect::new()
}

/// Creates an event handler for starting ripple effects
#[expect(dead_code)]
pub fn create_ripple_handler(
    ripple: RippleEffect,
    element: web_sys::Element,
) -> Closure<dyn FnMut(web_sys::Event)> {
    Closure::new(move |event: web_sys::Event| {
        let (client_x, client_y) =
            if let Some(mouse_event) = event.dyn_ref::<web_sys::MouseEvent>() {
                (mouse_event.client_x(), mouse_event.client_y())
            } else if let Ok(touch_event) = event.dyn_into::<web_sys::TouchEvent>() {
                // TouchEvent.touches is not available in minimal web-sys
                let _ = touch_event;
                return;
            } else {
                return;
            };

        // Use client coordinates directly for the ripple center
        let x = client_x as f64;
        let y = client_y as f64;

        let _ = ripple.start(x, y, &element);
    })
}

/// Creates an event handler for ending ripple effects
#[expect(dead_code)]
pub fn create_ripple_end_handler(ripple: RippleEffect) -> Closure<dyn FnMut(web_sys::Event)> {
    Closure::new(move |_event: web_sys::Event| {
        ripple.end();
    })
}
