use super::*;

pub fn calculate(operation: &str, operands: Vec<String>) -> CalculationResponse {
    let precision = PrecisionContext::default();

    // Binary operations
    if let Ok(operation) = BinaryOperation::try_from(operation) {
        if operands.len() != 2 {
            return invalid_operand_count_response();
        }

        let left = match Number::parse_exact(&operands[0]) {
            Ok(number) => number,
            Err(_) => return invalid_number_response(),
        };

        let right = match Number::parse_exact(&operands[1]) {
            Ok(number) => number,
            Err(_) => return invalid_number_response(),
        };

        let result = operation.execute(&left, &right, &precision);

        return into_calculation_response(result);
    }

    // Unary operations
    if let Ok(operation) = UnaryOperation::try_from(operation) {
        if operands.len() != 1 {
            return invalid_operand_count_response();
        }

        let value = match Number::parse_exact(&operands[0]) {
            Ok(number) => number,
            Err(_) => return invalid_number_response(),
        };

        let result = operation.execute(&value, &precision);

        return into_calculation_response(result);
    }

    // Neither operation category recognizes the name.
    invalid_operation_response()
}
