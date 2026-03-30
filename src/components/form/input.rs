//! Input component

#[derive(Clone, Debug)]
pub struct Input {
    value: String,
    placeholder: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }
}
