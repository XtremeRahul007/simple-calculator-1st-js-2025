use dashu::{Decimal, Rational, float::round::mode::HalfAway};

use crate::{Number, PrecisionContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalculationError {
    DivisionByZero,
    FloatingPointError,
    SquareRootOfNegative,
    InvalidPowerDomain,
}

pub fn add(
    left: &Number,
    right: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => Ok(Number::Exact(a + b)),
        _ => {
            let context = precision.float_context();
            let a = to_approximate(left, precision);
            let b = to_approximate(right, precision);

            let result = context
                .add(a.repr(), b.repr())
                .map_err(|_| CalculationError::FloatingPointError)?;

            Ok(Number::Approximate(result.value()))
        }
    }
}

pub fn subtract(
    left: &Number,
    right: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => Ok(Number::Exact(a - b)),
        _ => {
            let context = precision.float_context();
            let a = to_approximate(left, precision);
            let b = to_approximate(right, precision);

            let result = context
                .sub(a.repr(), b.repr())
                .map_err(|_| CalculationError::FloatingPointError)?;

            Ok(Number::Approximate(result.value()))
        }
    }
}

pub fn multiply(
    left: &Number,
    right: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => Ok(Number::Exact(a * b)),
        _ => {
            let context = precision.float_context();
            let a = to_approximate(left, precision);
            let b = to_approximate(right, precision);

            let result = context
                .mul(a.repr(), b.repr())
                .map_err(|_| CalculationError::FloatingPointError)?;

            Ok(Number::Approximate(result.value()))
        }
    }
}

pub fn divide(
    left: &Number,
    right: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => {
            if b == &Rational::ZERO {
                return Err(CalculationError::DivisionByZero);
            }

            Ok(Number::Exact(a / b))
        }
        _ => {
            let a = to_approximate(left, precision);
            let b = to_approximate(right, precision);

            if b == Decimal::ZERO {
                return Err(CalculationError::DivisionByZero);
            }

            let context = precision.float_context();

            let result = context
                .div(a.repr(), b.repr())
                .map_err(|_| CalculationError::FloatingPointError)?;

            Ok(Number::Approximate(result.value()))
        }
    }
}

pub fn power(
    number: &Number,
    exponent: isize,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    if exponent < 0 {
        match number {
            Number::Exact(value) if value == &Rational::ZERO => {
                return Err(CalculationError::DivisionByZero);
            }
            Number::Approximate(value) if value == &Decimal::ZERO => {
                return Err(CalculationError::DivisionByZero);
            }
            _ => {}
        }
    }

    match number {
        Number::Exact(value) => Ok(Number::Exact(value.pow(exponent))),

        Number::Approximate(value) => {
            let context = precision.float_context();

            let result = context
                .powi(&value.repr(), exponent.into())
                .map_err(|_| CalculationError::FloatingPointError)?;

            Ok(Number::Approximate(result.value()))
        }
    }
}

pub fn square_root(
    base: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match base {
        Number::Exact(value) => {
            if value < &Rational::ZERO {
                return Err(CalculationError::SquareRootOfNegative);
            }

            if let Some(result) = exact_square_root(value) {
                return Ok(Number::Exact(result));
            }
        }

        Number::Approximate(value) => {
            if value < &Decimal::ZERO {
                return Err(CalculationError::SquareRootOfNegative);
            }
        }
    }

    let approximate = to_approximate(base, precision);
    let context = precision.float_context();

    let result = context
        .sqrt(approximate.repr())
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn fractional_power(
    base: &Number,
    exponent: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    match base {
        Number::Exact(value) if value < &Rational::ZERO => {
            return Err(CalculationError::InvalidPowerDomain);
        }
        Number::Approximate(value) if value < &Decimal::ZERO => {
            return Err(CalculationError::InvalidPowerDomain);
        }
        _ => {}
    }

    let exponent_is_negative = match exponent {
        Number::Exact(value) => value < &Rational::ZERO,
        Number::Approximate(value) => value < &Decimal::ZERO,
    };
    let base_is_zero = match base {
        Number::Exact(value) => value == &Rational::ZERO,
        Number::Approximate(value) => value == &Decimal::ZERO,
    };

    if base_is_zero && exponent_is_negative {
        return Err(CalculationError::DivisionByZero);
    }

    let approximate_base = to_approximate(base, precision);
    let approximate_exponent = to_approximate(exponent, precision);

    let context = precision.float_context();

    let result = context
        .powf(&approximate_base.repr(), &approximate_exponent.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

fn exact_square_root(value: &Rational) -> Option<Rational> {
    if value < &Rational::ZERO {
        return None;
    }

    let (numerator, denominator) = value.clone().into_parts();

    let numerator = dashu::Natural::try_from(numerator).ok()?;

    let numerator_root = numerator.nth_root(2);
    let denominator_root = denominator.nth_root(2);

    if numerator_root.sqr() != numerator || denominator_root.sqr() != denominator {
        return None;
    }

    Some(Rational::from_parts(
        numerator_root.as_ibig().clone(),
        denominator_root,
    ))
}

fn to_approximate(number: &Number, precision: &PrecisionContext) -> Decimal {
    match number {
        Number::Exact(value) => value
            .to_float::<HalfAway, 10>(precision.precision())
            .value(),
        Number::Approximate(value) => value.clone(),
    }
}

pub fn absolute(number: &Number) -> Number {
    match number {
        Number::Exact(value) => {
            if value < &Rational::ZERO {
                Number::Exact(-value.clone())
            } else {
                Number::Exact(value.clone())
            }
        }

        Number::Approximate(value) => {
            if value < &Decimal::ZERO {
                Number::Approximate(-value.clone())
            } else {
                Number::Approximate(value.clone())
            }
        }
    }
}
