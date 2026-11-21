#[cfg(test)]

mod test {
    use crate::{
        BinaryOperator, Expression, Lexer, Parser, ParserError, Token, TokenKind, UnaryOperator,
    };

    #[test]
    fn parses_number() {
        let tokens = vec![
            Token {
                kind: TokenKind::Number("42".into()),
                position: 0,
            },
            Token {
                kind: TokenKind::EOF,
                position: 2,
            },
        ];

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.parse_primary(), Ok(Expression::Number("42".into())));
    }

    #[test]
    fn multiplication_has_higher_precedence_than_addition() {
        let mut lexer = Lexer::new("2 + 3 * 4");
        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Binary {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::Number("2".into())),
                right: Box::new(Expression::Binary {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(Expression::Number("3".into())),
                    right: Box::new(Expression::Number("4".into())),
                }),
            }
        );
    }

    #[test]
    fn subtraction_is_left_associative() {
        let mut lexer = Lexer::new("10 - 3 - 2");
        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse_expression().unwrap();

        assert_eq!(
            expression,
            Expression::Binary {
                operator: BinaryOperator::Subtract,
                left: Box::new(Expression::Binary {
                    operator: BinaryOperator::Subtract,
                    left: Box::new(Expression::Number("10".into())),
                    right: Box::new(Expression::Number("3".into())),
                }),
                right: Box::new(Expression::Number("2".into())),
            }
        );
    }

    fn parse(input: &str) -> Result<Expression, ParserError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().expect("Lexer failed");

        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    fn number(value: &str) -> Expression {
        Expression::Number(value.to_string())
    }

    fn binary(operator: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn unary(operator: UnaryOperator, operand: Expression) -> Expression {
        Expression::Unary {
            operator,
            operand: Box::new(operand),
        }
    }

    #[test]
    fn parses_integer() {
        assert_eq!(parse("42"), Ok(number("42")));
    }

    #[test]
    fn parses_decimal() {
        assert_eq!(parse("123.45"), Ok(number("123.45")));
    }

    #[test]
    fn parses_addition() {
        assert_eq!(
            parse("2 + 3"),
            Ok(binary(BinaryOperator::Add, number("2"), number("3"),))
        );
    }

    #[test]
    fn parses_subtraction() {
        assert_eq!(
            parse("10 - 5"),
            Ok(binary(BinaryOperator::Subtract, number("10"), number("5"),))
        );
    }

    #[test]
    fn division_has_higher_precedence_than_subtraction() {
        assert_eq!(
            parse("10 - 8 / 2"),
            Ok(binary(
                BinaryOperator::Subtract,
                number("10"),
                binary(BinaryOperator::Divide, number("8"), number("2"),),
            ))
        );
    }

    #[test]
    fn modulo_is_parsed_at_multiplicative_precedence() {
        assert_eq!(
            parse("10 + 8 % 3"),
            Ok(binary(
                BinaryOperator::Add,
                number("10"),
                binary(BinaryOperator::Modulo, number("8"), number("3"),),
            ))
        );
    }

    #[test]
    fn parses_negative_number() {
        assert_eq!(parse("-5"), Ok(unary(UnaryOperator::Negative, number("5"))));
    }

    #[test]
    fn parses_positive_number() {
        assert_eq!(parse("+5"), Ok(unary(UnaryOperator::Positive, number("5"))));
    }

    #[test]
    fn parses_unary_operator_after_multiplication() {
        assert_eq!(
            parse("2 * -3"),
            Ok(binary(
                BinaryOperator::Multiply,
                number("2"),
                unary(UnaryOperator::Negative, number("3")),
            ))
        );
    }

    #[test]
    fn parses_chained_unary_operators() {
        assert_eq!(
            parse("--5"),
            Ok(unary(
                UnaryOperator::Negative,
                unary(UnaryOperator::Negative, number("5")),
            ))
        );
    }

    #[test]
    fn parses_power() {
        assert_eq!(
            parse("2 ^ 3"),
            Ok(binary(BinaryOperator::Power, number("2"), number("3"),))
        );
    }

    #[test]
    fn power_is_right_associative() {
        assert_eq!(
            parse("2 ^ 3 ^ 2"),
            Ok(binary(
                BinaryOperator::Power,
                number("2"),
                binary(BinaryOperator::Power, number("3"), number("2"),),
            ))
        );
    }

    #[test]
    fn power_has_higher_precedence_than_unary_minus() {
        assert_eq!(
            parse("-2 ^ 2"),
            Ok(unary(
                UnaryOperator::Negative,
                binary(BinaryOperator::Power, number("2"), number("2"),),
            ))
        );
    }

    #[test]
    fn parentheses_override_power_precedence() {
        assert_eq!(
            parse("(-2) ^ 2"),
            Ok(binary(
                BinaryOperator::Power,
                unary(UnaryOperator::Negative, number("2")),
                number("2"),
            ))
        );
    }

    #[test]
    fn parses_negative_exponent() {
        assert_eq!(
            parse("2 ^ -3"),
            Ok(binary(
                BinaryOperator::Power,
                number("2"),
                unary(UnaryOperator::Negative, number("3")),
            ))
        );
    }

    #[test]
    fn parses_parenthesized_expression() {
        assert_eq!(
            parse("(2 + 3)"),
            Ok(binary(BinaryOperator::Add, number("2"), number("3"),))
        );
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(
            parse("(2 + 3) * 4"),
            Ok(binary(
                BinaryOperator::Multiply,
                binary(BinaryOperator::Add, number("2"), number("3"),),
                number("4"),
            ))
        );
    }

    #[test]
    fn parses_unary_function() {
        assert_eq!(
            parse("sqrt(16)"),
            Ok(Expression::FunctionCall {
                name: "sqrt".into(),
                arguments: vec![number("16")],
            })
        );
    }

    #[test]
    fn parses_binary_function() {
        assert_eq!(
            parse("nth_root(27, 3)"),
            Ok(Expression::FunctionCall {
                name: "nth_root".into(),
                arguments: vec![number("27"), number("3"),],
            })
        );
    }

    #[test]
    fn parses_expression_as_function_argument() {
        assert_eq!(
            parse("sqrt(16 + 9)"),
            Ok(Expression::FunctionCall {
                name: "sqrt".into(),
                arguments: vec![binary(BinaryOperator::Add, number("16"), number("9"),),],
            })
        );
    }

    #[test]
    fn parses_nested_function_calls() {
        assert_eq!(
            parse("sqrt(nth_root(27, 3) + 1)"),
            Ok(Expression::FunctionCall {
                name: "sqrt".into(),
                arguments: vec![binary(
                    BinaryOperator::Add,
                    Expression::FunctionCall {
                        name: "nth_root".into(),
                        arguments: vec![number("27"), number("3"),],
                    },
                    number("1"),
                ),],
            })
        );
    }

    #[test]
    fn parses_complex_expression() {
        assert_eq!(
            parse("2 + 3 * (4 - 1) ^ 2"),
            Ok(binary(
                BinaryOperator::Add,
                number("2"),
                binary(
                    BinaryOperator::Multiply,
                    number("3"),
                    binary(
                        BinaryOperator::Power,
                        binary(BinaryOperator::Subtract, number("4"), number("1"),),
                        number("2"),
                    ),
                ),
            ))
        );
    }

    #[test]
    fn rejects_trailing_identifier() {
        assert!(parse("2 + 3 abc").is_err());
    }

    #[test]
    fn rejects_adjacent_numbers() {
        assert!(parse("2 3").is_err());
    }

    #[test]
    fn rejects_missing_closing_parenthesis() {
        assert!(parse("(2 + 3").is_err());
    }

    #[test]
    fn rejects_extra_closing_parenthesis() {
        assert!(parse("2 + 3)").is_err());
    }

    #[test]
    fn rejects_missing_function_parenthesis() {
        assert!(parse("sqrt 16").is_err());
    }

    #[test]
    fn rejects_missing_function_argument() {
        assert!(parse("sqrt()").is_err());
    }

    #[test]
    fn rejects_missing_argument_after_comma() {
        assert!(parse("sqrt(16,)").is_err());
    }

    #[test]
    fn rejects_missing_argument_before_comma() {
        assert!(parse("sqrt(,16)").is_err());
    }

    #[test]
    fn rejects_incomplete_expression() {
        assert!(parse("2 +").is_err());
    }

    #[test]
    fn rejects_empty_expression() {
        assert!(parse("").is_err());
    }
}
