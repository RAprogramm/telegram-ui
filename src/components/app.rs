// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! AppRoot - Root wrapper component for Telegram Mini Apps

use crate::{Platform, Theme, ThemeContext, helpers::escape_html};

/// AppRoot is the root wrapper component for Telegram Mini Apps.
///
/// It provides:
/// - Platform detection (iOS/Android/Base)
/// - Automatic platform-specific styling
/// - Theme support (Light/Dark/Auto)
/// - Telegram color scheme integration
/// - Base CSS classes and attributes
/// - Optional custom CSS classes
///
/// # Example
///
/// ```ignore
/// use telegram_ui::{AppRoot, Theme, ThemeContext};
///
/// let app_root = AppRoot::new()
///     .platform(Platform::Ios)
///     .theme(Theme::Auto)
///     .theme_context(ThemeContext::new().with_bg_color("#ffffff"))
///     .class("my-app")
///     .children(/* your app content */);
/// ```
#[derive(Debug, Default)]
pub struct AppRoot {
    platform:      Option<Platform>,
    theme:         Option<Theme>,
    theme_context: Option<ThemeContext>,
    class:         Option<String>,
    children:      Option<String>,
    id:            Option<String>,
    style:         Option<String>
}

impl AppRoot {
    /// Create a new AppRoot instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the platform for styling
    ///
    /// If not set, automatic platform detection will be used.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to use (iOS, Android, or Base)
    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Set the theme mode
    ///
    /// If not set, defaults to Auto (follows system preference).
    ///
    /// # Arguments
    ///
    /// * `theme` - The theme mode (Light, Dark, or Auto)
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set the theme context with Telegram color scheme
    ///
    /// This allows you to apply Telegram's color scheme to your app.
    ///
    /// # Arguments
    ///
    /// * `theme_context` - The theme context containing color values
    pub fn theme_context(mut self, theme_context: ThemeContext) -> Self {
        self.theme_context = Some(theme_context);
        self
    }

    /// Add a custom CSS class
    ///
    /// # Arguments
    ///
    /// * `class` - The CSS class to add
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Set the children content
    ///
    /// # Arguments
    ///
    /// * `children` - The HTML content to render inside the AppRoot
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Set the element ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to set on the element
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Set inline styles
    ///
    /// # Arguments
    ///
    /// * `style` - Inline CSS styles
    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    /// Get the platform, using automatic detection if not explicitly set
    fn get_platform(&self) -> Platform {
        self.platform.unwrap_or_else(Self::detect_platform)
    }

    /// Get the theme, using Auto as default if not explicitly set
    fn get_theme(&self) -> Theme {
        self.theme.unwrap_or(Theme::Auto)
    }

    /// Auto-detect the platform from the user agent
    #[cfg(target_arch = "wasm32")]
    fn detect_platform() -> Platform {
        use wasm_bindgen::JsCast;

        // Try to get user agent from JavaScript
        web_sys::window()
            .and_then(|window| {
                let navigator = window.navigator();
                let user_agent = navigator.user_agent().ok()?;
                let ua_lower = user_agent.to_lowercase();

                // Detect iOS
                if ua_lower.contains("iphone")
                    || ua_lower.contains("ipad")
                    || ua_lower.contains("ipod")
                {
                    Some(Platform::Ios)
                }
                // Detect Android
                else if ua_lower.contains("android") {
                    Some(Platform::Android)
                }
                // Default to Base
                else {
                    Some(Platform::Base)
                }
            })
            .unwrap_or(Platform::Base)
    }

    /// Auto-detect the platform (non-WASM fallback)
    #[cfg(not(target_arch = "wasm32"))]
    fn detect_platform() -> Platform {
        // In non-WASM environments (e.g., tests), default to Base
        Platform::Base
    }

    /// Build the class attribute string
    fn build_class(&self) -> String {
        let mut classes = Vec::new();

        // Always add base class
        classes.push("tgui-app-root".to_string());

        // Add platform class
        let platform = self.get_platform();
        match platform {
            Platform::Ios => classes.push("tgui-platform-ios".to_string()),
            Platform::Android => classes.push("tgui-platform-android".to_string()),
            Platform::Base => classes.push("tgui-platform-base".to_string())
        }

        // Add theme class
        let theme = self.get_theme();
        classes.push(theme.css_class().to_string());

        // Add custom class if provided
        if let Some(ref custom_class) = self.class {
            classes.push(custom_class.clone());
        }

        classes.join(" ")
    }

