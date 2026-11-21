use calculator_engine::{CalculationErrorResponse, CalculationResponse};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum CalculationResponseTsified {
    Success {
        value: String,
    },
    Error {
        error: CalculationErrorResponseTsified,
    },
}

#[derive(Debug, Tsify, Serialize, Deserialize)]
pub struct CalculationErrorResponseTsified {
    pub code: String,
    pub message: String,
}

impl From<CalculationErrorResponse> for CalculationErrorResponseTsified {
    fn from(err: CalculationErrorResponse) -> Self {
        Self {
            code: err.code,
            message: err.message,
        }
    }
}

impl From<CalculationResponse> for CalculationResponseTsified {
    fn from(resp: CalculationResponse) -> Self {
        match resp {
            CalculationResponse::Success { value } => CalculationResponseTsified::Success { value },
            CalculationResponse::Error { error } => CalculationResponseTsified::Error {
                error: error.into(),
            },
        }
    }
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
