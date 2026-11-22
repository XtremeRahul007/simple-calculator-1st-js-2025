#[cfg(test)]
mod test {
    use crate::{
        ExecutionError, Lexer, Number, Operand, Parser, PrecisionContext, compile,
        execution::execute_expression, resolve_operand,
    };

    fn evaluate(input: &str) -> Number {
        let tokens = Lexer::new(input).tokenize().unwrap();
        let expression = Parser::new(tokens).parse().unwrap();
        let compiled = compile(&expression).unwrap();

        execute_expression(&compiled, &PrecisionContext::default()).unwrap()
    }

    #[test]
    fn executes_literal_expression() {
        let result = evaluate("42");

        assert_eq!(result.to_string(), "42");
    }

    #[test]
    fn executes_simple_binary_expression() {
        let result = evaluate("10 + 5");

        assert_eq!(result.to_string(), "15");
    }

    #[test]
    fn executes_operator_precedence() {
        let result = evaluate("2 + 3 * 4");

        assert_eq!(result.to_string(), "14");
    }

    #[test]
    fn executes_parenthesized_expression() {
        let result = evaluate("(2 + 3) * 4");

        assert_eq!(result.to_string(), "20");
    }

    #[test]
    fn executes_nested_expression() {
        let result = evaluate("(1 + 2) * (3 + 4)");

        assert_eq!(result.to_string(), "21");
    }

    #[test]
    fn executes_nested_function_expression() {
        let result = evaluate("square_root(nth_root(27, 3) + 1)");

        assert_eq!(result.to_string(), "2");
    }

    #[test]
    fn executes_negative_expression() {
        let result = evaluate("-(2 + 3)");

        assert_eq!(result.to_string(), "-5");
    }

    #[test]
    fn rejects_invalid_literal() {
        let operand = Operand::Literal("not_a_number".into());

        let result = resolve_operand(&operand, &[]);

        assert!(matches!(result, Err(ExecutionError::InvalidNumber)));
    }

    #[test]
    fn rejects_invalid_result_reference() {
        let operand = Operand::Result(5);

        let result = resolve_operand(&operand, &[]);

        assert!(matches!(
            result,
            Err(ExecutionError::InvalidResultReference { index: 5 })
        ));
    }

    #[test]
    fn propagates_calculation_errors() {
        let tokens = Lexer::new("10 / 0").tokenize().unwrap();
        let expression = Parser::new(tokens).parse().unwrap();
        let compiled = compile(&expression).unwrap();

        let result = execute_expression(&compiled, &PrecisionContext::default());

        assert!(matches!(result, Err(ExecutionError::Calculation(_))));
    }
}
