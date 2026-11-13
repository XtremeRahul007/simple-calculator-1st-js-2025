use dashu::{
    Decimal, Integer, Natural, Rational,
    base::{Abs, DivEuclid, Gcd, RemEuclid},
    dbig,
    float::round::mode::HalfAway,
};

use crate::{Number, PrecisionContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalculationError {
    DivisionByZero,
    FloatingPointError,
    SquareRootOfNegative,
    InvalidPowerDomain,
    InvalidLogDomain,
    InvalidTrigonometricDomain,
    InvalidHyperbolicDomain,
    InvalidCombinatoricsInput,
    IntegerInputRequired,
    PrimeFactorizationOfZero,
    InvalidRootDegree,
    EvenRootOfNegative,
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

pub fn natural_log(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let is_non_positive = match number {
        Number::Exact(value) => value <= &Rational::ZERO,
        Number::Approximate(value) => value <= &Decimal::ZERO,
    };

    if is_non_positive {
        return Err(CalculationError::InvalidLogDomain);
    }

    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .ln(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn common_log(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let is_non_positive = match number {
        Number::Exact(value) => value <= &Rational::ZERO,
        Number::Approximate(value) => value <= &Decimal::ZERO,
    };

    if is_non_positive {
        return Err(CalculationError::InvalidLogDomain);
    }

    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .log10(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn exponential(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .exp(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn power_of_ten(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();
    let ten = dbig!(10);

    let result = context
        .powf(ten.repr(), &value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn sine(number: &Number, precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .sin(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn cosine(number: &Number, precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .cos(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn tangent(number: &Number, precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .tan(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn arcsine(number: &Number, precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);

    if value < Decimal::NEG_ONE || value > Decimal::ONE {
        return Err(CalculationError::InvalidTrigonometricDomain);
    }

    let context = precision.float_context();

    let result = context
        .asin(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn arccosine(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);

    if value < Decimal::NEG_ONE || value > Decimal::ONE {
        return Err(CalculationError::InvalidTrigonometricDomain);
    }

    let context = precision.float_context();

    let result = context
        .acos(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn arctangent(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .atan(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn hyperbolic_sine(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .sinh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn hyperbolic_cosine(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .cosh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn hyperbolic_tangent(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .tanh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn inverse_hyperbolic_sine(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);
    let context = precision.float_context();

    let result = context
        .asinh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn inverse_hyperbolic_cosine(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);

    if value < Decimal::ONE {
        return Err(CalculationError::InvalidHyperbolicDomain);
    }

    let context = precision.float_context();

    let result = context
        .acosh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn inverse_hyperbolic_tangent(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let value = to_approximate(number, precision);

    if value <= Decimal::NEG_ONE || value >= Decimal::ONE {
        return Err(CalculationError::InvalidHyperbolicDomain);
    }

    let context = precision.float_context();

    let result = context
        .atanh(&value.repr(), None)
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn factorial(n: u32, precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let mut result = Number::from_integer(1);

    for factor in 2..=n {
        let factor = Number::from_integer(factor);
        result = multiply(&result, &factor, precision)?;
    }

    Ok(result)
}

pub fn permutations(
    n: u32,
    r: u32,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    if r > n {
        return Err(CalculationError::InvalidCombinatoricsInput);
    }

    let mut result = Number::from_integer(1);

    for offset in 0..r {
        let factor = Number::from_integer(n - offset);
        result = multiply(&result, &factor, precision)?;
    }

    Ok(result)
}

pub fn combinations(
    n: u32,
    r: u32,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    if r > n {
        return Err(CalculationError::InvalidCombinatoricsInput);
    }

    let r = r.min(n - r);

    let mut result = Number::from_integer(1);

    for i in 0..r {
        let numerator = Number::from_integer(n - i);
        let denominator = Number::from_integer(i + 1);

        result = multiply(&result, &numerator, precision)?;
        result = divide(&result, &denominator, precision)?;
    }

    Ok(result)
}

pub fn percentage(
    value: &Number,
    percent: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    let product = multiply(value, percent, precision)?;

    divide(&product, &Number::from_integer(100), precision)
}

pub fn greatest_common_divisor(left: &Number, right: &Number) -> Result<Number, CalculationError> {
    let left = extract_integer(left)?;
    let right = extract_integer(right)?;

    if left == Integer::ZERO && right == Integer::ZERO {
        return Ok(Number::from_integer(0));
    }

    let result = left.gcd(right);

    Ok(Number::from_rational(Rational::from(result)))
}

pub fn least_common_multiple(left: &Number, right: &Number) -> Result<Number, CalculationError> {
    let left = extract_integer(left)?;
    let right = extract_integer(right)?;

    if left == Integer::ZERO || right == Integer::ZERO {
        return Ok(Number::from_integer(0));
    }

    let product = Rational::from(left.clone()).abs() * Rational::from(right.clone()).abs();

    let divisor = Rational::from(left.gcd(right));

    Ok(Number::from_rational(product / divisor))
}

pub fn modulo(left: &Number, right: &Number) -> Result<Number, CalculationError> {
    let left = extract_integer(left)?;
    let right = extract_integer(right)?;

    if right == Integer::ZERO {
        return Err(CalculationError::DivisionByZero);
    }

    let left = Rational::from(left);
    let right = Rational::from(right);

    Ok(Number::from_rational(left.rem_euclid(&right)))
}

pub fn floor(number: &Number) -> Number {
    match number {
        Number::Exact(value) => Number::from_integer(value.clone().div_euclid(&Rational::ONE)),
        Number::Approximate(value) => Number::from_integer(value.floor().to_int().value()),
    }
}

pub fn ceiling(number: &Number) -> Number {
    match number {
        Number::Exact(value) => {
            let negative = -value.clone();
            Number::from_integer(-negative.div_euclid(&Rational::ONE))
        }
        Number::Approximate(value) => Number::from_integer(value.ceil().to_int().value()),
    }
}

pub fn round(number: &Number) -> Number {
    match number {
        Number::Exact(value) => {
            let quotient = value.clone().div_euclid(&Rational::ONE);
            let remainder = value.clone().rem_euclid(&Rational::ONE);

            let half = Rational::from_parts(Integer::ONE, Natural::from(2u32));

            let should_increment = if value < &Rational::ZERO {
                remainder > half
            } else {
                remainder >= half
            };

            if should_increment {
                Number::from_integer(quotient + Integer::ONE)
            } else {
                Number::from_integer(quotient)
            }
        }
        Number::Approximate(value) => Number::from_integer(value.round().to_int().value()),
    }
}

pub fn truncate(number: &Number) -> Number {
    match number {
        Number::Exact(value) => {
            let negative = value < &Rational::ZERO;

            let result = if negative {
                -(-value.clone()).div_euclid(&Rational::ONE)
            } else {
                value.clone().div_euclid(&Rational::ONE)
            };

            Number::from_integer(result)
        }
        Number::Approximate(value) => Number::from_integer(value.trunc().to_int().value()),
    }
}

pub fn is_prime(number: &Number) -> Result<bool, CalculationError> {
    let value = extract_integer(number)?;

    if value < Integer::from(2) {
        return Ok(false);
    }

    if value == Integer::from(2) {
        return Ok(true);
    }

    if &value % Integer::from(2) == Integer::ZERO {
        return Ok(false);
    }

    let mut divisor = Integer::from(3);

    while divisor <= &value / &divisor {
        if &value % &divisor == Integer::ZERO {
            return Ok(false);
        }

        divisor += 2;
    }

    Ok(true)
}

pub fn prime_factors(number: &Number) -> Result<Vec<Number>, CalculationError> {
    let value = extract_integer(number)?;

    if value == Integer::ZERO {
        return Err(CalculationError::PrimeFactorizationOfZero);
    }

    let negative = value < Integer::ZERO;

    let mut remaining = if negative { -value } else { value };

    let mut factors = Vec::new();

    if negative {
        factors.push(Number::from_integer(-1));
    }

    let mut divisor = Integer::from(2);

    while divisor <= &remaining / &divisor {
        while &remaining % &divisor == Integer::ZERO {
            factors.push(Number::from_integer(divisor.clone()));
            remaining /= &divisor;
        }

        if divisor == Integer::from(2) {
            divisor = Integer::from(3);
        } else {
            divisor += 2;
        }
    }

    if remaining > Integer::ONE {
        factors.push(Number::from_integer(remaining));
    }

    Ok(factors)
}

pub fn cube_root(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    nth_root(number, 3, precision)
}

pub fn nth_root(
    number: &Number,
    degree: usize,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    if degree == 0 {
        return Err(CalculationError::InvalidRootDegree);
    }

    let value = to_approximate(number, precision);

    if value < Decimal::ZERO && degree % 2 == 0 {
        return Err(CalculationError::EvenRootOfNegative);
    }

    let result = precision
        .float_context()
        .nth_root(degree, &value.repr())
        .map_err(|_| CalculationError::FloatingPointError)?
        .value();

    Ok(Number::from_approximate(result))
}

pub fn reciprocal(
    number: &Number,
    precision: &PrecisionContext,
) -> Result<Number, CalculationError> {
    divide(&Number::from_integer(1), number, precision)
}

pub fn sign(number: &Number) -> Number {
    let result = match number {
        Number::Exact(value) => {
            if value > &Rational::ZERO {
                1
            } else if value < &Rational::ZERO {
                -1
            } else {
                0
            }
        }
        Number::Approximate(value) => {
            if value > &Decimal::ZERO {
                1
            } else if value < &Decimal::ZERO {
                -1
            } else {
                0
            }
        }
    };

    Number::from_integer(result)
}

pub fn minimum(left: &Number, right: &Number) -> Number {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => {
            Number::Exact(if a <= b { a.clone() } else { b.clone() })
        }
        _ => {
            let a = to_approximate(left, &PrecisionContext::default());
            let b = to_approximate(right, &PrecisionContext::default());

            Number::Approximate(if a <= b { a } else { b })
        }
    }
}

pub fn maximum(left: &Number, right: &Number) -> Number {
    match (left, right) {
        (Number::Exact(a), Number::Exact(b)) => {
            Number::Exact(if a >= b { a.clone() } else { b.clone() })
        }
        _ => {
            let a = to_approximate(left, &PrecisionContext::default());
            let b = to_approximate(right, &PrecisionContext::default());

            Number::Approximate(if a >= b { a } else { b })
        }
    }
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

fn extract_integer(number: &Number) -> Result<Integer, CalculationError> {
    let Number::Exact(value) = number else {
        return Err(CalculationError::IntegerInputRequired);
    };

    let (numerator, denominator) = value.clone().into_parts();

    if denominator != Natural::ONE {
        return Err(CalculationError::IntegerInputRequired);
    }

    Ok(numerator)
}
