#[derive(Debug, Clone)]
pub struct Blockquote {
    r#type:   BlockquoteType,
    children: String
}

#[derive(Debug, Clone, Default)]
pub enum BlockquoteType {
    #[default]
    Text,
    Other
}

impl Blockquote {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            r#type:   BlockquoteType::Text,
            children: String::new()
        }
    }

    #[must_use]
    pub fn with_type(mut self, r#type: &str) -> Self {
        self.r#type = match r#type {
            "other" => BlockquoteType::Other,
            _ => BlockquoteType::Text
        };
        self
    }

    #[must_use]
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let type_class = match self.r#type {
            BlockquoteType::Text => "blockquote--text",
            BlockquoteType::Other => "blockquote--other"
        };

        let content = match self.r#type {
            BlockquoteType::Text => format!(
                "<span class=\"telegram-ui-blockquote-text\">{}</span>",
                self.children
            ),
            BlockquoteType::Other => self.children.clone()
        };

        format!(
            r#"<div class="telegram-ui-blockquote {type_class}">
  {content}
</div>"#
        )
    }
}

impl Default for Blockquote {
    fn default() -> Self {
        Self::new()
    }
}
