use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidOperation(pub String);

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported operation: {:?}", self.0)
    }
}

impl Error for InvalidOperation {}
