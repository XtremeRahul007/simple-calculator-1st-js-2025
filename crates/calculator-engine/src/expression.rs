use super::*;

#[derive(Debug)]
pub enum ExpressionError {
    Lexer(LexerError),
    Parser(ParserError),
    Compilation(InstructionCompileError),
    Execution(ExecutionError),
}

pub fn calculate_expression(
    input: &str,
    precision: &PrecisionContext,
) -> Result<Number, ExpressionError> {
    // 1. Tokenize
    let tokens = Lexer::new(input)
        .tokenize()
        .map_err(ExpressionError::Lexer)?;

    // 2. Parse
    let expression = Parser::new(tokens)
        .parse()
        .map_err(ExpressionError::Parser)?;

    // 3. Compile
    let compiled = compile(&expression).map_err(ExpressionError::Compilation)?;

    // 4. Execute
    execute_expression(&compiled, precision).map_err(ExpressionError::Execution)
}
