// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Banner component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Banner type/variant
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BannerType {
    #[default]
    /// Regular banner
    Regular,
    /// Picture banner
    Picture,
    /// Promo banner
    Promo,
}

/// Banner component
#[derive(Debug, Clone)]
pub struct Banner {
    banner_type: BannerType,
    title: String,
    subtitle: Option<String>,
    description: Option<String>,
    link: Option<String>,
    image: Option<String>,
}

impl Banner {
    /// Create a new Banner
    pub fn new() -> Self {
        Self {
            banner_type: BannerType::Regular,
            title: String::new(),
            subtitle: None,
            description: None,
            link: None,
            image: None,
        }
    }

    /// Set banner type
    pub fn banner_type(mut self, banner_type: BannerType) -> Self {
        self.banner_type = banner_type;
        self
    }

    /// Set the title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the subtitle
    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    /// Set the description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set the link URL
    pub fn link(mut self, link: &str) -> Self {
        self.link = Some(link.to_string());
        self
    }

    /// Set the image URL
    pub fn image(mut self, image: &str) -> Self {
        self.image = Some(image.to_string());
        self
    }

    /// Render the banner as HTML string
    pub fn render(&self) -> String {
        let type_class = match self.banner_type {
            BannerType::Regular => "banner--regular",
            BannerType::Picture => "banner--picture",
            BannerType::Promo => "banner--promo",
        };

        let mut html = format!(
            r#"<div class="telegram-ui-banner {type_class}">"#,
            type_class = type_class
        );

        if let Some(ref image) = self.image {
            html.push_str(&format!(
                r#"<div class="banner-image"><img src="{image}" alt=""></div>"#,
                image = escape_html(image)
            ));
        }

        html.push_str(&format!(
            r#"<div class="banner-content">
                <div class="banner-title">{title}</div>"#,
            title = escape_html(&self.title)
        ));

        if let Some(ref subtitle) = self.subtitle {
            html.push_str(&format!(
                r#"<div class="banner-subtitle">{subtitle}</div>"#,
                subtitle = escape_html(subtitle)
            ));
        }

        if let Some(ref description) = self.description {
            html.push_str(&format!(
                r#"<div class="banner-description">{description}</div>"#,
                description = escape_html(description)
            ));
        }

        if let Some(ref link) = self.link {
            html.push_str(&format!(
                r#"<a href="{link}" class="banner-link">Learn more</a>"#,
                link = escape_html(link)
            ));
        }

        html.push_str("</div></div>");
        html
    }
}

impl Default for Banner {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Banner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_default() {
        let banner = Banner::new();
        assert_eq!(banner.banner_type, BannerType::Regular);
        assert!(banner.title.is_empty());
    }

    #[test]
    fn test_banner_render() {
        let banner = Banner::new()
            .title("Welcome")
            .subtitle("Hello world")
            .description("This is a description");

        let html = banner.render();
        assert!(html.contains("Welcome"));
        assert!(html.contains("Hello world"));
    }
}
