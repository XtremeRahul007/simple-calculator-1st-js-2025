mod errors;
mod operations;
mod responses;
mod tests;

use calculator_engine::*;
use operations::*;
use responses::*;

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(
    operation: &str,
    operands: Vec<String>,
) -> Result<Ts<CalculationResponse>, JsError> {
    let precision = PrecisionContext::default();

    // Binary operations
    if let Ok(operation) = BinaryOperation::try_from(operation) {
        if operands.len() != 2 {
            return invalid_operand_count_response()
                .into_ts()
                .map_err(Into::into);
        }

        let left = match Number::parse_exact(&operands[0]) {
            Ok(number) => number,
            Err(_) => {
                return invalid_number_response().into_ts().map_err(Into::into);
            }
        };

        let right = match Number::parse_exact(&operands[1]) {
            Ok(number) => number,
            Err(_) => {
                return invalid_number_response().into_ts().map_err(Into::into);
            }
        };

        let result = operation.execute(&left, &right, &precision);

        return into_calculation_response(result)
            .into_ts()
            .map_err(Into::into);
    }

    // Unary operations
    if let Ok(operation) = UnaryOperation::try_from(operation) {
        if operands.len() != 1 {
            return invalid_operand_count_response()
                .into_ts()
                .map_err(Into::into);
        }

        let value = match Number::parse_exact(&operands[0]) {
            Ok(number) => number,
            Err(_) => {
                return invalid_number_response().into_ts().map_err(Into::into);
            }
        };

        let result = operation.execute(&value, &precision);

        return into_calculation_response(result)
            .into_ts()
            .map_err(Into::into);
    }

    // Neither operation category recognizes the name.
    invalid_operation_response().into_ts().map_err(Into::into)
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
