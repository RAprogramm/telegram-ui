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
/// Returns `Ok(true)` if the app is running inside Telegram,
/// `Ok(false)` if running outside Telegram, or `Err` on error.
#[cfg(feature = "webapp-sdk")]
pub fn init_webapp() -> Result<bool, String> {
    TelegramWebApp::instance()
        .map(|_| true)
        .ok_or_else(|| "Telegram WebApp not available".to_string())
}

/// Show an alert dialog in Telegram WebApp
#[allow(dead_code)]
pub fn show_alert(_message: &str) -> Result<(), String> {
    #[cfg(feature = "webapp-sdk")]
    {
        TelegramWebApp::instance()
            .ok_or_else(|| "Failed to get webapp instance".to_string())
            .and_then(|webapp| {
                webapp.show_alert(_message)
                    .map_err(|_| "Failed to show alert".to_string())
            })
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        // Fallback: no-op
        Ok(())
    }
}

/// Show a confirm dialog in Telegram WebApp
#[allow(dead_code)]
pub fn show_confirm(_message: &str) -> Result<bool, String> {
    #[cfg(feature = "webapp-sdk")]
    {
        TelegramWebApp::instance()
            .ok_or_else(|| "Failed to get webapp instance".to_string())
            .and_then(|webapp| {
                // show_confirm requires a callback, so we use a simple approach
                // In a real app, you'd pass a proper callback
                webapp.show_confirm(_message, |_| {})
                    .map(|_| true)
                    .map_err(|_| "Failed to show confirm".to_string())
            })
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        // Fallback: return true
        Ok(true)
    }
}

/// Expand the WebApp to full height
#[allow(dead_code)]
pub fn expand() -> Result<(), String> {
    #[cfg(feature = "webapp-sdk")]
    {
        TelegramWebApp::instance()
            .ok_or_else(|| "Failed to get webapp instance".to_string())
            .and_then(|webapp| {
                webapp.expand()
                    .map_err(|_| "Failed to expand".to_string())
            })
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        // Fallback: no-op
        Ok(())
    }
}

/// Close the WebApp
#[allow(dead_code)]
pub fn close() -> Result<(), String> {
    #[cfg(feature = "webapp-sdk")]
    {
        TelegramWebApp::instance()
            .ok_or_else(|| "Failed to get webapp instance".to_string())
            .and_then(|webapp| {
                webapp.close()
                    .map_err(|_| "Failed to close".to_string())
            })
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        // Fallback: no-op
        Ok(())
    }
}

/// Get the current user info
#[allow(dead_code)]
pub fn get_user() -> Option<String> {
    #[cfg(feature = "webapp-sdk")]
    {
        // The SDK doesn't expose a direct user getter
        // This is a placeholder for future expansion
        None
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        None
    }
}

/// Get the current color scheme
#[allow(dead_code)]
pub fn get_color_scheme() -> String {
    #[cfg(feature = "webapp-sdk")]
    {
        // The SDK doesn't expose a direct color scheme getter
        // This is a placeholder for future expansion
        "unknown".to_string()
    }
    #[cfg(not(feature = "webapp-sdk"))]
    {
        "unknown".to_string()
    }
}
