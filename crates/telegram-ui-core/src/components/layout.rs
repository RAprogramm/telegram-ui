//! Layout components for Telegram UI

use std::fmt;

/// Container component
#[derive(Debug, Clone)]
pub struct Container {
    padding: Option<String>,
    margin: Option<String>,
    max_width: Option<String>,
    children: String,
}

impl Container {
    /// Creates a new Container with default settings
    pub fn new() -> Self {
        Self {
            padding: None,
            margin: None,
            max_width: None,
            children: String::new(),
        }
    }

    /// Sets the container padding
    pub fn padding(mut self, padding: &str) -> Self {
        self.padding = Some(padding.to_string());
        self
    }

    /// Sets the container margin
    pub fn margin(mut self, margin: &str) -> Self {
        self.margin = Some(margin.to_string());
        self
    }

    /// Sets the container max width
    pub fn max_width(mut self, max_width: &str) -> Self {
        self.max_width = Some(max_width.to_string());
        self
    }

    /// Sets the container children content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the container padding
    pub fn get_padding(&self) -> Option<&str> {
        self.padding.as_deref()
    }

    /// Returns the container margin
    pub fn get_margin(&self) -> Option<&str> {
        self.margin.as_deref()
    }

    /// Returns the container max width
    pub fn get_max_width(&self) -> Option<&str> {
        self.max_width.as_deref()
    }

    /// Returns the container children content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the container as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(padding) = &self.padding {
            style.push(format!("padding: {};", padding));
        }

        if let Some(margin) = &self.margin {
            style.push(format!("margin: {};", margin));
        }

        if let Some(max_width) = &self.max_width {
            style.push(format!("max-width: {};", max_width));
        }

        let style_str = style.join(" ");

        format!(
            "<div class=\"telegram-ui-container\"{}>{}</div>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Row component (flex row)
#[derive(Debug, Clone)]
pub struct Row {
    align: Option<String>,
    justify: Option<String>,
    gap: Option<String>,
    children: String,
}

impl Row {
    /// Creates a new Row with default settings
    pub fn new() -> Self {
        Self {
            align: None,
            justify: None,
            gap: None,
            children: String::new(),
        }
    }

    /// Sets the align-items CSS property
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the justify-content CSS property
    pub fn justify(mut self, justify: &str) -> Self {
        self.justify = Some(justify.to_string());
        self
    }

    /// Sets the gap between children
    pub fn gap(mut self, gap: &str) -> Self {
        self.gap = Some(gap.to_string());
        self
    }

    /// Adds a child to the row
    pub fn child(mut self, child: &str) -> Self {
        self.children.push_str(child);
        self
    }

    /// Adds multiple children to the row
    pub fn children(mut self, children: &[&str]) -> Self {
        for child in children {
            self.children.push_str(child);
        }
        self
    }

    /// Returns the align-items value
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the justify-content value
    pub fn get_justify(&self) -> Option<&str> {
        self.justify.as_deref()
    }

    /// Returns the gap value
    pub fn get_gap(&self) -> Option<&str> {
        self.gap.as_deref()
    }

    /// Render the row as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(align) = &self.align {
            style.push(format!("align-items: {};", align));
        }

        if let Some(justify) = &self.justify {
            style.push(format!("justify-content: {};", justify));
        }

        if let Some(gap) = &self.gap {
            style.push(format!("gap: {};", gap));
        }

        let style_str = style.join(" ");

        format!(
            "<div class=\"telegram-ui-row\"{}>{}</div>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Column component (flex column)
#[derive(Debug, Clone)]
pub struct Column {
    align: Option<String>,
    justify: Option<String>,
    gap: Option<String>,
    children: String,
}

impl Column {
    /// Creates a new Column with default settings
    pub fn new() -> Self {
        Self {
            align: None,
            justify: None,
            gap: None,
            children: String::new(),
        }
    }

    /// Sets the align-items CSS property
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the justify-content CSS property
    pub fn justify(mut self, justify: &str) -> Self {
        self.justify = Some(justify.to_string());
        self
    }

    /// Sets the gap between children
    pub fn gap(mut self, gap: &str) -> Self {
        self.gap = Some(gap.to_string());
        self
    }

    /// Adds a child to the column
    pub fn child(mut self, child: &str) -> Self {
        self.children.push_str(child);
        self
    }

    /// Adds multiple children to the column
    pub fn children(mut self, children: &[&str]) -> Self {
        for child in children {
            self.children.push_str(child);
        }
        self
    }

    /// Returns the align-items value
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the justify-content value
    pub fn get_justify(&self) -> Option<&str> {
        self.justify.as_deref()
    }

    /// Returns the gap value
    pub fn get_gap(&self) -> Option<&str> {
        self.gap.as_deref()
    }

    /// Render the column as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(align) = &self.align {
            style.push(format!("align-items: {};", align));
        }

        if let Some(justify) = &self.justify {
            style.push(format!("justify-content: {};", justify));
        }

        if let Some(gap) = &self.gap {
            style.push(format!("gap: {};", gap));
        }

        let style_str = style.join(" ");

        format!(
            "<div class=\"telegram-ui-column\"{}>{}</div>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Spacer component
#[derive(Debug, Clone)]
pub struct Spacer {
    size: u32,
}

impl Spacer {
    /// Creates a new Spacer with default size (16px)
    pub fn new() -> Self {
        Self { size: 16 }
    }

    /// Sets the spacer size
    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    /// Returns the spacer size
    pub fn get_size(&self) -> u32 {
        self.size
    }

    /// Render the spacer as HTML string
    pub fn render(&self) -> String {
        format!(
            "<div class=\"telegram-ui-spacer\" style=\"height: {}px;\"></div>",
            self.size
        )
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Spacer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_render() {
        let container = Container::new()
            .padding("16px")
            .max_width("600px")
            .children("<p>Content</p>");

        let html = container.render();
        assert!(html.contains("telegram-ui-container"));
        assert!(html.contains("padding: 16px;"));
        assert!(html.contains("<p>Content</p>"));
    }

    #[test]
    fn test_row_render() {
        let row = Row::new()
            .align("center")
            .justify("space-between")
            .gap("12px")
            .children(&["<div>A</div>", "<div>B</div>"]);

        let html = row.render();
        assert!(html.contains("telegram-ui-row"));
        assert!(html.contains("align-items: center;"));
        assert!(html.contains("gap: 12px;"));
    }

    #[test]
    fn test_column_render() {
        let column = Column::new()
            .gap("8px")
            .children(&["<div>Item 1</div>", "<div>Item 2</div>"]);

        let html = column.render();
        assert!(html.contains("telegram-ui-column"));
        assert!(html.contains("gap: 8px;"));
    }

    #[test]
    fn test_spacer_render() {
        let spacer = Spacer::new().size(32);
        let html = spacer.render();
        assert!(html.contains("telegram-ui-spacer"));
        assert!(html.contains("height: 32px;"));
    }
}
