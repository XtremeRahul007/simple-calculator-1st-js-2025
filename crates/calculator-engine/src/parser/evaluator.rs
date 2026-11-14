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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse, tokenize};

    fn calculate(input: &str) -> Result<Number, EvaluationError> {
        let tokens = tokenize(input).unwrap();
        let expression = parse(tokens).unwrap();

        evaluate(&expression, &PrecisionContext::default())
    }

    #[test]
    fn evaluates_addition() {
        assert_eq!(calculate("10 + 20").unwrap(), Number::from_integer(30));
    }

    #[test]
    fn evaluates_operator_precedence() {
        assert_eq!(calculate("2 + 3 * 4").unwrap(), Number::from_integer(14));
    }

    #[test]
    fn evaluates_parentheses() {
        assert_eq!(calculate("(2 + 3) * 4").unwrap(), Number::from_integer(20));
    }

    #[test]
    fn evaluates_unary_minus() {
        assert_eq!(calculate("-5 + 2").unwrap(), Number::from_integer(-3));
    }

    #[test]
    fn evaluates_power() {
        assert_eq!(calculate("2 ^ 3").unwrap(), Number::from_integer(8));
    }

    #[test]
    fn evaluates_right_associative_power() {
        assert_eq!(calculate("2 ^ 3 ^ 2").unwrap(), Number::from_integer(512));
    }

    #[test]
    fn evaluates_negative_exponent() {
        assert_eq!(
            calculate("2 ^ -3").unwrap(),
            Number::parse_exact("1/8").unwrap()
        );
    }

    #[test]
    fn unary_minus_has_lower_precedence_than_power() {
        assert_eq!(calculate("-2 ^ 2").unwrap(), Number::from_integer(-4));
    }

    #[test]
    fn evaluates_exact_division() {
        assert_eq!(
            calculate("1 / 3").unwrap(),
            Number::parse_exact("1/3").unwrap()
        );
    }

    #[test]
    fn evaluates_modulo() {
        assert_eq!(calculate("17 % 5").unwrap(), Number::from_integer(2));
    }

    #[test]
    fn propagates_division_by_zero() {
        assert_eq!(
            calculate("10 / 0"),
            Err(EvaluationError::Calculation(
                CalculationError::DivisionByZero
            ))
        );
    }

    #[test]
    fn evaluates_scientific_notation() {
        assert_eq!(calculate("1e3 + 2e3").unwrap(), Number::from_integer(3000));
    }
}
