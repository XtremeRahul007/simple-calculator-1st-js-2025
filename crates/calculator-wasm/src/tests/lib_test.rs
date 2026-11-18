/*#[cfg(test)]
mod tests {
    use calculator_engine::divide;

    use crate::{BinaryOperation, CalculationResponse, UnaryOperation, calculate_with};

    #[test]
    fn parses_supported_operations() {
        assert!(matches!(
            BinaryOperation::try_from("add"),
            Ok(BinaryOperation::Add)
        ));
        assert!(matches!(
            BinaryOperation::try_from("subtract"),
            Ok(BinaryOperation::Subtract)
        ));
        assert!(matches!(
            BinaryOperation::try_from("divide"),
            Ok(BinaryOperation::Divide)
        ));
        assert!(matches!(
            BinaryOperation::try_from("multiply"),
            Ok(BinaryOperation::Multiply)
        ));
    }

    #[test]
    fn rejects_unsupported_operations() {
        let error = BinaryOperation::try_from("unknown").unwrap_err();

        assert_eq!(error.0, "unknown");
    }

    #[test]
    fn calculation_pipeline_returns_success() {
        let response = calculate_with("10", "2", divide);

        assert!(matches!(
            response,
            CalculationResponse::Success { value } if value == "5"
        ));
    }

    #[test]
    fn calculation_pipeline_rejects_invalid_numbers() {
        let response = calculate_with("hello", "2", divide);

        assert!(matches!(
            response,
            CalculationResponse::Error { error }
                if error.code == "INVALID_NUMBER"
        ));
    }

    #[test]
    fn calculation_pipeline_returns_calculation_errors() {
        let response = calculate_with("10", "0", divide);

        assert!(matches!(
            response,
            CalculationResponse::Error { error }
                if error.code == "DIVISION_BY_ZERO"
        ));
    }

    #[test]
    fn parses_supported_unary_operations() {
        assert!(matches!(
            UnaryOperation::try_from("square_root"),
            Ok(UnaryOperation::SquareRoot)
        ));

        assert!(matches!(
            UnaryOperation::try_from("factorial"),
            Ok(UnaryOperation::Factorial)
        ));

        assert!(matches!(
            UnaryOperation::try_from("absolute"),
            Ok(UnaryOperation::Absolute)
        ));
    }

    #[test]
    fn rejects_unsupported_unary_operations() {
        assert!(UnaryOperation::try_from("power").is_err());
        assert!(UnaryOperation::try_from("").is_err());
    }
}
*/
