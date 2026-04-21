// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! `TimelineItem` component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// Timeline item mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineItemMode {
    /// Pre-active state (not yet completed)
    #[default]
    PreActive,
    /// Active state (currently active)
    Active
}

/// Timeline item component
#[derive(Debug, Clone)]
pub struct TimelineItem {
    platform:   String,
    horizontal: bool,
    mode:       TimelineItemMode,
    class:      Option<String>,
    header:     Option<String>,
    children:   Option<String>
}

impl TimelineItem {
    /// Create a new `TimelineItem`
    #[must_use]
    pub fn new() -> Self {
        Self {
            platform:   "base".to_string(),
            horizontal: false,
            mode:       TimelineItemMode::default(),
            class:      None,
            header:     None,
            children:   None
        }
    }

    /// Set platform (ios, android, base)
    #[must_use]
    pub fn platform(mut self, platform: &str) -> Self {
        self.platform = platform.to_string();
        self
    }

    /// Set whether the timeline is horizontal
    #[must_use]
    pub const fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    /// Set the timeline item mode
    #[must_use]
    pub const fn mode(mut self, mode: TimelineItemMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set custom CSS class
    #[must_use]
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Set the item header
    #[must_use]
    pub fn header(mut self, header: &str) -> Self {
        self.header = Some(header.to_string());
        self
    }

    /// Set the item children (description)
    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Render the timeline item as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let mut classes = vec!["telegram-ui-timeline-item"];

        // Mode classes
        match self.mode {
            TimelineItemMode::Active => {
                classes.push("telegram-ui-timeline-item--active");
            }
            TimelineItemMode::PreActive => {
                classes.push("telegram-ui-timeline-item--pre-active");
            }
        }

        // Platform-specific class
        if self.platform == "ios" {
            classes.push("telegram-ui-timeline-item--ios");
        }

        // Horizontal class
        if self.horizontal {
            classes.push("telegram-ui-timeline-item--horizontal");
        }

        // Custom class
        if let Some(custom_class) = &self.class {
            classes.push(custom_class);
        }

        let class_str = classes.join(" ");

        // Build content
        let mut content = String::new();
        content.push_str(r#"<div class="telegram-ui-timeline-item-side">"#);
        content.push_str(r#"<div class="telegram-ui-timeline-item-line"></div>"#);
        content.push_str(r#"<div class="telegram-ui-timeline-item-dot"></div>"#);
        content.push_str("</div>");

        content.push_str(r#"<div class="telegram-ui-timeline-item-fields">"#);
        if let Some(header) = &self.header {
            content.push_str(&format!(
                r#"<span class="telegram-ui-timeline-item-title">{}</span>"#,
                escape_html(header)
            ));
        }
        if let Some(children) = &self.children {
            let _subheadline_level = if self.platform == "ios" { "1" } else { "2" };
            content.push_str(&format!(
                r#"<span class="telegram-ui-timeline-item-description">{}</span>"#,
                escape_html(children)
            ));
        }
        content.push_str("</div>");

        format!(
            r#"<li class="{}">
                {}
            </li>"#,
            escape_html(&class_str),
            content
        )
    }
}

impl Default for TimelineItem {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TimelineItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_item_default() {
        let item = TimelineItem::new();
        assert_eq!(item.platform, "base");
        assert!(!item.horizontal);
        assert_eq!(item.mode, TimelineItemMode::PreActive);
        assert!(item.header.is_none());
        assert!(item.children.is_none());
    }

    #[test]
    fn test_timeline_item_custom() {
        let item = TimelineItem::new()
            .platform("ios")
            .horizontal(true)
            .mode(TimelineItemMode::Active)
            .class("custom")
            .header("2026")
            .children("Description text");

        assert_eq!(item.platform, "ios");
        assert!(item.horizontal);
        assert_eq!(item.mode, TimelineItemMode::Active);
        assert_eq!(item.class, Some("custom".to_string()));
        assert_eq!(item.header, Some("2026".to_string()));
        assert_eq!(item.children, Some("Description text".to_string()));
    }

    #[test]
    fn test_timeline_item_render() {
        let item = TimelineItem::new().header("2026").children("Description");

        let html = item.render();
        assert!(html.contains("telegram-ui-timeline-item"));
        assert!(html.contains("2026"));
        assert!(html.contains("Description"));
    }

    #[test]
    fn test_timeline_item_modes() {
        let active = TimelineItem::new()
            .mode(TimelineItemMode::Active)
            .header("Active");

        let pre_active = TimelineItem::new()
            .mode(TimelineItemMode::PreActive)
            .header("Pre-active");

        assert!(
            active
                .render()
                .contains("telegram-ui-timeline-item--active")
        );
        assert!(
            pre_active
                .render()
                .contains("telegram-ui-timeline-item--pre-active")
        );
    }

    #[test]
    fn test_timeline_item_ios() {
        let item = TimelineItem::new().platform("ios").header("Header");

        let html = item.render();
        assert!(html.contains("telegram-ui-timeline-item--ios"));
    }

    #[test]
    fn test_timeline_item_horizontal() {
        let item = TimelineItem::new().horizontal(true).header("Header");

        let html = item.render();
        assert!(html.contains("telegram-ui-timeline-item--horizontal"));
    }

    #[test]
    fn test_timeline_item_escape_html() {
        let item = TimelineItem::new().header("<script>alert(1)</script>");

        let html = item.render();
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    }
}
