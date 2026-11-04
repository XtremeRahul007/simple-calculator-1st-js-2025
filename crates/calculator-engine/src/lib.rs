pub mod number;
pub mod operations;
pub mod precision;

pub use number::*;
pub use operations::*;
pub use precision::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_exact_decimal() {
        let number = Number::parse_exact("2.75").unwrap();

        assert_eq!(number, Number::Exact("11/4".parse().unwrap()));
    }

    #[test]
    fn parses_exact_fraction() {
        let number = Number::parse_exact("1/3").unwrap();

        assert_eq!(number, Number::Exact("1/3".parse().unwrap()));
    }

    #[test]
    fn parses_approximate_decimal() {
        let number = Number::parse_approximate("3.141592653589793").unwrap();

        assert!(matches!(number, Number::Approximate(_)));
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(Number::parse_exact("Hello").is_err());
        assert!(Number::parse_approximate("Hello").is_err());
    }

    #[test]
    fn adds_exact_numbers() {
        let a = Number::parse_exact("1/3").unwrap();
        let b = Number::parse_exact("1/6").unwrap();

        let precision = PrecisionContext::default();
        let result = add(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1/2").unwrap());
    }

    #[test]
    fn adds_exact_and_approximate_numbers() {
        let a = Number::parse_exact("1/2").unwrap();
        let b = Number::parse_approximate("1.5").unwrap();

        let precision = PrecisionContext::default();
        let result = add(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("2").unwrap());
    }

    #[test]
    fn subtracts_exact_numbers() {
        let a = Number::parse_exact("5/6").unwrap();
        let b = Number::parse_exact("1/3").unwrap();

        let precision = PrecisionContext::default();
        let result = subtract(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1/2").unwrap());
    }

    #[test]
    fn multiplies_exact_numbers() {
        let a = Number::parse_exact("2/3").unwrap();
        let b = Number::parse_exact("3/4").unwrap();

        let precision = PrecisionContext::default();
        let result = multiply(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1/2").unwrap());
    }

    #[test]
    fn divides_exact_numbers() {
        let a = Number::parse_exact("5").unwrap();
        let b = Number::parse_exact("2").unwrap();

        let precision = PrecisionContext::default();
        let result = divide(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("5/2").unwrap());
    }

    #[test]
    fn division_by_zero_returns_error() {
        let a = Number::parse_exact("5").unwrap();
        let b = Number::parse_exact("0").unwrap();

        let precision = PrecisionContext::default();
        assert_eq!(
            divide(&a, &b, &precision),
            Err(CalculationError::DivisionByZero)
        );
    }

    #[test]
    fn creates_precision_context() {
        let context = PrecisionContext::new(100).unwrap();

        assert_eq!(context.precision(), 100);
    }

    #[test]
    fn rejects_zero_precision() {
        assert_eq!(PrecisionContext::new(0), Err(PrecisionError::ZeroPrecision));
    }

    #[test]
    fn default_precision_is_50() {
        let context = PrecisionContext::default();

        assert_eq!(context.precision(), 50);
    }

    #[test]
    fn creates_dashu_context() {
        let context = PrecisionContext::new(80).unwrap();
        let float_context = context.float_context();

        assert_eq!(float_context.precision(), 80);
    }

    #[test]
    fn subtracts_mixed_numbers() {
        let a = Number::parse_exact("5").unwrap();
        let b = Number::parse_approximate("2.5").unwrap();
        let precision = PrecisionContext::default();

        let result = subtract(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("2.5").unwrap());
    }

    #[test]
    fn multiplies_mixed_numbers() {
        let a = Number::parse_exact("2").unwrap();
        let b = Number::parse_approximate("1.5").unwrap();
        let precision = PrecisionContext::default();

        let result = multiply(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("3").unwrap());
    }

    #[test]
    fn divides_mixed_numbers() {
        let a = Number::parse_exact("1").unwrap();
        let b = Number::parse_approximate("2").unwrap();
        let precision = PrecisionContext::default();

        let result = divide(&a, &b, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0.5").unwrap());
    }

    #[test]
    fn approximate_division_by_zero_returns_error() {
        let a = Number::parse_exact("5").unwrap();
        let b = Number::parse_approximate("0").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            divide(&a, &b, &precision),
            Err(CalculationError::DivisionByZero)
        );
    }

    #[test]
    fn absolute_value_of_negative_rational() {
        let number = Number::parse_exact("-3/4").unwrap();

        let result = absolute(&number);

        assert_eq!(result, Number::parse_exact("3/4").unwrap());
    }

    #[test]
    fn absolute_value_of_negative_approximate() {
        let number = Number::parse_approximate("-2.5").unwrap();

        let result = absolute(&number);

        assert_eq!(result, Number::parse_approximate("2.5").unwrap());
    }

    #[test]
    fn absolute_value_preserves_positive_exact_number() {
        let number = Number::parse_exact("10").unwrap();

        assert_eq!(absolute(&number), number);
    }

    #[test]
    fn square_root_of_perfect_integer_is_exact() {
        let number = Number::parse_exact("9").unwrap();
        let precision = PrecisionContext::default();

        let result = square_root(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("3").unwrap());
    }

    #[test]
    fn square_root_of_perfect_fraction_is_exact() {
        let number = Number::parse_exact("9/4").unwrap();
        let precision = PrecisionContext::default();

        let result = square_root(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("3/2").unwrap());
    }

    #[test]
    fn square_root_of_non_perfect_integer_is_approximate() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = square_root(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn square_root_of_negative_number_returns_error() {
        let number = Number::parse_exact("-4").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            square_root(&number, &precision),
            Err(CalculationError::SquareRootOfNegative)
        );
    }

    #[test]
    fn square_root_of_zero_is_exact() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = square_root(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("0").unwrap());
    }

    #[test]
    fn power_of_exact_integer_is_exact() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = power(&number, 3, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("8").unwrap());
    }

    #[test]
    fn power_of_exact_fraction_is_exact() {
        let number = Number::parse_exact("2/3").unwrap();
        let precision = PrecisionContext::default();

        let result = power(&number, 2, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("4/9").unwrap());
    }

    #[test]
    fn negative_exponent_produces_exact_fraction() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = power(&number, -3, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1/8").unwrap());
    }

    #[test]
    fn zero_with_negative_exponent_returns_error() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            power(&number, -1, &precision),
            Err(CalculationError::DivisionByZero)
        );
    }

    #[test]
    fn approximate_power_remains_approximate() {
        let number = Number::parse_approximate("1.5").unwrap();
        let precision = PrecisionContext::default();

        let result = power(&number, 2, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn fractional_power_of_two() {
        let base = Number::parse_exact("2").unwrap();
        let exponent = Number::parse_approximate("0.5").unwrap();
        let precision = PrecisionContext::default();

        let result = fractional_power(&base, &exponent, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn fractional_power_with_negative_exponent() {
        let base = Number::parse_exact("4").unwrap();
        let exponent = Number::parse_approximate("-0.5").unwrap();
        let precision = PrecisionContext::default();

        let result = fractional_power(&base, &exponent, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn fractional_power_of_negative_base_returns_error() {
        let base = Number::parse_exact("-2").unwrap();
        let exponent = Number::parse_approximate("0.5").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            fractional_power(&base, &exponent, &precision),
            Err(CalculationError::InvalidPowerDomain)
        );
    }

    #[test]
    fn fractional_power_zero_to_negative_exponent_returns_error() {
        let base = Number::parse_exact("0").unwrap();
        let exponent = Number::parse_approximate("-0.5").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            fractional_power(&base, &exponent, &precision),
            Err(CalculationError::DivisionByZero)
        );
    }
}
