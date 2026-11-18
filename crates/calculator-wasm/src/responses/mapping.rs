use calculator_engine::{CalculationError, Number};

use crate::responses::types::*;

pub fn map_calculation_error(error: CalculationError) -> CalculationErrorResponse {
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

pub fn invalid_number_response() -> CalculationResponse {
    CalculationResponse::Error {
        error: CalculationErrorResponse {
            code: "INVALID_NUMBER".to_string(),
            message: "One or more inputs are not valid numbers".to_string(),
        },
    }
}

pub fn into_calculation_response(result: Result<Number, CalculationError>) -> CalculationResponse {
    match result {
        Ok(number) => CalculationResponse::Success {
            value: number.to_string(),
        },
        Err(error) => CalculationResponse::Error {
            error: map_calculation_error(error),
        },
    }
}

pub fn invalid_operation_response() -> CalculationResponse {
    CalculationResponse::Error {
        error: CalculationErrorResponse {
            code: "INVALID_OPERATION".to_string(),
            message: "The requested operation is not supported".to_string(),
        },
    }
}
