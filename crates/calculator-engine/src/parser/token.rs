#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,

    LeftParen,
    RightParen,

    EndOfInput,
}
