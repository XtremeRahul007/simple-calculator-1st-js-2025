/*#[derive(Debug, Tsify, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]*/
pub enum CalculationResponse {
    Success { value: String },
    Error { error: CalculationErrorResponse },
}

/*#[derive(Debug, Tsify, Serialize, Deserialize)]*/
pub struct CalculationErrorResponse {
    pub code: String,
    pub message: String,
}
