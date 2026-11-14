pub mod constants;
pub mod number;
pub mod operations;
pub mod parser;
pub mod precision;

pub use constants::*;
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

    #[test]
    fn natural_log_of_one_is_zero() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = natural_log(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn natural_log_of_non_positive_number_returns_error() {
        let number = Number::parse_exact("-2").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            natural_log(&number, &precision),
            Err(CalculationError::InvalidLogDomain)
        );
    }

    #[test]
    fn common_log_of_one_is_zero() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = common_log(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn common_log_of_one_hundred_is_two() {
        let number = Number::parse_exact("100").unwrap();
        let precision = PrecisionContext::default();

        let result = common_log(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("2").unwrap());
    }

    #[test]
    fn common_log_of_zero_returns_error() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            common_log(&number, &precision),
            Err(CalculationError::InvalidLogDomain)
        );
    }

    #[test]
    fn exponential_of_zero_is_one() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = exponential(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("1").unwrap());
    }

    #[test]
    fn exponential_of_one_is_approximate() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = exponential(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn power_of_ten_zero_is_one() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = power_of_ten(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("1").unwrap());
    }

    #[test]
    fn power_of_ten_two_is_one_hundred() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = power_of_ten(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("100").unwrap());
    }

    #[test]
    fn sine_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = sine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn cosine_of_zero_is_one() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = cosine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("1").unwrap());
    }

    #[test]
    fn tangent_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = tangent(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn sine_returns_approximate_result() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = sine(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn cosine_returns_approximate_result() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = cosine(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn tangent_returns_approximate_result() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = tangent(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn arcsine_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = arcsine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn arccosine_of_one_is_zero() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = arccosine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn arctangent_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = arctangent(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn arcsine_rejects_values_outside_domain() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            arcsine(&number, &precision),
            Err(CalculationError::InvalidTrigonometricDomain)
        );
    }

    #[test]
    fn arccosine_rejects_values_outside_domain() {
        let number = Number::parse_exact("-2").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            arccosine(&number, &precision),
            Err(CalculationError::InvalidTrigonometricDomain)
        );
    }

    #[test]
    fn arctangent_accepts_large_values() {
        let number = Number::parse_exact("1000000").unwrap();
        let precision = PrecisionContext::default();

        let result = arctangent(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn hyperbolic_sine_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_sine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn hyperbolic_cosine_of_zero_is_one() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_cosine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("1").unwrap());
    }

    #[test]
    fn hyperbolic_tangent_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_tangent(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn hyperbolic_sine_returns_approximate_result() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_sine(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn hyperbolic_cosine_returns_approximate_result() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_cosine(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn hyperbolic_tangent_returns_approximate_result() {
        let number = Number::parse_exact("2").unwrap();
        let precision = PrecisionContext::default();

        let result = hyperbolic_tangent(&number, &precision).unwrap();

        assert!(matches!(result, Number::Approximate(_)));
    }

    #[test]
    fn inverse_hyperbolic_sine_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = inverse_hyperbolic_sine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn inverse_hyperbolic_cosine_of_one_is_zero() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        let result = inverse_hyperbolic_cosine(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn inverse_hyperbolic_tangent_of_zero_is_zero() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        let result = inverse_hyperbolic_tangent(&number, &precision).unwrap();

        assert_eq!(result, Number::parse_approximate("0").unwrap());
    }

    #[test]
    fn inverse_hyperbolic_cosine_rejects_values_below_one() {
        let number = Number::parse_exact("0").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            inverse_hyperbolic_cosine(&number, &precision),
            Err(CalculationError::InvalidHyperbolicDomain)
        );
    }

    #[test]
    fn inverse_hyperbolic_tangent_rejects_one() {
        let number = Number::parse_exact("1").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            inverse_hyperbolic_tangent(&number, &precision),
            Err(CalculationError::InvalidHyperbolicDomain)
        );
    }

    #[test]
    fn inverse_hyperbolic_tangent_rejects_negative_one() {
        let number = Number::parse_exact("-1").unwrap();
        let precision = PrecisionContext::default();

        assert_eq!(
            inverse_hyperbolic_tangent(&number, &precision),
            Err(CalculationError::InvalidHyperbolicDomain)
        );
    }

    #[cfg(test)]
    mod tests {
        use dashu::Decimal;

        use super::*;

        fn assert_between(number: Number, lower: &str, upper: &str) {
            let Number::Approximate(value) = number else {
                panic!("Expected an approximate number");
            };

            let lower: Decimal = lower.parse().unwrap();
            let upper: Decimal = upper.parse().unwrap();

            assert!(value > lower);
            assert!(value < upper);
        }

        #[test]
        fn calculates_pi() {
            let precision = PrecisionContext::default();

            assert_between(pi(&precision), "3", "4");
        }

        #[test]
        fn calculates_euler_number() {
            let precision = PrecisionContext::default();

            assert_between(euler(&precision), "2", "3");
        }

        #[test]
        fn calculates_tau() {
            let precision = PrecisionContext::default();

            assert_between(tau(&precision).unwrap(), "6", "7");
        }

        #[test]
        fn calculates_golden_ratio() {
            let precision = PrecisionContext::default();

            assert_between(golden_ratio(&precision).unwrap(), "1", "2");
        }
    }

    #[test]
    fn factorial_of_five_is_120() {
        let precision = PrecisionContext::default();

        let result = factorial(5, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("120").unwrap());
    }

    #[test]
    fn factorial_of_zero_is_one() {
        let precision = PrecisionContext::default();

        let result = factorial(0, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1").unwrap());
    }

    #[test]
    fn permutations_of_five_choose_two_positions() {
        let precision = PrecisionContext::default();

        let result = permutations(5, 2, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("20").unwrap());
    }

    #[test]
    fn combinations_of_five_choose_two() {
        let precision = PrecisionContext::default();

        let result = combinations(5, 2, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("10").unwrap());
    }

    #[test]
    fn combinations_with_zero_selection_is_one() {
        let precision = PrecisionContext::default();

        let result = combinations(5, 0, &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1").unwrap());
    }

    #[test]
    fn permutations_reject_invalid_input() {
        let precision = PrecisionContext::default();

        assert_eq!(
            permutations(3, 5, &precision),
            Err(CalculationError::InvalidCombinatoricsInput)
        );
    }

    #[test]
    fn calculates_gcd() {
        let a = Number::parse_exact("24").unwrap();
        let b = Number::parse_exact("36").unwrap();

        let result = greatest_common_divisor(&a, &b).unwrap();

        assert_eq!(result, Number::parse_exact("12").unwrap());
    }

    #[test]
    fn calculates_gcd_with_negative_numbers() {
        let a = Number::parse_exact("-24").unwrap();
        let b = Number::parse_exact("36").unwrap();

        let result = greatest_common_divisor(&a, &b).unwrap();

        assert_eq!(result, Number::parse_exact("12").unwrap());
    }

    #[test]
    fn gcd_of_two_zeros_is_zero() {
        let zero = Number::parse_exact("0").unwrap();

        let result = greatest_common_divisor(&zero, &zero).unwrap();

        assert_eq!(result, zero);
    }

    #[test]
    fn calculates_lcm() {
        let a = Number::parse_exact("4").unwrap();
        let b = Number::parse_exact("6").unwrap();

        let result = least_common_multiple(&a, &b).unwrap();

        assert_eq!(result, Number::parse_exact("12").unwrap());
    }

    #[test]
    fn lcm_with_zero_is_zero() {
        let a = Number::parse_exact("12").unwrap();
        let zero = Number::parse_exact("0").unwrap();

        let result = least_common_multiple(&a, &zero).unwrap();

        assert_eq!(result, zero);
    }

    #[test]
    fn calculates_modulo() {
        let a = Number::parse_exact("17").unwrap();
        let b = Number::parse_exact("5").unwrap();

        let result = modulo(&a, &b).unwrap();

        assert_eq!(result, Number::parse_exact("2").unwrap());
    }

    #[test]
    fn modulo_of_negative_number_is_non_negative() {
        let a = Number::parse_exact("-17").unwrap();
        let b = Number::parse_exact("5").unwrap();

        let result = modulo(&a, &b).unwrap();

        assert_eq!(result, Number::parse_exact("3").unwrap());
    }

    #[test]
    fn modulo_by_zero_returns_error() {
        let a = Number::parse_exact("17").unwrap();
        let zero = Number::parse_exact("0").unwrap();

        assert_eq!(modulo(&a, &zero), Err(CalculationError::DivisionByZero));
    }

    #[test]
    fn gcd_rejects_fractional_input() {
        let a = Number::parse_exact("3/2").unwrap();
        let b = Number::parse_exact("6").unwrap();

        assert_eq!(
            greatest_common_divisor(&a, &b),
            Err(CalculationError::IntegerInputRequired)
        );
    }

    #[test]
    fn identifies_prime_number() {
        let number = Number::parse_exact("17").unwrap();

        assert!(is_prime(&number).unwrap());
    }

    #[test]
    fn identifies_composite_number() {
        let number = Number::parse_exact("18").unwrap();

        assert!(!is_prime(&number).unwrap());
    }

    #[test]
    fn zero_is_not_prime() {
        let number = Number::parse_exact("0").unwrap();

        assert!(!is_prime(&number).unwrap());
    }

    #[test]
    fn one_is_not_prime() {
        let number = Number::parse_exact("1").unwrap();

        assert!(!is_prime(&number).unwrap());
    }

    #[test]
    fn negative_number_is_not_prime() {
        let number = Number::parse_exact("-7").unwrap();

        assert!(!is_prime(&number).unwrap());
    }

    #[test]
    fn prime_factorization_of_60() {
        let number = Number::parse_exact("60").unwrap();

        let result = prime_factors(&number).unwrap();

        let expected = ["2", "2", "3", "5"].map(|value| Number::parse_exact(value).unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn prime_factorization_of_prime_number() {
        let number = Number::parse_exact("13").unwrap();

        let result = prime_factors(&number).unwrap();

        assert_eq!(result, vec![Number::parse_exact("13").unwrap()]);
    }

    #[test]
    fn prime_factorization_of_negative_number() {
        let number = Number::parse_exact("-12").unwrap();

        let result = prime_factors(&number).unwrap();

        let expected = ["-1", "2", "2", "3"].map(|value| Number::parse_exact(value).unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn prime_factorization_of_zero_returns_error() {
        let number = Number::parse_exact("0").unwrap();

        assert_eq!(
            prime_factors(&number),
            Err(CalculationError::PrimeFactorizationOfZero)
        );
    }

    #[test]
    fn floors_positive_and_negative_numbers() {
        assert_eq!(
            floor(&Number::parse_exact("3.7").unwrap()),
            Number::from_integer(3)
        );

        assert_eq!(
            floor(&Number::parse_exact("-3.7").unwrap()),
            Number::from_integer(-4)
        );
    }

    #[test]
    fn ceilings_positive_and_negative_numbers() {
        assert_eq!(
            ceiling(&Number::parse_exact("3.2").unwrap()),
            Number::from_integer(4)
        );

        assert_eq!(
            ceiling(&Number::parse_exact("-3.2").unwrap()),
            Number::from_integer(-3)
        );
    }

    #[test]
    fn rounds_half_away_from_zero() {
        assert_eq!(
            round(&Number::parse_exact("2.5").unwrap()),
            Number::from_integer(3)
        );

        assert_eq!(
            round(&Number::parse_exact("-2.5").unwrap()),
            Number::from_integer(-3)
        );

        assert_eq!(
            round(&Number::parse_exact("2.4").unwrap()),
            Number::from_integer(2)
        );
    }

    #[test]
    fn truncates_toward_zero() {
        assert_eq!(
            truncate(&Number::parse_exact("3.9").unwrap()),
            Number::from_integer(3)
        );

        assert_eq!(
            truncate(&Number::parse_exact("-3.9").unwrap()),
            Number::from_integer(-3)
        );
    }

    #[test]
    fn calculates_cube_roots() {
        let precision = PrecisionContext::default();

        let result = cube_root(&Number::parse_exact("27").unwrap(), &precision).unwrap();

        assert_eq!(result, Number::from_approximate("3".parse().unwrap()));

        let negative = cube_root(&Number::parse_exact("-27").unwrap(), &precision).unwrap();

        assert_eq!(negative, Number::from_approximate("-3".parse().unwrap()));
    }

    #[test]
    fn calculates_nth_roots() {
        let precision = PrecisionContext::default();

        let result = nth_root(&Number::parse_exact("16").unwrap(), 4, &precision).unwrap();

        assert_eq!(result, Number::from_approximate("2".parse().unwrap()));
    }

    #[test]
    fn rejects_invalid_root_degree() {
        let precision = PrecisionContext::default();

        assert_eq!(
            nth_root(&Number::from_integer(-8), 0, &precision),
            Err(CalculationError::InvalidRootDegree)
        );
    }

    #[test]
    fn rejects_even_root_of_negative_number() {
        let precision = PrecisionContext::default();

        assert_eq!(
            nth_root(&Number::from_integer(-16), 2, &precision),
            Err(CalculationError::EvenRootOfNegative)
        );
    }

    #[test]
    fn calculates_exact_reciprocal() {
        let precision = PrecisionContext::default();

        let result = reciprocal(&Number::parse_exact("4").unwrap(), &precision).unwrap();

        assert_eq!(result, Number::parse_exact("1/4").unwrap());
    }

    #[test]
    fn reciprocal_of_zero_returns_error() {
        let precision = PrecisionContext::default();

        assert_eq!(
            reciprocal(&Number::from_integer(0), &precision),
            Err(CalculationError::DivisionByZero)
        );
    }

    #[test]
    fn calculates_sign() {
        assert_eq!(sign(&Number::from_integer(25)), Number::from_integer(1));
        assert_eq!(sign(&Number::from_integer(-25)), Number::from_integer(-1));
        assert_eq!(sign(&Number::from_integer(0)), Number::from_integer(0));
    }

    #[test]
    fn calculates_minimum() {
        let a = Number::parse_exact("10").unwrap();
        let b = Number::parse_exact("20").unwrap();

        assert_eq!(minimum(&a, &b), a);
    }

    #[test]
    fn calculates_maximum() {
        let a = Number::parse_exact("10").unwrap();
        let b = Number::parse_exact("20").unwrap();

        assert_eq!(maximum(&a, &b), b);
    }

    #[test]
    fn calculates_exact_percentage() {
        let precision = PrecisionContext::default();

        let result = percentage(
            &Number::from_integer(250),
            &Number::from_integer(20),
            &precision,
        )
        .unwrap();

        assert_eq!(result, Number::from_integer(50));
    }

    #[test]
    fn calculates_fractional_percentage() {
        let precision = PrecisionContext::default();

        let result = percentage(
            &Number::from_integer(100),
            &Number::parse_exact("12.5").unwrap(),
            &precision,
        )
        .unwrap();

        assert_eq!(result, Number::parse_exact("25/2").unwrap());
    }
}
