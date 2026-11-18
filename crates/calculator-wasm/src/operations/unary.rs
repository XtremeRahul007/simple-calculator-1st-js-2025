use calculator_engine::*;

use crate::errors::InvalidOperation;

#[derive(Debug)]
pub enum UnaryOperation {
    SquareRoot,
    Absolute,
    Factorial,
}

impl UnaryOperation {
    pub fn execute(
        self,
        value: &Number,
        precision: &PrecisionContext,
    ) -> Result<Number, CalculationError> {
        match self {
            Self::SquareRoot => square_root(value, precision),
            Self::Factorial => {
                let n = value
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;

                factorial(n, precision)
            }
            Self::Absolute => Ok(absolute(value)),
        }
    }
}

impl TryFrom<&str> for UnaryOperation {
    type Error = InvalidOperation;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "square_root" => Ok(Self::SquareRoot),
            "factorial" => Ok(Self::Factorial),
            "absolute" => Ok(Self::Absolute),
            _ => Err(InvalidOperation(value.to_string())),
        }
    }
}
