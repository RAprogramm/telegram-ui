//! Alert component

#[derive(Clone, Copy, Debug)]
pub enum AlertKind {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Alert {
    kind: AlertKind,
    message: String,
}

impl Alert {
    pub fn new(kind: AlertKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> AlertKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
