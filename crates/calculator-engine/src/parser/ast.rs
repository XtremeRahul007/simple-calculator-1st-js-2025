#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Positive,
    Negative,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

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
}
