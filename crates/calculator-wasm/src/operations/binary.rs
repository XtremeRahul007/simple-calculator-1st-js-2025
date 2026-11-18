use calculator_engine::*;

use crate::errors::InvalidOperation;

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    NthRoot,
    Power,

    // Advanced Math & Comparison
    FractionalPower,
    Percentage,
    GreatestCommonDivisor,
    LeastCommonMultiple,
    Minimum,
    Maximum,

    // Combinatorics (taking n and r)
    Permutations,
    Combinations,
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
            Self::Modulo => modulo(left, right),
            Self::NthRoot => {
                let number = left;
                let degree = right
                    .as_usize()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                nth_root(number, degree, precision)
            }
            Self::Power => {
                let number = left;
                let exponent = right
                    .as_isize()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                power(number, exponent, precision)
            }
            Self::FractionalPower => {
                let base = left;
                let exponent = right;
                fractional_power(base, exponent, precision)
            }
            Self::Percentage => {
                let value = left;
                let percent = right;
                percentage(value, percent, precision)
            }
            Self::GreatestCommonDivisor => greatest_common_divisor(left, right),
            Self::LeastCommonMultiple => least_common_multiple(left, right),
            Self::Minimum => Ok(minimum(left, right)),
            Self::Maximum => Ok(maximum(left, right)),
            Self::Permutations => {
                let n = left
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                let r = right
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                permutations(n, r, precision)
            }
            Self::Combinations => {
                let n = left
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                let r = right
                    .as_u64()
                    .ok_or(CalculationError::InvalidCombinatoricsInput)?;
                combinations(n, r, precision)
            }
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
            "modulo" => Ok(Self::Modulo),
            "nth_root" => Ok(Self::NthRoot),
            "power" => Ok(Self::Power),
            "fractional_power" => Ok(Self::FractionalPower),
            "percentage" => Ok(Self::Percentage),
            "greatest_common_divisor" => Ok(Self::GreatestCommonDivisor),
            "least_common_multiple" => Ok(Self::LeastCommonMultiple),
            "minimum" => Ok(Self::Minimum),
            "maximum" => Ok(Self::Maximum),
            "permutations" => Ok(Self::Permutations),
            "combinations" => Ok(Self::Combinations),
            _ => Err(InvalidOperation(value.to_string())),
        }
    }
}
