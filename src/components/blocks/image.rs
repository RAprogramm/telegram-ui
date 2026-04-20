// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Image component for Telegram UI

use std::fmt;

/// Image fit mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ImageFit {
    #[default]
    /// Cover the entire area
    Cover,
    /// Contain within the area
    Contain,
    /// Stretch to fill
    Fill,
    /// Use none
    None
}

/// Image component with lazy loading support
#[derive(Debug, Clone)]
pub struct Image {
    src:     String,
    alt:     String,
    width:   Option<String>,
    height:  Option<String>,
    fit:     ImageFit,
    lazy:    bool,
    rounded: bool
}

impl Image {
    /// Create a new Image
    pub fn new(src: &str) -> Self {
        Self {
            src:     src.to_string(),
            alt:     String::new(),
            width:   None,
            height:  None,
            fit:     ImageFit::Cover,
            lazy:    true,
            rounded: false
        }
    }

    /// Set alt text
    pub fn alt(mut self, alt: &str) -> Self {
        self.alt = alt.to_string();
        self
    }

    /// Set width
    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    /// Set height
    pub fn height(mut self, height: &str) -> Self {
        self.height = Some(height.to_string());
        self
    }

    /// Set fit mode
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// Set lazy loading
    pub fn lazy(mut self, lazy: bool) -> Self {
        self.lazy = lazy;
        self
    }

    /// Set rounded corners
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    /// Render the image as HTML string
    pub fn render(&self) -> String {
        let fit_class = match self.fit {
            ImageFit::Cover => "image--cover",
            ImageFit::Contain => "image--contain",
            ImageFit::Fill => "image--fill",
            ImageFit::None => "image--none"
        };

        let rounded_class = if self.rounded { " image--rounded" } else { "" };
        let lazy_attr = if self.lazy { " loading=\"lazy\"" } else { "" };

        let mut style = String::new();
        if let Some(ref w) = self.width {
            style.push_str(&format!("width: {};", w));
        }
        if let Some(ref h) = self.height {
            style.push_str(&format!("height: {};", h));
        }

        format!(
            r#"<img src="{src}" alt="{alt}"{lazy_attr} class="telegram-ui-image {fit_class}{rounded_class}"{style_attr}>"#,
            src = &self.src,
            alt = &self.alt,
            lazy_attr = lazy_attr,
            fit_class = fit_class,
            rounded_class = rounded_class,
            style_attr = if !style.is_empty() {
                format!(" style=\"{}\"", style)
            } else {
                String::new()
            }
        )
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_default() {
        let img = Image::new("test.jpg");
        assert_eq!(img.src, "test.jpg");
        assert!(img.lazy);
    }

    #[test]
    fn test_image_render() {
        let img = Image::new("image.png")
            .alt("Test image")
            .width("100px")
            .rounded(true);

        let html = img.render();
        assert!(html.contains("image.png"));
        assert!(html.contains("Test image"));
        assert!(html.contains("rounded"));
    }
}
