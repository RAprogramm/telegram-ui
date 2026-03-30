//! Row component

#[derive(Clone, Debug)]
pub struct Row {
    children: Vec<String>,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}
