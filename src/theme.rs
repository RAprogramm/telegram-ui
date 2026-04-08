// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Theme support for Telegram Mini Apps.
//!
//! This module provides theme management for Telegram UI components, including:
//! - `Theme` enum for light/dark/auto theme selection
//! - `ThemeContext` struct for holding Telegram color scheme values
//!
//! # Example
//!
//! ```
//! use telegram_ui::{AppRoot, Theme, ThemeContext};
//!
//! let theme_context = ThemeContext::default()
//!     .with_bg_color("#ffffff")
//!     .with_text_color("#000000");
//!
//! let app_root = AppRoot::new()
//!     .theme(Theme::Auto)
//!     .theme_context(theme_context);
//! ```

use std::fmt;

/// Theme mode for Telegram UI components.
///
/// This enum represents the three theme modes supported by Telegram:
/// - `Light` - Light theme with white backgrounds and dark text
/// - `Dark` - Dark theme with dark backgrounds and light text
/// - `Auto` - Automatically switches between light and dark based on
///   system/user preference
///
/// # Example
///
/// ```
/// use telegram_ui::Theme;
///
/// let theme = Theme::Auto;
/// let css_class = theme.css_class(); // Returns "tg-theme-auto"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Light theme - white backgrounds, dark text
    Light,
    /// Dark theme - dark backgrounds, light text
    Dark,
    /// Auto theme - follows system/user preference
    #[default]
    Auto
}

impl Theme {
    /// Returns the CSS class name for this theme.
    ///
    /// # Returns
    ///
    /// A string slice representing the CSS class name:
    /// - `"tg-theme-light"` for `Theme::Light`
    /// - `"tg-theme-dark"` for `Theme::Dark`
    /// - `"tg-theme-auto"` for `Theme::Auto`
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::Theme;
    ///
    /// assert_eq!(Theme::Light.css_class(), "tg-theme-light");
    /// assert_eq!(Theme::Dark.css_class(), "tg-theme-dark");
    /// assert_eq!(Theme::Auto.css_class(), "tg-theme-auto");
    /// ```
    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "tg-theme-light",
            Theme::Dark => "tg-theme-dark",
            Theme::Auto => "tg-theme-auto"
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
            Theme::Auto => write!(f, "auto")
        }
    }
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            "auto" => Ok(Theme::Auto),
            _ => Err(format!("Invalid theme: {}", s))
        }
    }
}

/// Theme context containing Telegram color scheme values.
///
/// This struct holds all the color parameters provided by Telegram's WebApp
/// API. These colors can be applied to the document root or used directly in
/// component styling.
///
/// # Color Parameters
///
/// All colors should be in `#RRGGBB` format. The following parameters are
/// supported:
///
/// | Parameter | CSS Variable | Description |
/// | `bg_color` | `--tg-theme-bg-color` | Primary background color |
/// | `text_color` | `--tg-theme-text-color` | Primary text color |
/// | `hint_color` | `--tg-theme-hint-color` | Hint text color |
/// | `link_color` | `--tg-theme-link-color` | Link color |
/// | `button_color` | `--tg-theme-button-color` | Button background color |
/// | `button_text_color` | `--tg-theme-button-text-color` | Button text color |
/// | `secondary_bg_color` | `--tg-theme-secondary-bg-color` | Secondary background color |
/// | `header_bg_color` | `--tg-theme-header-bg-color` | Header background color |
/// | `bottom_bar_bg_color` | `--tg-theme-bottom-bar-bg-color` | Bottom bar background color |
/// | `accent_text_color` | `--tg-theme-accent-text-color` | Accent text color |
/// | `section_bg_color` | `--tg-theme-section-bg-color` | Section background color |
/// | `section_header_text_color` | `--tg-theme-section-header-text-color` | Section header text color |
/// | `section_separator_color` | `--tg-theme-section-separator-color` | Section separator color |
/// | `subtitle_text_color` | `--tg-theme-subtitle-text-color` | Subtitle text color |
/// | `destructive_text_color` | `--tg-theme-destructive-text-color` | Destructive action text color |
///
/// # Example
///
/// ```
/// use telegram_ui::ThemeContext;
///
/// let theme_context = ThemeContext::default()
///     .with_bg_color("#ffffff")
///     .with_text_color("#000000")
///     .with_button_color("#3390ec");
///
/// // Apply to document root
/// theme_context.apply_to_root().ok();
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThemeContext {
    /// Primary background color
    pub bg_color:                  Option<String>,
    /// Primary text color
    pub text_color:                Option<String>,
    /// Hint text color
    pub hint_color:                Option<String>,
    /// Link color
    pub link_color:                Option<String>,
    /// Button background color
    pub button_color:              Option<String>,
    /// Button text color
    pub button_text_color:         Option<String>,
    /// Secondary background color
    pub secondary_bg_color:        Option<String>,
    /// Header background color
    pub header_bg_color:           Option<String>,
    /// Bottom bar background color
    pub bottom_bar_bg_color:       Option<String>,
    /// Accent text color
    pub accent_text_color:         Option<String>,
    /// Section background color
    pub section_bg_color:          Option<String>,
    /// Section header text color
    pub section_header_text_color: Option<String>,
    /// Section separator color
    pub section_separator_color:   Option<String>,
    /// Subtitle text color
    pub subtitle_text_color:       Option<String>,
    /// Destructive action text color
    pub destructive_text_color:    Option<String>
}

