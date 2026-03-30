//! Toast component

#[derive(Clone, Debug)]
pub struct Toast {
    message: String,
    duration: u32,
}

impl Toast {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            duration: 3000,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }
}
