#[cfg(test)]
mod test {
    use crate::{PrecisionContext, calculate_expression};

    #[test]
    fn calculates_expression_through_public_api() {
        let precision = PrecisionContext::default();

        let result = calculate_expression("(1 + 2) * (3 + 4)", &precision).unwrap();

        assert_eq!(result.to_string(), "21");
    }

    #[test]
    fn calculates_literal_through_public_api() {
        let precision = PrecisionContext::default();

        let result = calculate_expression("42", &precision).unwrap();

        assert_eq!(result.to_string(), "42");
    }
}