impl ThemeContext {
    /// Creates a new empty `ThemeContext`.
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new();
    /// assert!(theme_context.bg_color.is_none());
    /// ```
    pub fn new() -> Self {
        ThemeContext::default()
    }

    /// Sets the primary background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_bg_color("#ffffff");
    /// assert_eq!(theme_context.bg_color, Some("#ffffff".to_string()));
    /// ```
    pub fn with_bg_color(mut self, color: &str) -> Self {
        self.bg_color = Some(color.to_string());
        self
    }

    /// Sets the primary text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_text_color("#000000");
    /// assert_eq!(theme_context.text_color, Some("#000000".to_string()));
    /// ```
    pub fn with_text_color(mut self, color: &str) -> Self {
        self.text_color = Some(color.to_string());
        self
    }

    /// Sets the hint text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_hint_color("#999999");
    /// assert_eq!(theme_context.hint_color, Some("#999999".to_string()));
    /// ```
    pub fn with_hint_color(mut self, color: &str) -> Self {
        self.hint_color = Some(color.to_string());
        self
    }

    /// Sets the link color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_link_color("#3390ec");
    /// assert_eq!(theme_context.link_color, Some("#3390ec".to_string()));
    /// ```
    pub fn with_link_color(mut self, color: &str) -> Self {
        self.link_color = Some(color.to_string());
        self
    }

    /// Sets the button background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_button_color("#3390ec");
    /// assert_eq!(theme_context.button_color, Some("#3390ec".to_string()));
    /// ```
    pub fn with_button_color(mut self, color: &str) -> Self {
        self.button_color = Some(color.to_string());
        self
    }

    /// Sets the button text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_button_text_color("#ffffff");
    /// assert_eq!(theme_context.button_text_color, Some("#ffffff".to_string()));
    /// ```
    pub fn with_button_text_color(mut self, color: &str) -> Self {
        self.button_text_color = Some(color.to_string());
        self
    }

    /// Sets the secondary background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_secondary_bg_color("#f4f4f5");
    /// assert_eq!(
    ///     theme_context.secondary_bg_color,
    ///     Some("#f4f4f5".to_string())
    /// );
    /// ```
    pub fn with_secondary_bg_color(mut self, color: &str) -> Self {
        self.secondary_bg_color = Some(color.to_string());
        self
    }

    /// Sets the header background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_header_bg_color("#ffffff");
    /// assert_eq!(theme_context.header_bg_color, Some("#ffffff".to_string()));
    /// ```
    pub fn with_header_bg_color(mut self, color: &str) -> Self {
        self.header_bg_color = Some(color.to_string());
        self
    }

