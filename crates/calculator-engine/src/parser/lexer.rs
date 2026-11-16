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
