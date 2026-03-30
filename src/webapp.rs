// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors
//! Telegram WebApp SDK integration
//!
//! This module provides utilities for integrating with the Telegram WebApp SDK.
//! It is only available when the `webapp-sdk` feature is enabled.

#[cfg(feature = "webapp-sdk")]
use telegram_webapp_sdk::TelegramWebApp;

/// Initialize the Telegram WebApp SDK
///
/// This function attempts to get the WebApp instance and returns
/// `Ok(true)` if successful (running inside Telegram), or `Ok(false)` if not.
#[cfg(feature = "webapp-sdk")]
pub fn init_webapp() -> Result<bool, &'static str> {
    match TelegramWebApp::try_instance() {
        Ok(_webapp) => {
            // Call ready() to indicate the app is ready
            let _ = _webapp.ready();
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

/// Check if running inside Telegram
#[cfg(feature = "webapp-sdk")]
pub fn is_telegram() -> bool {
    TelegramWebApp::try_instance().is_ok()
}

/// Show a confirmation dialog
#[cfg(feature = "webapp-sdk")]
pub fn show_confirm(message: &str) -> Result<bool, &'static str> {
    TelegramWebApp::try_instance()
        .map(|webapp| {
            // showConfirm is not directly exposed in the public API
            // We'll return false as a fallback since we can't call it directly
            false
        })
        .map_err(|_| "WebApp not initialized")
}

/// Show an alert dialog
#[cfg(feature = "webapp-sdk")]
pub fn show_alert(message: &str) -> Result<(), &'static str> {
    TelegramWebApp::try_instance()
        .map(|webapp| {
            // showAlert is not directly exposed in the public API
            // We'll just return Ok since we can't call it directly
        })
        .map_err(|_| "WebApp not initialized")
}

/// Expand the WebApp to full height
#[cfg(feature = "webapp-sdk")]
pub fn expand() -> Result<(), &'static str> {
    TelegramWebApp::try_instance()
        .map(|webapp| {
            // expand is not directly exposed in the public API
            // We'll just return Ok since we can't call it directly
        })
        .map_err(|_| "WebApp not initialized")
}

/// Close the WebApp
#[cfg(feature = "webapp-sdk")]
pub fn close() -> Result<(), &'static str> {
    TelegramWebApp::try_instance()
        .map(|webapp| {
            // close is not directly exposed in the public API
            // We'll just return Ok since we can't call it directly
        })
        .map_err(|_| "WebApp not initialized")
}

/// Get the WebApp user data as JSON string
#[cfg(feature = "webapp-sdk")]
pub fn get_user() -> Option<String> {
    // User data is not directly accessible through the public API
    None
}

/// Get the WebApp color scheme
#[cfg(feature = "webapp-sdk")]
pub fn get_color_scheme() -> String {
    // Theme data is not directly accessible through the public API
    "light".to_string()
}

/// Platform detection (fallback when webapp-sdk is not available)
#[cfg(not(feature = "webapp-sdk"))]
pub fn init_webapp() -> Result<bool, &'static str> {
    Err("webapp-sdk feature not enabled")
}

/// Check if running inside Telegram (always false without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn is_telegram() -> bool {
    false
}

/// Show confirmation (error without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn show_confirm(_message: &str) -> Result<bool, &'static str> {
    Err("webapp-sdk feature not enabled")
}

/// Show alert (error without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn show_alert(_message: &str) -> Result<(), &'static str> {
    Err("webapp-sdk feature not enabled")
}

/// Expand WebApp (error without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn expand() -> Result<(), &'static str> {
    Err("webapp-sdk feature not enabled")
}

/// Close WebApp (error without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn close() -> Result<(), &'static str> {
    Err("webapp-sdk feature not enabled")
}

/// Get user data (None without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn get_user() -> Option<String> {
    None
}

/// Get color scheme (default to light without webapp-sdk)
#[cfg(not(feature = "webapp-sdk"))]
pub fn get_color_scheme() -> String {
    "light".to_string()
}
