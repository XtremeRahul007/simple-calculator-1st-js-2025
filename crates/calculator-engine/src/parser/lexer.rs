#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Values
    Number(String),
    Identifier(String),

    // Arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Grouping
    LeftParen,
    RightParen,
    Comma,

    // Special
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub position: usize,
    pub character: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;

        self.position += character.len_utf8();

        Some(character)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.advance();
        }
    }

    pub fn read_number(&mut self) -> String {
        let start = self.position;

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') {
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        self.source[start..self.position].to_string()
    }

    pub fn read_identifier(&mut self) -> String {
        let start = self.position;

        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }

        self.source[start..self.position].to_string()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            let position = self.position;

            let Some(character) = self.peek() else {
                tokens.push(Token {
                    kind: TokenKind::EOF,
                    position,
                });

                break;
            };

            let kind = match character {
                '0'..='9' => TokenKind::Number(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => TokenKind::Identifier(self.read_identifier()),
                '+' => {
                    self.advance();
                    TokenKind::Plus
                }
                '-' => {
                    self.advance();
                    TokenKind::Minus
                }
                '*' => {
                    self.advance();
                    TokenKind::Multiply
                }
                '/' => {
                    self.advance();
                    TokenKind::Divide
                }
                '%' => {
                    self.advance();
                    TokenKind::Modulo
                }
                '^' => {
                    self.advance();
                    TokenKind::Power
                }
                '(' => {
                    self.advance();
                    TokenKind::LeftParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RightParen
                }
                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                _ => {
                    return Err(LexerError {
                        position,
                        character,
                    });
                }
            };

            tokens.push(Token { kind, position });
        }

        Ok(tokens)
    }
}