    /// Render the component to HTML string
    pub fn render(&self) -> String {
        let mut html = String::new();

        // Open tag
        html.push_str("<div");

        // ID attribute
        if let Some(ref id) = self.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        // Class attribute
        let class = self.build_class();
        html.push_str(&format!(" class=\"{}\"", escape_html(&class)));

        // Collect all style properties
        let mut style_attrs = String::new();

        // Apply theme context CSS variables as style if provided
        if let Some(ref theme_context) = self.theme_context {
            for (key, value) in theme_context.css_vars() {
                if !style_attrs.is_empty() {
                    style_attrs.push(' ');
                }
                style_attrs.push_str(&format!("{}: {}", key, value));
            }
        }

        // Add custom style if provided
        if let Some(ref custom_style) = self.style {
            if !style_attrs.is_empty() {
                style_attrs.push(' ');
            }
            style_attrs.push_str(custom_style);
        }

        // Add safe-area-inset styles for iOS
        let platform = self.get_platform();
        if platform == Platform::Ios {
            style_attrs.push_str(" padding-top: env(safe-area-inset-top); padding-bottom: env(safe-area-inset-bottom);");
        }

        // Style attribute
        if !style_attrs.is_empty() {
            html.push_str(&format!(" style=\"{}\"", escape_html(&style_attrs)));
        }

        // Data attributes for platform
        html.push_str(&format!(
            " data-platform=\"{}\"",
            match platform {
                Platform::Ios => "ios",
                Platform::Android => "android",
                Platform::Base => "base"
            }
        ));

        // Data attribute for theme
        let theme = self.get_theme();
        html.push_str(&format!(
            " data-theme=\"{}\"",
            match theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
                Theme::Auto => "auto"
            }
        ));

        html.push('>');

        // Add AppRootContext.Provider equivalent as a comment
        // In Rust we use HTML data attributes instead of React Context
        // <AppRootContext.Provider value={contextValue}>

        // Children (raw HTML, not escaped)
        if let Some(ref children) = self.children {
            html.push_str(&escape_html(children));
        }

        // </AppRootContext.Provider> - end of context provider
        // </div>
        html.push_str("</div>");

        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_root_new() {
        let app_root = AppRoot::new();
        assert_eq!(app_root.platform, None);
        assert_eq!(app_root.class, None);
        assert_eq!(app_root.children, None);
    }

    #[test]
    fn test_app_root_platform() {
        let app_root = AppRoot::new().platform(Platform::Ios);
        assert_eq!(app_root.platform, Some(Platform::Ios));
    }

    #[test]
    fn test_app_root_class() {
        let app_root = AppRoot::new().class("my-class");
        assert_eq!(app_root.class, Some("my-class".to_string()));
    }

    #[test]
    fn test_app_root_children() {
        let app_root = AppRoot::new().children("<h1>Hello</h1>");
        assert_eq!(app_root.children, Some("<h1>Hello</h1>".to_string()));
    }

    #[test]
    fn test_app_root_render_basic() {
        let app_root = AppRoot::new();
        let html = app_root.render();
        assert!(html.contains("<div"));
        assert!(html.contains("tgui-app-root"));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn test_app_root_render_with_class() {
        let app_root = AppRoot::new().class("my-app");
        let html = app_root.render();
        assert!(html.contains("tgui-app-root"));
        assert!(html.contains("my-app"));
    }

    #[test]
    fn test_app_root_render_with_platform() {
        let app_root = AppRoot::new().platform(Platform::Ios);
        let html = app_root.render();
        assert!(html.contains("tgui-platform-ios"));
        assert!(html.contains("data-platform=\"ios\""));
    }

    #[test]
    fn test_app_root_render_with_children() {
        let app_root = AppRoot::new().children("<h1>Hello</h1>");
        let html = app_root.render();
        assert!(html.contains("&lt;h1&gt;Hello&lt;/h1&gt;")); // Escaped
    }

    #[test]
    fn test_app_root_render_with_id() {
        let app_root = AppRoot::new().id("app");
        let html = app_root.render();
        assert!(html.contains("id=\"app\""));
    }

    #[test]
    fn test_app_root_render_with_style() {
        let app_root = AppRoot::new().style("color: red;");
        let html = app_root.render();
        assert!(html.contains("style=\"color: red;\""));
    }

    #[test]
    fn test_app_root_chain() {
        let app_root = AppRoot::new()
            .platform(Platform::Android)
            .class("my-app")
            .id("app")
            .style("padding: 10px;")
            .children("<p>Content</p>");

        let html = app_root.render();
        assert!(html.contains("tgui-app-root"));
        assert!(html.contains("tgui-platform-android"));
        assert!(html.contains("my-app"));
        assert!(html.contains("id=\"app\""));
        assert!(html.contains("style=\"padding: 10px;\""));
        assert!(html.contains("&lt;p&gt;Content&lt;/p&gt;"));
    }
}
