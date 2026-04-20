// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! SegmentedControl component for Telegram UI

use std::fmt;

use crate::helpers::escape_html;

/// SegmentedControl component
#[derive(Debug, Clone)]
pub struct SegmentedControl {
    active_value: String,
    options:      Vec<Segment>,
    disabled:     bool,
    stretched:    bool,
    size:         SegmentedControlSize,
    class:        Option<String>,
    id:           Option<String>,
    style:        Option<String>
}

/// Segment option in SegmentedControl
#[derive(Debug, Clone)]
pub struct Segment {
    value: String,
    label: String,
    icon:  Option<String>
}

impl Segment {
    /// Creates a new segment
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            icon:  None
        }
    }

    /// Sets an icon for this segment
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
}

/// SegmentedControl size
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SegmentedControlSize {
    /// Small size
    #[default]
    S,
    /// Medium size
    M,
    /// Large size
    L
}

impl SegmentedControlSize {
    /// Convert to CSS class suffix
    pub fn css_class(&self) -> &'static str {
        match self {
            SegmentedControlSize::S => "tgui-segmented-control--s",
            SegmentedControlSize::M => "tgui-segmented-control--m",
            SegmentedControlSize::L => "tgui-segmented-control--l"
        }
    }
}

impl SegmentedControl {
    /// Creates a new SegmentedControl instance
    pub fn new() -> Self {
        Self {
            active_value: String::new(),
            options:      Vec::new(),
            disabled:     false,
            stretched:    false,
            size:         SegmentedControlSize::M,
            class:        None,
            id:           None,
            style:        None
        }
    }

    /// Sets the currently active value
    pub fn active_value(mut self, value: &str) -> Self {
        self.active_value = value.to_string();
        self
    }

    /// Adds an option to the control
    pub fn option(mut self, option: Segment) -> Self {
        self.options.push(option);
        self
    }

    /// Adds multiple options at once
    pub fn options(mut self, options: Vec<Segment>) -> Self {
        self.options = options;
        self
    }

    /// Sets whether the control is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the control should be stretched to full width
    pub fn stretched(mut self, stretched: bool) -> Self {
        self.stretched = stretched;
        self
    }

    /// Sets the size of the control
    pub fn size(mut self, size: SegmentedControlSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the size from string
    pub fn size_str(mut self, size: &str) -> Self {
        self.size = match size {
            "s" => SegmentedControlSize::S,
            "l" => SegmentedControlSize::L,
            _ => SegmentedControlSize::M
        };
        self
    }

    /// Adds a custom CSS class
    pub fn class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Sets the element ID
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets inline styles
    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    /// Get the active value
    pub fn get_active_value(&self) -> &str {
        &self.active_value
    }

    /// Render the SegmentedControl as HTML string
    pub fn render(&self) -> String {
        let mut classes = vec!["tgui-segmented-control".to_string()];

        classes.push(self.size.css_class().to_string());

        if self.stretched {
            classes.push("tgui-segmented-control--stretched".to_string());
        }

        if self.disabled {
            classes.push("tgui-segmented-control--disabled".to_string());
        }

        if let Some(ref custom_class) = self.class {
            classes.push(custom_class.clone());
        }

        let class_str = classes.join(" ");

        let mut html = String::new();

        html.push_str("<div");
        if let Some(ref id) = self.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }
        html.push_str(&format!(" class=\"{}\"", escape_html(&class_str)));
        if let Some(ref style) = self.style {
            html.push_str(&format!(" style=\"{}\"", escape_html(style)));
        }
        html.push_str(">\n");

        html.push_str("<div class=\"tgui-segmented-control__container\">\n");

        for (i, option) in self.options.iter().enumerate() {
            let mut option_classes = vec!["tgui-segment".to_string()];

            if option.value == self.active_value {
                option_classes.push("tgui-segment--active".to_string());
            }

            if i == 0 {
                option_classes.push("tgui-segment--first".to_string());
            }

            if i == self.options.len() - 1 {
                option_classes.push("tgui-segment--last".to_string());
            }

            let option_class_str = option_classes.join(" ");

            let icon_html = if let Some(ref icon) = option.icon {
                format!(
                    "<div class=\"tgui-segment__icon\">{}</div>",
                    escape_html(icon)
                )
            } else {
                String::new()
            };

            html.push_str("<button");
            html.push_str(&format!(" class=\"{}\"", escape_html(&option_class_str)));
            if self.disabled {
                html.push_str(" disabled");
            }
            html.push_str(&format!(" data-value=\"{}\"", escape_html(&option.value)));
            html.push_str(">\n");

            html.push_str(&icon_html);
            html.push_str(&format!(
                "<span class=\"tgui-segment__label\">{}</span>\n",
                escape_html(&option.label)
            ));

            html.push_str("</button>\n");
        }

        html.push_str("</div>\n");
        html.push_str("</div>\n");

        html
    }
}

impl Default for SegmentedControl {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SegmentedControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_builder() {
        let segment = Segment::new("all", "All");
        assert_eq!(segment.value, "all");
        assert_eq!(segment.label, "All");
        assert!(segment.icon.is_none());
    }

    #[test]
    fn test_segment_with_icon() {
        let segment = Segment::new("users", "Users").icon("👤");
        assert_eq!(segment.icon, Some("👤".to_string()));
    }

    #[test]
    fn test_segmented_control_new() {
        let control = SegmentedControl::new();
        assert!(control.active_value.is_empty());
        assert!(control.options.is_empty());
    }

    #[test]
    fn test_segmented_control_add_options() {
        let control = SegmentedControl::new()
            .option(Segment::new("all", "All"))
            .option(Segment::new("users", "Users"))
            .option(Segment::new("groups", "Groups"));

        assert_eq!(control.options.len(), 3);
        assert_eq!(control.options[0].value, "all");
        assert_eq!(control.options[1].value, "users");
        assert_eq!(control.options[2].value, "groups");
    }

    #[test]
    fn test_segmented_control_active_value() {
        let control = SegmentedControl::new()
            .active_value("users")
            .option(Segment::new("all", "All"))
            .option(Segment::new("users", "Users"));

        assert_eq!(control.get_active_value(), "users");
    }

    #[test]
    fn test_segmented_control_render() {
        let control = SegmentedControl::new()
            .active_value("users")
            .option(Segment::new("all", "All"))
            .option(Segment::new("users", "Users"));

        let html = control.render();
        assert!(html.contains("tgui-segmented-control"));
        assert!(html.contains("tgui-segment--active"));
        assert!(html.contains("Users"));
    }

    #[test]
    fn test_segmented_control_disabled() {
        let control = SegmentedControl::new()
            .disabled(true)
            .option(Segment::new("all", "All"));

        let html = control.render();
        assert!(html.contains("tgui-segmented-control--disabled"));
        assert!(html.contains("disabled"));
    }
}
