mod errors;
mod operations;
mod responses;
mod tests;

use calculator_engine::*;
use operations::*;

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::responses::*;

fn calculate_with(
    left: &str,
    right: &str,
    operation: impl FnOnce(&Number, &Number, &PrecisionContext) -> Result<Number, CalculationError>,
) -> CalculationResponse {
    let left = match Number::parse_exact(left) {
        Ok(number) => number,
        Err(_) => return invalid_number_response(),
    };

    let right = match Number::parse_exact(right) {
        Ok(number) => number,
        Err(_) => return invalid_number_response(),
    };

    let result = operation(&left, &right, &PrecisionContext::default());

    into_calculation_response(result)
}

#[wasm_bindgen]
pub fn calculate(
    left: &str,
    right: &str,
    operation: &str,
) -> Result<Ts<CalculationResponse>, JsError> {
    let operation = match BinaryOperation::try_from(operation) {
        Ok(operation) => operation,
        Err(error) => {
            eprintln!("{error}");
            return Ok(invalid_operation_response().into_ts()?);
        }
    };

    calculate_with(left, right, |left, right, precision| {
        operation.execute(left, right, precision)
    })
    .into_ts()
    .map_err(Into::into)
}

#[wasm_bindgen]
pub fn calculate_unary(value: &str, operation: &str) -> Result<Ts<CalculationResponse>, JsError> {
    let operation = match UnaryOperation::try_from(operation) {
        Ok(operation) => operation,
        Err(_) => return invalid_operation_response().into_ts().map_err(Into::into),
    };

    let value = match Number::parse_exact(value) {
        Ok(value) => value,
        Err(_) => return invalid_number_response().into_ts().map_err(Into::into),
    };

    let result = operation.execute(&value, &PrecisionContext::default());

    into_calculation_response(result)
        .into_ts()
        .map_err(Into::into)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Greetings from Rust")
}

#[wasm_bindgen]
pub fn inspect_text(input: &str) -> String {
    let byte_length = input.len();

    format!("Rust received: '{input}' (UTF-8 bytes: {byte_length})")
}
