use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum CalculationResponse {
    Success { value: String },
    Error { error: CalculationErrorResponse },
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
pub struct CalculationErrorResponse {
    pub code: String,
    pub message: String,
}

#[wasm_bindgen]
#[derive(Debug, Tsify, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    NthRoot,
    Power,
    FractionalPower,
    Percentage,
    GreatestCommonDivisor,
    LeastCommonMultiple,
    Minimum,
    Maximum,
    Permutations,
    Combinations,
    SquareRoot,
    CubeRoot,
    Absolute,
    Factorial,
    Reciprocal,
    Sign,
    Exponential,
    PowerOfTen,
    NaturalLog,
    CommonLog,
    Sine,
    Cosine,
    Tangent,
    Arcsine,
    Arccosine,
    Arctangent,
    HyperbolicSine,
    HyperbolicCosine,
    HyperbolicTangent,
    InverseHyperbolicSine,
    InverseHyperbolicCosine,
    InverseHyperbolicTangent,
    Truncate,
    Round,
    Ceiling,
    Floor,
}
