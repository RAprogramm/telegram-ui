// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

//! TabBar component for Telegram UI (iOS-style)

use std::fmt;

use crate::helpers::escape_html;

/// TabBar component
#[derive(Debug, Clone)]
pub struct TabBar {
    active_tab: usize,
    tabs:       Vec<Tab>,
    children:   Option<String>,
    stretched:  bool,
    scrollable: bool,
    class:      Option<String>,
    id:         Option<String>,
    style:      Option<String>
}

/// Tab item in TabBar
#[derive(Debug, Clone)]
pub struct Tab {
    index:    usize,
    icon:     String,
    label:    String,
    badge:    Option<String>,
    disabled: bool
}

impl Tab {
    /// Creates a new tab
    pub fn new(index: usize, label: &str, icon: &str) -> Self {
        Self {
            index,
            label: label.to_string(),
            icon: icon.to_string(),
            badge: None,
            disabled: false
        }
    }

    /// Sets the badge for this tab
    pub fn badge(mut self, badge: &str) -> Self {
        self.badge = Some(badge.to_string());
        self
    }

    /// Sets whether this tab is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl TabBar {
    /// Creates a new TabBar instance
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            tabs:       Vec::new(),
            children:   None,
            stretched:  false,
            scrollable: false,
            class:      None,
            id:         None,
            style:      None
        }
    }

    /// Sets the active tab index
    pub fn active_tab(mut self, index: usize) -> Self {
        self.active_tab = index;
        self
    }

    /// Adds a tab to the TabBar
    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Adds multiple tabs at once
    pub fn tabs(mut self, tabs: Vec<Tab>) -> Self {
        self.tabs = tabs;
        self
    }

    /// Sets the children content
    pub fn children(mut self, children: &str) -> Self {
        self.children = Some(children.to_string());
        self
    }

    /// Sets whether the TabBar should be stretched to full width
    pub fn stretched(mut self, stretched: bool) -> Self {
        self.stretched = stretched;
        self
    }

    /// Sets whether the TabBar should be scrollable
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
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

    /// Render the TabBar as HTML string
    pub fn render(&self) -> String {
        let mut classes = vec!["tgui-tab-bar".to_string()];

        if self.stretched {
            classes.push("tgui-tab-bar--stretched".to_string());
        }

        if self.scrollable {
            classes.push("tgui-tab-bar--scrollable".to_string());
        }

        if let Some(ref custom_class) = self.class {
            classes.push(custom_class.clone());
        }

        let class_str = classes.join(" ");

        let mut html = String::new();

        // Open tag
        html.push_str("<div");

        if let Some(ref id) = self.id {
            html.push_str(&format!(" id=\"{}\"", escape_html(id)));
        }

        html.push_str(&format!(" class=\"{}\"", escape_html(&class_str)));

        if let Some(ref style) = self.style {
            html.push_str(&format!(" style=\"{}\"", escape_html(style)));
        }

        html.push_str(&format!(" data-active-tab=\"{}\"", self.active_tab));
        html.push_str(">\n");

        // Render tabs
        html.push_str("<div class=\"tgui-tab-bar__content\">\n");
        html.push_str("<div class=\"tgui-tab-bar__tabs\">\n");

        for tab in &self.tabs {
            let mut tab_classes = vec!["tgui-tab".to_string()];

            if tab.index == self.active_tab {
                tab_classes.push("tgui-tab--active".to_string());
            }

            if tab.disabled {
                tab_classes.push("tgui-tab--disabled".to_string());
            }

            let tab_class_str = tab_classes.join(" ");

            let badge_html = if let Some(ref badge) = tab.badge {
                format!("<div class=\"tgui-tab-badge\">{}</div>", escape_html(badge))
            } else {
                String::new()
            };

            html.push_str(&format!(
                "<button class=\"{}\"{} data-tab-index=\"{}\">\n",
                escape_html(&tab_class_str),
                if tab.disabled { " disabled" } else { "" },
                tab.index
            ));
            html.push_str(&format!(
                "<div class=\"tgui-tab__icon\">{}</div>\n",
                escape_html(&tab.icon)
            ));
            html.push_str(&format!(
                "<div class=\"tgui-tab__label\">{}</div>\n",
                escape_html(&tab.label)
            ));
            html.push_str(&badge_html);
            html.push_str("</button>\n");
        }

        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // Render active tab content
        if let Some(ref children) = self.children {
            html.push_str("<div class=\"tgui-tab-bar__content-area\">\n");
            html.push_str(children);
            html.push_str("</div>\n");
        }

        html.push_str("</div>");

        html
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TabBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_builder() {
        let tab = Tab::new(0, "Home", "🏠");
        assert_eq!(tab.index, 0);
        assert_eq!(tab.label, "Home");
        assert_eq!(tab.icon, "🏠");
        assert!(tab.badge.is_none());
    }

    #[test]
    fn test_tab_with_badge() {
        let tab = Tab::new(0, "Messages", "💬").badge("5");
        assert_eq!(tab.badge, Some("5".to_string()));
    }

    #[test]
    fn test_tabbar_new() {
        let tab_bar = TabBar::new();
        assert_eq!(tab_bar.active_tab, 0);
        assert!(tab_bar.tabs.is_empty());
    }

    #[test]
    fn test_tabbar_add_tabs() {
        let tab_bar = TabBar::new()
            .tab(Tab::new(0, "Home", "🏠"))
            .tab(Tab::new(1, "Search", "🔍"))
            .tab(Tab::new(2, "Profile", "👤"));

        assert_eq!(tab_bar.tabs.len(), 3);
        assert_eq!(tab_bar.tabs[0].label, "Home");
        assert_eq!(tab_bar.tabs[1].label, "Search");
        assert_eq!(tab_bar.tabs[2].label, "Profile");
    }

    #[test]
    fn test_tabbar_active_tab() {
        let tab_bar = TabBar::new()
            .active_tab(1)
            .tab(Tab::new(0, "Home", "🏠"))
            .tab(Tab::new(1, "Search", "🔍"));

        assert_eq!(tab_bar.active_tab, 1);
    }

    #[test]
    fn test_tabbar_render() {
        let tab_bar = TabBar::new()
            .active_tab(0)
            .tab(Tab::new(0, "Home", "🏠"))
            .tab(Tab::new(1, "Search", "🔍"));

        let html = tab_bar.render();
        assert!(html.contains("tgui-tab-bar"));
        assert!(html.contains("tgui-tab--active"));
        assert!(html.contains("Home"));
        assert!(html.contains("Search"));
        assert!(html.contains("data-active-tab=\"0\""));
    }

    #[test]
    fn test_tabbar_with_badge() {
        let tab_bar = TabBar::new().tab(Tab::new(0, "Messages", "💬").badge("3"));

        let html = tab_bar.render();
        assert!(html.contains("tgui-tab-badge"));
        assert!(html.contains("3"));
    }

    #[test]
    fn test_tabbar_disabled_tab() {
        let tab_bar = TabBar::new().tab(Tab::new(0, "Home", "🏠").disabled(true));

        let html = tab_bar.render();
        assert!(html.contains("tgui-tab--disabled"));
        assert!(html.contains("disabled"));
    }
}
