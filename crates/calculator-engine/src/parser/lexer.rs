use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut position = 0;

    while position < chars.len() {
        let current = chars[position];

        if current.is_whitespace() {
            position += 1;
            continue;
        }

        let token = match current {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '%' => Some(Token::Modulo),
            '^' => Some(Token::Power),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            _ => None,
        };

        if let Some(token) = token {
            tokens.push(token);
            position += 1;
            continue;
        }

        if current.is_ascii_digit() || current == '.' {
            let start = position;
            let mut has_digit = false;
            let mut has_decimal = false;

            while position < chars.len() {
                let character = chars[position];

                if character.is_ascii_digit() {
                    has_digit = true;
                    position += 1;
                } else if character == '.' && !has_decimal {
                    has_decimal = true;
                    position += 1;
                } else {
                    break;
                }
            }

            if !has_digit {
                let number: String = chars[start..position].iter().collect();
                return Err(LexerError::InvalidNumber(number));
            }

            if position < chars.len() && matches!(chars[position], 'e' | 'E') {
                position += 1;

                if position < chars.len() && matches!(chars[position], '+' | '-') {
                    position += 1;
                }

                let exponent_start = position;

                while position < chars.len() && chars[position].is_ascii_digit() {
                    position += 1;
                }

                if exponent_start == position {
                    let number: String = chars[start..position].iter().collect();
                    return Err(LexerError::InvalidNumber(number));
                }
            }

            let number: String = chars[start..position].iter().collect();

            if position < chars.len() && chars[position] == '.' {
                return Err(LexerError::InvalidNumber(number));
            }

            tokens.push(Token::Number(number));
            continue;
        }

        return Err(LexerError::InvalidCharacter(current));
    }

    tokens.push(Token::EndOfInput);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
