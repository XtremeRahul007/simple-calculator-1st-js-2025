use calculator_engine::*;
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
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

fn map_calculation_error(error: CalculationError) -> CalculationErrorResponse {
    let (code, message) = match error {
        CalculationError::DivisionByZero => ("DIVISION_BY_ZERO", "Cannot divide by zero"),
        CalculationError::FloatingPointError => (
            "FLOATING_POINT_ERROR",
            "A floating-point calculation failed",
        ),
        CalculationError::SquareRootOfNegative => (
            "SQUARE_ROOT_OF_NEGATIVE",
            "Cannot calculate the square root of a negative number",
        ),
        CalculationError::InvalidPowerDomain => (
            "INVALID_POWER_DOMAIN",
            "The power operation is undefined for these inputs",
        ),
        CalculationError::InvalidLogDomain => (
            "INVALID_LOG_DOMAIN",
            "The logarithm is undefined for this input",
        ),
        CalculationError::InvalidTrigonometricDomain => (
            "INVALID_TRIGONOMETRIC_DOMAIN",
            "The trigonometric operation is undefined for this input",
        ),
        CalculationError::InvalidHyperbolicDomain => (
            "INVALID_HYPERBOLIC_DOMAIN",
            "The hyperbolic operation is undefined for this input",
        ),
        CalculationError::InvalidCombinatoricsInput => (
            "INVALID_COMBINATORICS_INPUT",
            "Invalid input for the combinatorics operation",
        ),
        CalculationError::IntegerInputRequired => (
            "INTEGER_INPUT_REQUIRED",
            "This operation requires an integer input",
        ),
        CalculationError::PrimeFactorizationOfZero => (
            "PRIME_FACTORIZATION_OF_ZERO",
            "Zero cannot be prime-factorized",
        ),
        CalculationError::InvalidRootDegree => {
            ("INVALID_ROOT_DEGREE", "The root degree must be valid")
        }
        CalculationError::EvenRootOfNegative => (
            "EVEN_ROOT_OF_NEGATIVE",
            "An even root of a negative number is not real",
        ),
    };

    CalculationErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    }
}

fn invalid_number_response() -> CalculationResponse {
    CalculationResponse::Error {
        error: CalculationErrorResponse {
            code: "INVALID_NUMBER".to_string(),
            message: "One or more inputs are not valid numbers".to_string(),
        },
    }
}

fn into_calculation_response(result: Result<Number, CalculationError>) -> CalculationResponse {
    match result {
        Ok(number) => CalculationResponse::Success {
            value: number.to_string(),
        },
        Err(error) => CalculationResponse::Error {
            error: map_calculation_error(error),
        },
    }
}

fn invalid_operation_response() -> CalculationResponse {
    CalculationResponse::Error {
        error: CalculationErrorResponse {
            code: "INVALID_OPERATION".to_string(),
            message: "The requested operation is not supported".to_string(),
        },
    }
}

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

enum Operation {
    Divide,
    Multiply,
}

impl Operation {
    fn execute(
        self,
        left: &Number,
        right: &Number,
        precision: &PrecisionContext,
    ) -> Result<Number, CalculationError> {
        match self {
            Self::Divide => divide(left, right, precision),
            Self::Multiply => multiply(left, right, precision),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "divide" => Ok(Self::Divide),
            "multiply" => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
pub fn calculate(
    left: &str,
    right: &str,
    operation: &str,
) -> Result<Ts<CalculationResponse>, JsError> {
    let operation = match Operation::try_from(operation) {
        Ok(operation) => operation,
        Err(_) => return Ok(invalid_operation_response().into_ts()?),
    };

    calculate_with(left, right, |left, right, precision| {
        operation.execute(left, right, precision)
    })
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
