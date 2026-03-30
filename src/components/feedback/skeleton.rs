//! Skeleton component

#[derive(Clone, Debug)]
pub struct Skeleton {
    width: String,
    height: String,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            width: "100%".to_string(),
            height: "100px".to_string(),
        }
    }

    pub fn width(&self) -> &str {
        &self.width
    }

    pub fn height(&self) -> &str {
        &self.height
    }
}
