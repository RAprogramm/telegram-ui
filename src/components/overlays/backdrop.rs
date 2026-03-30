//! Backdrop component

#[derive(Clone, Debug)]
pub struct Backdrop {
    visible: bool,
}

impl Backdrop {
    pub fn new() -> Self {
        Self {
            visible: false,
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
