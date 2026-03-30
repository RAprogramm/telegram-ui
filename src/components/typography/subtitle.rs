//! Subtitle component

#[derive(Clone, Debug)]
pub struct Subtitle {
    text: String,
}

impl Subtitle {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
