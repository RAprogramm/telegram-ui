//! Headline component

#[derive(Clone, Debug)]
pub struct Headline {
    text: String,
    level: u32,
}

impl Headline {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            level: 1,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn level(&self) -> u32 {
        self.level
    }
}
