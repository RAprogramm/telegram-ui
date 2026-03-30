//! Badge component

#[derive(Clone, Debug)]
pub struct Badge {
    value: String,
    visible: bool,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            visible: false,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
