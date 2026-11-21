mod types;

use crate::types::CalculationResponseTsified;

use calculator_engine::calculate;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_expression(
    operation: &str,
    operands: Vec<String>,
) -> Result<Ts<CalculationResponseTsified>, JsError> {
    let tsified: CalculationResponseTsified = calculate(operation, operands).into();

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
