pub mod binary;
pub mod operations;
pub mod unary;

pub use binary::*;
pub use operations::*;
pub use unary::*;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidOperation(pub String);

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unsupported operation: {:?}", self.0)
    }
}

impl Error for InvalidOperation {}
