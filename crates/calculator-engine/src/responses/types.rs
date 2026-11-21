use crate::Number;

pub enum CalculationResponse {
    Success { value: Number },
    Error { error: CalculationErrorResponse },
}

pub struct CalculationErrorResponse {
    pub code: String,
    pub message: String,
}
