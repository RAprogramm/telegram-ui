//! Textarea component

#[derive(Clone, Debug)]
pub struct Textarea {
    value: String,
    placeholder: String,
}

impl Textarea {
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
