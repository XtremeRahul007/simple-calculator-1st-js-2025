mod types;
use calculator_engine::{PrecisionContext, calculate_expression as engine_calculate_expression};
use calculator_engine::{calculate, into_expression_response};

use crate::types::CalculationResponseTsified;

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_operation(
    operation: &str,
    operands: Vec<String>,
) -> Result<Ts<CalculationResponseTsified>, JsError> {
    let tsified: CalculationResponseTsified = calculate(operation, operands).into();

    tsified.into_ts().map_err(Into::into)
}

#[wasm_bindgen]
pub fn calculate_expression(
    expression: &str,
    precision: Option<usize>,
) -> Result<Ts<CalculationResponseTsified>, JsError> {
    let precision = match precision {
        Some(value) => {
            PrecisionContext::new(value).map_err(|error| JsError::new(&format!("{error:?}")))?
        }
        None => PrecisionContext::default(),
    };

    let result = engine_calculate_expression(expression, &precision);

    let response = into_expression_response(result);

    let tsified: CalculationResponseTsified = response.into();

    tsified.into_ts().map_err(Into::into)
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