    /// Sets the bottom bar background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_bottom_bar_bg_color("#ffffff");
    /// assert_eq!(
    ///     theme_context.bottom_bar_bg_color,
    ///     Some("#ffffff".to_string())
    /// );
    /// ```
    pub fn with_bottom_bar_bg_color(mut self, color: &str) -> Self {
        self.bottom_bar_bg_color = Some(color.to_string());
        self
    }

    /// Sets the accent text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_accent_text_color("#3390ec");
    /// assert_eq!(theme_context.accent_text_color, Some("#3390ec".to_string()));
    /// ```
    pub fn with_accent_text_color(mut self, color: &str) -> Self {
        self.accent_text_color = Some(color.to_string());
        self
    }

    /// Sets the section background color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_section_bg_color("#ffffff");
    /// assert_eq!(theme_context.section_bg_color, Some("#ffffff".to_string()));
    /// ```
    pub fn with_section_bg_color(mut self, color: &str) -> Self {
        self.section_bg_color = Some(color.to_string());
        self
    }

    /// Sets the section header text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_section_header_text_color("#707579");
    /// assert_eq!(
    ///     theme_context.section_header_text_color,
    ///     Some("#707579".to_string())
    /// );
    /// ```
    pub fn with_section_header_text_color(mut self, color: &str) -> Self {
        self.section_header_text_color = Some(color.to_string());
        self
    }

    /// Sets the section separator color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_section_separator_color("#c8c7cc");
    /// assert_eq!(
    ///     theme_context.section_separator_color,
    ///     Some("#c8c7cc".to_string())
    /// );
    /// ```
    pub fn with_section_separator_color(mut self, color: &str) -> Self {
        self.section_separator_color = Some(color.to_string());
        self
    }

    /// Sets the subtitle text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_subtitle_text_color("#707579");
    /// assert_eq!(
    ///     theme_context.subtitle_text_color,
    ///     Some("#707579".to_string())
    /// );
    /// ```
    pub fn with_subtitle_text_color(mut self, color: &str) -> Self {
        self.subtitle_text_color = Some(color.to_string());
        self
    }

    /// Sets the destructive action text color.
    ///
    /// # Arguments
    ///
    /// * `color` - Color value in `#RRGGBB` format
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new().with_destructive_text_color("#e53935");
    /// assert_eq!(
    ///     theme_context.destructive_text_color,
    ///     Some("#e53935".to_string())
    /// );
    /// ```
    pub fn with_destructive_text_color(mut self, color: &str) -> Self {
        self.destructive_text_color = Some(color.to_string());
        self
    }

    /// Converts all theme colors into a map of CSS custom properties.
    ///
    /// # Returns
    ///
    /// A `HashMap` where each key is a CSS variable name like
    /// `"--tg-theme-bg-color"`, and the corresponding value is the `#RRGGBB`
    /// color string.
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new()
    ///     .with_bg_color("#ffffff")
    ///     .with_text_color("#000000");
    ///
    /// let vars = theme_context.css_vars();
    /// assert_eq!(
    ///     vars.get("--tg-theme-bg-color"),
    ///     Some(&"#ffffff".to_string())
    /// );
    /// assert_eq!(
    ///     vars.get("--tg-theme-text-color"),
    ///     Some(&"#000000".to_string())
    /// );
    /// ```
    pub fn css_vars(&self) -> std::collections::HashMap<String, String> {
        let mut vars: std::collections::HashMap<String, String> =
            std::collections::HashMap::with_capacity(16);

        let mut push = |key: &str, value: Option<&String>| {
            if let Some(v) = value {
                vars.insert(format!("--tg-theme-{}", key), v.clone());
            }
        };

        push("bg-color", self.bg_color.as_ref());
        push("text-color", self.text_color.as_ref());
        push("hint-color", self.hint_color.as_ref());
        push("link-color", self.link_color.as_ref());
        push("button-color", self.button_color.as_ref());
        push("button-text-color", self.button_text_color.as_ref());
        push("secondary-bg-color", self.secondary_bg_color.as_ref());
        push("header-bg-color", self.header_bg_color.as_ref());
        push("bottom-bar-bg-color", self.bottom_bar_bg_color.as_ref());
        push("accent-text-color", self.accent_text_color.as_ref());
        push("section-bg-color", self.section_bg_color.as_ref());
        push(
            "section-header-text-color",
            self.section_header_text_color.as_ref()
        );
        push(
            "section-separator-color",
            self.section_separator_color.as_ref()
        );
        push("subtitle-text-color", self.subtitle_text_color.as_ref());
        push(
            "destructive-text-color",
            self.destructive_text_color.as_ref()
        );

        vars
    }

