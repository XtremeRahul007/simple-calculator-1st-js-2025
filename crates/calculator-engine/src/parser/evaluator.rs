use dashu::Natural;

use crate::{number::Number, operations::*, precision::PrecisionContext};

use super::ast::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationError {
    InvalidNumber(String),
    Calculation(CalculationError),
}

pub fn evaluate(
    expression: &Expression,
    precision: &PrecisionContext,
) -> Result<Number, EvaluationError> {
    match expression {
        Expression::Number(value) => {
            Number::parse_exact(value).map_err(|_| EvaluationError::InvalidNumber(value.clone()))
        }

        Expression::Unary { operator, operand } => {
            let value = evaluate(operand, precision)?;

            match operator {
                UnaryOperator::Positive => Ok(value),
                UnaryOperator::Negative => {
                    let negative_one = Number::from_integer(-1);

                    multiply(&negative_one, &value, precision).map_err(EvaluationError::Calculation)
                }
            }
        }

        Expression::Binary {
            operator,
            left,
            right,
        } => {
            let left = evaluate(left, precision)?;
            let right = evaluate(right, precision)?;

            evaluate_binary(operator, &left, &right, precision)
        }
    }
}

fn evaluate_binary(
    operator: &BinaryOperator,
    left: &Number,
    right: &Number,
    precision: &PrecisionContext,
) -> Result<Number, EvaluationError> {
    let result = match operator {
        BinaryOperator::Add => add(left, right, precision),

        BinaryOperator::Subtract => subtract(left, right, precision),

        BinaryOperator::Multiply => multiply(left, right, precision),

        BinaryOperator::Divide => divide(left, right, precision),

        BinaryOperator::Modulo => modulo(left, right),

        BinaryOperator::Power => match right {
            Number::Exact(value) if value.denominator() == &Natural::ONE => {
                let exponent = isize::try_from(value.numerator().clone()).map_err(|_| {
                    EvaluationError::Calculation(CalculationError::InvalidPowerDomain)
                })?;

                power(left, exponent, precision)
            }

            _ => fractional_power(left, right, precision),
        },
    };

    result.map_err(EvaluationError::Calculation)
}
