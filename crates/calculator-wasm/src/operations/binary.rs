use calculator_engine::*;

use crate::errors::InvalidOperation;

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl BinaryOperation {
    pub fn execute(
        self,
        left: &Number,
        right: &Number,
        precision: &PrecisionContext,
    ) -> Result<Number, CalculationError> {
        match self {
            Self::Add => add(left, right, precision),
            Self::Subtract => subtract(left, right, precision),
            Self::Divide => divide(left, right, precision),
            Self::Multiply => multiply(left, right, precision),
        }
    }
}

impl TryFrom<&str> for BinaryOperation {
    type Error = InvalidOperation;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "divide" => Ok(Self::Divide),
            "multiply" => Ok(Self::Multiply),
            _ => Err(InvalidOperation(value.to_string())),
        }
    }
}
