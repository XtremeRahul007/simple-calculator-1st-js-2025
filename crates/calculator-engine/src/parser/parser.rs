use super::{ast::*, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token),
    ExpectedExpression(Token),
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

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EndOfInput)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();

        if !matches!(token, Token::EndOfInput) {
            self.position += 1;
        }

        token
    }

    fn consume(&mut self, expected: &Token) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        let expression = self.parse_expression()?;

        if !matches!(self.current(), Token::EndOfInput) {
            return Err(ParserError::UnexpectedToken(self.current().clone()));
        }

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_term()?;

        loop {
            let operator = match self.current() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_term()?;

            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_unary()?;

        loop {
            let operator = match self.current() {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::Modulo => BinaryOperator::Modulo,
                _ => break,
            };

            self.advance();

            let right = self.parse_unary()?;

            left = Expression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParserError> {
        let operator = match self.current() {
            Token::Plus => Some(UnaryOperator::Positive),
            Token::Minus => Some(UnaryOperator::Negative),
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

        if self.consume(&Token::Power) {
            let right = self.parse_unary()?;

            return Ok(Expression::Binary {
                operator: BinaryOperator::Power,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        match self.advance() {
            Token::Number(value) => Ok(Expression::Number(value)),

            Token::LeftParen => {
                let expression = self.parse_expression()?;

                if !self.consume(&Token::RightParen) {
                    return Err(ParserError::UnexpectedToken(self.current().clone()));
                }

                Ok(expression)
            }

            Token::EndOfInput => Err(ParserError::ExpectedExpression(Token::EndOfInput)),

            token => Err(ParserError::ExpectedExpression(token)),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expression, ParserError> {
    Parser::new(tokens).parse()
}
