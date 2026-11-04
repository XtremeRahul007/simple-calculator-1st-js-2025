use wasm_bindgen::prelude::*;

use calculator_engine::Number;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Greetings from Rust")
}
