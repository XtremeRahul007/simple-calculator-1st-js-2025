use dashu::Decimal;

use crate::{CalculationError, Number, PrecisionContext};

pub fn pi(precision: &PrecisionContext) -> Number {
    let context = precision.float_context();

    Number::Approximate(context.pi::<10>(None).value())
}

pub fn euler(precision: &PrecisionContext) -> Number {
    let context = precision.float_context();

    Number::Approximate(context.e::<10>().value())
}

pub fn tau(precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let context = precision.float_context();

    let pi = context.pi::<10>(None).value();
    let two = Decimal::from(2);

    let result = context
        .mul(&pi.repr(), &two.repr())
        .map_err(|_| CalculationError::FloatingPointError)?;

    Ok(Number::Approximate(result.value()))
}

pub fn golden_ratio(precision: &PrecisionContext) -> Result<Number, CalculationError> {
    let context = precision.float_context();

    let five = Decimal::from(5);
    let one = Decimal::ONE;
    let two = Decimal::from(2);

    let sqrt_five = context
        .sqrt(&five.repr())
        .map_err(|_| CalculationError::FloatingPointError)?
        .value();

    let sum = context
        .add(&sqrt_five.repr(), &one.repr())
        .map_err(|_| CalculationError::FloatingPointError)?
        .value();

    let result = context
        .div(&sum.repr(), &two.repr())
        .map_err(|_| CalculationError::FloatingPointError)?
        .value();

    Ok(Number::Approximate(result))
}
