#[cfg(test)]
mod tests {
    use crate::parser::*;
    #[test]
    fn tokenizes_basic_expression() {
        let result = tokenize("12 + 3 * 4").unwrap();

        assert_eq!(
            result,
            vec![
                Token::Number("12".into()),
                Token::Plus,
                Token::Number("3".into()),
                Token::Multiply,
                Token::Number("4".into()),
                Token::EndOfInput,
            ]
        );
    }

    #[test]
    fn tokenizes_decimal_numbers() {
        let result = tokenize("12.5 + 0.25").unwrap();

        assert_eq!(
            result,
            vec![
                Token::Number("12.5".into()),
                Token::Plus,
                Token::Number("0.25".into()),
                Token::EndOfInput,
            ]
        );
    }

    #[test]
    fn tokenizes_parentheses() {
        let result = tokenize("(8 - 2) / 3").unwrap();

        assert_eq!(
            result,
            vec![
                Token::LeftParen,
                Token::Number("8".into()),
                Token::Minus,
                Token::Number("2".into()),
                Token::RightParen,
                Token::Divide,
                Token::Number("3".into()),
                Token::EndOfInput,
            ]
        );
    }

    #[test]
    fn ignores_whitespace() {
        assert_eq!(tokenize("  5   +   3 ").unwrap(), tokenize("5+3").unwrap());
    }

    #[test]
    fn rejects_invalid_characters() {
        assert_eq!(
            tokenize("5 + @").unwrap_err(),
            LexerError::InvalidCharacter('@')
        );
    }

    #[test]
    fn rejects_decimal_without_digits() {
        assert_eq!(
            tokenize("5 + .").unwrap_err(),
            LexerError::InvalidNumber(".".into())
        );
    }

    #[test]
    fn tokenizes_scientific_notation() {
        let result = tokenize("1.25e-10 + 6.022E23").unwrap();

        assert_eq!(
            result,
            vec![
                Token::Number("1.25e-10".into()),
                Token::Plus,
                Token::Number("6.022E23".into()),
                Token::EndOfInput,
            ]
        );
    }

    #[test]
    fn tokenizes_positive_exponent() {
        assert_eq!(
            tokenize("3E+8").unwrap(),
            vec![Token::Number("3E+8".into()), Token::EndOfInput,]
        );
    }

    #[test]
    fn tokenizes_exponent_without_decimal() {
        assert_eq!(
            tokenize("2e5").unwrap(),
            vec![Token::Number("2e5".into()), Token::EndOfInput,]
        );
    }

    #[test]
    fn rejects_missing_exponent_digits() {
        assert!(matches!(tokenize("1e+"), Err(LexerError::InvalidNumber(_))));
    }

    #[test]
    fn rejects_multiple_decimal_points() {
        assert!(matches!(
            tokenize("1.2.3"),
            Err(LexerError::InvalidNumber(_))
        ));
    }

    fn parse_input(input: &str) -> Result<Expression, ParserError> {
        let tokens = tokenize(input).unwrap();
        parse(tokens)
    }

    #[test]
    fn parses_number() {
        assert_eq!(parse_input("42").unwrap(), Expression::Number("42".into()));
    }

    #[test]
    fn respects_operator_precedence() {
        let result = parse_input("2 + 3 * 4").unwrap();

        assert_eq!(
            result,
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
    fn parses_parentheses() {
        assert!(parse_input("(2 + 3) * 4").is_ok());
    }

    #[test]
    fn parses_unary_minus() {
        assert!(matches!(
            parse_input("-5").unwrap(),
            Expression::Unary {
                operator: UnaryOperator::Negative,
                ..
            }
        ));
    }

    #[test]
    fn parses_right_associative_power() {
        let result = parse_input("2 ^ 3 ^ 2").unwrap();

        assert!(matches!(
            result,
            Expression::Binary {
                operator: BinaryOperator::Power,
                right,
                ..
            } if matches!(
                *right,
                Expression::Binary {
                    operator: BinaryOperator::Power,
                    ..
                }
            )
        ));
    }

    #[test]
    fn parses_negative_exponent() {
        assert!(parse_input("2 ^ -3").is_ok());
    }

    #[test]
    fn unary_minus_has_lower_precedence_than_power() {
        let result = parse_input("-2 ^ 2").unwrap();

        assert!(matches!(
            result,
            Expression::Unary {
                operator: UnaryOperator::Negative,
                operand,
            } if matches!(
                *operand,
                Expression::Binary {
                    operator: BinaryOperator::Power,
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_unclosed_parentheses() {
        assert!(parse_input("(2 + 3").is_err());
    }

    #[test]
    fn rejects_trailing_operator() {
        assert!(parse_input("2 +").is_err());
    }

    #[test]
    fn rejects_unexpected_tokens() {
        assert!(parse_input("2 3").is_err());
    }
}
