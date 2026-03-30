//! Typography components for Telegram UI

use std::fmt;

/// Typography text component
#[derive(Debug, Clone)]
pub struct Text {
    size: Option<String>,
    weight: Option<u32>,
    align: Option<String>,
    color: Option<String>,
    children: String,
}

impl Text {
    /// Creates a new Text with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            weight: None,
            align: None,
            color: None,
            children: String::new(),
        }
    }

    /// Sets the font size (e.g., "17px", "1.2rem")
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the font weight (e.g., 400, 600, 700)
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the text color
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Sets the text content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the font size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns the font weight
    pub fn get_weight(&self) -> Option<u32> {
        self.weight
    }

    /// Returns the text alignment
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the text color
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Returns the text content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the text as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(size) = &self.size {
            style.push(format!("font-size: {};", size));
        }

        if let Some(weight) = self.weight {
            style.push(format!("font-weight: {};", weight));
        }

        if let Some(align) = &self.align {
            style.push(format!("text-align: {};", align));
        }

        if let Some(color) = &self.color {
            style.push(format!("color: {};", color));
        }

        let style_str = style.join(" ");

        format!(
            "<span class=\"telegram-ui-text\"{}>{}</span>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Typography title component
#[derive(Debug, Clone)]
pub struct Title {
    size: Option<String>,
    weight: Option<u32>,
    align: Option<String>,
    children: String,
}

impl Title {
    /// Creates a new Title with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            weight: None,
            align: None,
            children: String::new(),
        }
    }

    /// Sets the font size (e.g., "28px")
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the font weight (e.g., 600, 700)
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the title content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the font size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns the font weight
    pub fn get_weight(&self) -> Option<u32> {
        self.weight
    }

    /// Returns the text alignment
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the title content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the title as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(size) = &self.size {
            style.push(format!("font-size: {};", size));
        }

        if let Some(weight) = self.weight {
            style.push(format!("font-weight: {};", weight));
        }

        if let Some(align) = &self.align {
            style.push(format!("text-align: {};", align));
        }

        let style_str = style.join(" ");

        format!(
            "<h1 class=\"telegram-ui-title\"{}>{}</h1>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Typography subtitle component
#[derive(Debug, Clone)]
pub struct Subtitle {
    size: Option<String>,
    weight: Option<u32>,
    align: Option<String>,
    children: String,
}

impl Subtitle {
    /// Creates a new Subtitle with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            weight: None,
            align: None,
            children: String::new(),
        }
    }

    /// Sets the font size (e.g., "20px")
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the font weight (e.g., 600)
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the subtitle content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the font size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns the font weight
    pub fn get_weight(&self) -> Option<u32> {
        self.weight
    }

    /// Returns the text alignment
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the subtitle content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the subtitle as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(size) = &self.size {
            style.push(format!("font-size: {};", size));
        }

        if let Some(weight) = self.weight {
            style.push(format!("font-weight: {};", weight));
        }

        if let Some(align) = &self.align {
            style.push(format!("text-align: {};", align));
        }

        let style_str = style.join(" ");

        format!(
            "<h2 class=\"telegram-ui-subtitle\"{}>{}</h2>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Subtitle {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Typography headline component
#[derive(Debug, Clone)]
pub struct Headline {
    size: Option<String>,
    weight: Option<u32>,
    align: Option<String>,
    children: String,
}

impl Headline {
    /// Creates a new Headline with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            weight: None,
            align: None,
            children: String::new(),
        }
    }

    /// Sets the font size (e.g., "19px")
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the font weight (e.g., 600)
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the headline content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the font size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns the font weight
    pub fn get_weight(&self) -> Option<u32> {
        self.weight
    }

    /// Returns the text alignment
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the headline content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the headline as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(size) = &self.size {
            style.push(format!("font-size: {};", size));
        }

        if let Some(weight) = self.weight {
            style.push(format!("font-weight: {};", weight));
        }

        if let Some(align) = &self.align {
            style.push(format!("text-align: {};", align));
        }

        let style_str = style.join(" ");

        format!(
            "<h3 class=\"telegram-ui-headline\"{}>{}</h3>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Headline {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Headline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Typography caption component
#[derive(Debug, Clone)]
pub struct Caption {
    size: Option<String>,
    weight: Option<u32>,
    align: Option<String>,
    color: Option<String>,
    children: String,
}

impl Caption {
    /// Creates a new Caption with default settings
    pub fn new() -> Self {
        Self {
            size: None,
            weight: None,
            align: None,
            color: None,
            children: String::new(),
        }
    }

    /// Sets the font size (e.g., "13px")
    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    /// Sets the font weight (e.g., 400)
    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: &str) -> Self {
        self.align = Some(align.to_string());
        self
    }

    /// Sets the text color
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Sets the caption content
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    /// Returns the font size
    pub fn get_size(&self) -> Option<&str> {
        self.size.as_deref()
    }

    /// Returns the font weight
    pub fn get_weight(&self) -> Option<u32> {
        self.weight
    }

    /// Returns the text alignment
    pub fn get_align(&self) -> Option<&str> {
        self.align.as_deref()
    }

    /// Returns the text color
    pub fn get_color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    /// Returns the caption content
    pub fn get_children(&self) -> &str {
        &self.children
    }

    /// Render the caption as HTML string
    pub fn render(&self) -> String {
        let mut style = Vec::new();

        if let Some(size) = &self.size {
            style.push(format!("font-size: {};", size));
        }

        if let Some(weight) = self.weight {
            style.push(format!("font-weight: {};", weight));
        }

        if let Some(align) = &self.align {
            style.push(format!("text-align: {};", align));
        }

        if let Some(color) = &self.color {
            style.push(format!("color: {};", color));
        }

        let style_str = style.join(" ");

        format!(
            "<span class=\"telegram-ui-caption\"{}>{}</span>",
            if style_str.is_empty() { String::new() } else { format!(" style=\"{}\"", style_str) },
            self.children
        )
    }
}

impl Default for Caption {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Caption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_render() {
        let text = Text::new()
            .size("17px")
            .weight(400)
            .children("Hello World");

        let html = text.render();
        assert!(html.contains("telegram-ui-text"));
        assert!(html.contains("Hello World"));
    }

    #[test]
    fn test_title_render() {
        let title = Title::new()
            .size("28px")
            .weight(700)
            .children("My App");

        let html = title.render();
        assert!(html.contains("telegram-ui-title"));
        assert!(html.contains("<h1"));
        assert!(html.contains("My App"));
    }

    #[test]
    fn test_subtitle_render() {
        let subtitle = Subtitle::new()
            .size("20px")
            .weight(600)
            .children("Subtitle");

        let html = subtitle.render();
        assert!(html.contains("telegram-ui-subtitle"));
        assert!(html.contains("<h2"));
        assert!(html.contains("Subtitle"));
    }

    #[test]
    fn test_headline_render() {
        let headline = Headline::new()
            .size("19px")
            .weight(600)
            .children("Headline");

        let html = headline.render();
        assert!(html.contains("telegram-ui-headline"));
        assert!(html.contains("<h3"));
        assert!(html.contains("Headline"));
    }

    #[test]
    fn test_caption_render() {
        let caption = Caption::new()
            .size("13px")
            .color("#707579")
            .children("Caption text");

        let html = caption.render();
        assert!(html.contains("telegram-ui-caption"));
        assert!(html.contains("Caption text"));
    }
}