    /// Returns the number of set color values.
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new()
    ///     .with_bg_color("#ffffff")
    ///     .with_text_color("#000000");
    ///
    /// assert_eq!(theme_context.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let mut count = 0;
        if self.bg_color.is_some() {
            count += 1;
        }
        if self.text_color.is_some() {
            count += 1;
        }
        if self.hint_color.is_some() {
            count += 1;
        }
        if self.link_color.is_some() {
            count += 1;
        }
        if self.button_color.is_some() {
            count += 1;
        }
        if self.button_text_color.is_some() {
            count += 1;
        }
        if self.secondary_bg_color.is_some() {
            count += 1;
        }
        if self.header_bg_color.is_some() {
            count += 1;
        }
        if self.bottom_bar_bg_color.is_some() {
            count += 1;
        }
        if self.accent_text_color.is_some() {
            count += 1;
        }
        if self.section_bg_color.is_some() {
            count += 1;
        }
        if self.section_header_text_color.is_some() {
            count += 1;
        }
        if self.section_separator_color.is_some() {
            count += 1;
        }
        if self.subtitle_text_color.is_some() {
            count += 1;
        }
        if self.destructive_text_color.is_some() {
            count += 1;
        }
        count
    }

    /// Returns true if no colors are set.
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new();
    /// assert!(theme_context.is_empty());
    ///
    /// let theme_context = theme_context.with_bg_color("#ffffff");
    /// assert!(!theme_context.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Applies all theme colors as CSS custom properties to the document root.
    ///
    /// This method sets CSS variables on the `:root` element, making them
    /// available throughout your application via `var(--tg-theme-*)`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err(String)` if running outside a browser
    /// environment.
    ///
    /// # Example
    ///
    /// ```
    /// use telegram_ui::ThemeContext;
    ///
    /// let theme_context = ThemeContext::new()
    ///     .with_bg_color("#ffffff")
    ///     .with_text_color("#000000");
    ///
    /// theme_context.apply_to_root().ok();
    /// ```
    #[cfg(target_arch = "wasm32")]
    pub fn apply_to_root(&self) -> Result<(), String> {
        use wasm_bindgen::prelude::*;
        use web_sys::HtmlElement;

        let window = web_sys::window().ok_or("No window available")?;
        let document = window.document().ok_or("No document available")?;
        let html_element = document
            .document_element()
            .ok_or("No document element available")?
            .dyn_into::<HtmlElement>()
            .map_err(|_| "Cannot cast to HtmlElement")?;

        let style = html_element.style();

        for (key, value) in self.css_vars() {
            style
                .set_property(&key, &value)
                .map_err(|e| format!("Failed to set {}: {}", key, e))?;
        }

        Ok(())
    }

    /// Applies all theme colors as CSS custom properties to the document root.
    ///
    /// This is a stub implementation for non-WASM targets.
    ///
    /// # Returns
    ///
    /// Always returns `Err` with a message indicating this is only available in
    /// WASM.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn apply_to_root(&self) -> Result<(), String> {
        Err("apply_to_root is only available on wasm32 targets".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_css_class() {
        assert_eq!(Theme::Light.css_class(), "tg-theme-light");
        assert_eq!(Theme::Dark.css_class(), "tg-theme-dark");
        assert_eq!(Theme::Auto.css_class(), "tg-theme-auto");
    }

    #[test]
    fn test_theme_display() {
        assert_eq!(format!("{}", Theme::Light), "light");
        assert_eq!(format!("{}", Theme::Dark), "dark");
        assert_eq!(format!("{}", Theme::Auto), "auto");
    }

    #[test]
    fn test_theme_from_str() {
        assert_eq!("light".parse::<Theme>().unwrap(), Theme::Light);
        assert_eq!("dark".parse::<Theme>().unwrap(), Theme::Dark);
        assert_eq!("auto".parse::<Theme>().unwrap(), Theme::Auto);
        assert_eq!("LIGHT".parse::<Theme>().unwrap(), Theme::Light);
        assert_eq!("Dark".parse::<Theme>().unwrap(), Theme::Dark);
        assert!("invalid".parse::<Theme>().is_err());
    }

    #[test]
    fn test_theme_default() {
        let theme: Theme = Default::default();
        assert_eq!(theme, Theme::Auto);
    }

    #[test]
    fn test_theme_context_builder() {
        let theme_context = ThemeContext::new()
            .with_bg_color("#ffffff")
            .with_text_color("#000000")
            .with_button_color("#3390ec")
            .with_button_text_color("#ffffff");

        assert_eq!(theme_context.bg_color, Some("#ffffff".to_string()));
        assert_eq!(theme_context.text_color, Some("#000000".to_string()));
        assert_eq!(theme_context.button_color, Some("#3390ec".to_string()));
        assert_eq!(theme_context.button_text_color, Some("#ffffff".to_string()));
        assert!(theme_context.hint_color.is_none());
    }

    #[test]
    fn test_theme_context_css_vars() {
        let theme_context = ThemeContext::new()
            .with_bg_color("#ffffff")
            .with_text_color("#000000")
            .with_hint_color("#999999");

        let vars = theme_context.css_vars();

        assert_eq!(
            vars.get("--tg-theme-bg-color"),
            Some(&"#ffffff".to_string())
        );
        assert_eq!(
            vars.get("--tg-theme-text-color"),
            Some(&"#000000".to_string())
        );
        assert_eq!(
            vars.get("--tg-theme-hint-color"),
            Some(&"#999999".to_string())
        );
        assert!(!vars.contains_key("--tg-theme-button-color"));
    }

    #[test]
    fn test_theme_context_len_and_is_empty() {
        let theme_context = ThemeContext::new();
        assert!(theme_context.is_empty());
        assert_eq!(theme_context.len(), 0);

        let theme_context = theme_context.with_bg_color("#ffffff");
        assert!(!theme_context.is_empty());
        assert_eq!(theme_context.len(), 1);

        let theme_context = theme_context
            .with_text_color("#000000")
            .with_button_color("#3390ec");
        assert_eq!(theme_context.len(), 3);
    }

    #[test]
    fn test_theme_context_all_colors() {
        let theme_context = ThemeContext::new()
            .with_bg_color("#ffffff")
            .with_text_color("#000000")
            .with_hint_color("#999999")
            .with_link_color("#3390ec")
            .with_button_color("#3390ec")
            .with_button_text_color("#ffffff")
            .with_secondary_bg_color("#f4f4f5")
            .with_header_bg_color("#ffffff")
            .with_bottom_bar_bg_color("#ffffff")
            .with_accent_text_color("#3390ec")
            .with_section_bg_color("#ffffff")
            .with_section_header_text_color("#707579")
            .with_section_separator_color("#c8c7cc")
            .with_subtitle_text_color("#707579")
            .with_destructive_text_color("#e53935");

        assert_eq!(theme_context.len(), 15);
        assert_eq!(theme_context.css_vars().len(), 15);
    }

    #[test]
    fn test_theme_context_apply_to_root_non_wasm() {
        let theme_context = ThemeContext::new().with_bg_color("#ffffff");
        let result = theme_context.apply_to_root();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("wasm32"));
    }
}
