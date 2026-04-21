// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! Timeline component for Telegram UI

use std::fmt;

use crate::components::blocks::timeline_item::TimelineItem;

/// Timeline component
#[derive(Debug, Clone)]
pub struct Timeline {
    horizontal: bool,
    items:      Vec<TimelineItem>
}

impl Timeline {
    /// Create a new Timeline
    #[must_use]
    pub const fn new() -> Self {
        Self {
            horizontal: false,
            items:      Vec::new()
        }
    }

    /// Set whether the timeline is horizontal
    #[must_use]
    pub const fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    /// Add an item to the timeline
    #[must_use]
    pub fn add_item(mut self, item: TimelineItem) -> Self {
        self.items.push(item);
        self
    }

    /// Render the timeline as HTML string
    #[must_use]
    pub fn render(&self) -> String {
        let orientation_class = if self.horizontal {
            "telegram-ui-timeline--horizontal"
        } else {
            ""
        };

        let items_html = self
            .items
            .iter()
            .map(|item| {
                let mut item_rendered = item.clone();
                if self.horizontal {
                    item_rendered = item_rendered.horizontal(true);
                }
                item_rendered.render()
            })
            .collect::<String>();

        format!(
            r#"<ul class="telegram-ui-timeline {orientation_class}">
                {items_html}
            </ul>"#
        )
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_default() {
        let timeline = Timeline::new();
        assert!(!timeline.horizontal);
        assert!(timeline.items.is_empty());
    }

    #[test]
    fn test_timeline_custom() {
        let timeline = Timeline::new()
            .horizontal(true)
            .add_item(TimelineItem::new().header("2026"))
            .add_item(TimelineItem::new().header("2025"));

        assert!(timeline.horizontal);
        assert_eq!(timeline.items.len(), 2);
    }

    #[test]
    fn test_timeline_render() {
        let timeline = Timeline::new()
            .add_item(TimelineItem::new().header("2026"))
            .add_item(TimelineItem::new().header("2025"));

        let html = timeline.render();
        assert!(html.contains("telegram-ui-timeline"));
        assert!(html.contains("2026"));
        assert!(html.contains("2025"));
    }

    #[test]
    fn test_timeline_horizontal() {
        let timeline = Timeline::new()
            .horizontal(true)
            .add_item(TimelineItem::new().header("2026"));

        let html = timeline.render();
        assert!(html.contains("telegram-ui-timeline--horizontal"));
    }

    #[test]
    fn test_timeline_items_horizontal() {
        let timeline = Timeline::new()
            .horizontal(true)
            .add_item(TimelineItem::new().header("2026"));

        let html = timeline.render();
        assert!(html.contains("telegram-ui-timeline-item--horizontal"));
    }
}
