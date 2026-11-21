use crate::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(String),

    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> &Token {
        let current = &self.tokens[self.position];

        if !matches!(current.kind, TokenKind::EOF) {
            self.position += 1;
        }

        current
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let expression = self.parse_expression()?;

        let token = self.peek().clone();

        if token.kind != TokenKind::EOF {
            return Err(ParserError::UnexpectedToken {
                position: token.position,
                found: token.kind,
                expected: "end of expression".into(),
            });
        }

        Ok(expression)
    }

    pub fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.advance().clone();

        match token.kind {
            TokenKind::Number(value) => Ok(Expression::Number(value)),

            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;

                self.expect(TokenKind::RightParen)?;

                Ok(expression)
            }

            TokenKind::Identifier(name) => {
                self.expect(TokenKind::LeftParen)?;

                let mut arguments = Vec::new();

                loop {
                    arguments.push(self.parse_expression()?);

                    if matches!(self.peek().kind, TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }

                self.expect(TokenKind::RightParen)?;

                Ok(Expression::FunctionCall { name, arguments })
            }

            _ if token.kind == TokenKind::EOF => Err(ParserError::UnexpectedEnd {
                position: token.position,
                expected: "an expression".into(),
            }),

            _ => Err(ParserError::UnexpectedToken {
                position: token.position,
                found: token.kind,
                expected: "a number, function call, or opening parenthesis".into(),
            }),
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {
        let token = self.peek().clone();

        if token.kind == expected {
            self.advance();
            return Ok(());
        }

        if token.kind == TokenKind::EOF {
            return Err(ParserError::UnexpectedEnd {
                position: token.position,
                expected: format!("{expected:?}"),
            });
        }

        Err(ParserError::UnexpectedToken {
            position: token.position,
            found: token.kind,
            expected: format!("{expected:?}"),
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_multiplicative()?;

        loop {
            let operator = match &self.peek().kind {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_multiplicative()?;

            expression = Expression::Binary {
                operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParserError> {
        let mut expression = self.parse_unary()?;

        loop {
            let operator = match &self.peek().kind {
                TokenKind::Multiply => BinaryOperator::Multiply,
                TokenKind::Divide => BinaryOperator::Divide,
                TokenKind::Modulo => BinaryOperator::Modulo,
                _ => break,
            };

            self.advance();

            let right = self.parse_unary()?;

            expression = Expression::Binary {
                operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        let operator = match &self.peek().kind {
            TokenKind::Plus => Some(UnaryOperator::Positive),
            TokenKind::Minus => Some(UnaryOperator::Negative),
            _ => None,
        };

        if let Some(operator) = operator {
            self.advance();

            let operand = self.parse_unary()?;

            return Ok(Expression::Unary {
                operator,
                operand: Box::new(operand),
            });
        }

        self.parse_power()
    }

    fn parse_power(&mut self) -> Result<Expression, ParserError> {
        let left = self.parse_primary()?;

        if matches!(self.peek().kind, TokenKind::Power) {
            self.advance();

            let right = self.parse_unary()?;

            return Ok(Expression::Binary {
                operator: BinaryOperator::Power,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ok(left)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken {
        position: usize,
        found: TokenKind,
        expected: String,
    },

    UnexpectedEnd {
        position: usize,
        expected: String,
    },
}
