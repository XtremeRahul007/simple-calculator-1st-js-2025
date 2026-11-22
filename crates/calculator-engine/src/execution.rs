use super::*;

#[derive(Debug)]
pub enum ExecutionError {
    InvalidNumber,
    InvalidResultReference { index: usize },
    Calculation(CalculationError),
}

pub fn resolve_operand(operand: &Operand, results: &[Number]) -> Result<Number, ExecutionError> {
    match operand {
        Operand::Literal(value) => {
            Number::parse_exact(value).map_err(|_| ExecutionError::InvalidNumber)
        }

        Operand::Result(index) => results
            .get(*index)
            .cloned()
            .ok_or(ExecutionError::InvalidResultReference { index: *index }),
    }
}

pub fn execute_instruction(
    instruction: &Instruction,
    results: &[Number],
    precision: &PrecisionContext,
) -> Result<Number, ExecutionError> {
    let operands = instruction
        .operands
        .iter()
        .map(|operand| resolve_operand(operand, results))
        .collect::<Result<Vec<Number>, ExecutionError>>()?;

    match &instruction.operation {
        InstructionOperation::Binary(operation) => operation
            .execute(&operands[0], &operands[1], precision)
            .map_err(ExecutionError::Calculation),

        InstructionOperation::Unary(operation) => operation
            .execute(&operands[0], precision)
            .map_err(ExecutionError::Calculation),
    }
}

pub fn execute_expression(
    compiled: &CompiledExpression,
    precision: &PrecisionContext,
) -> Result<Number, ExecutionError> {
    let mut results: Vec<Number> = Vec::with_capacity(compiled.instructions.len());

    for instruction in &compiled.instructions {
        let result = execute_instruction(instruction, &results, precision)?;

        results.push(result);
    }

    resolve_operand(&compiled.result, &results)
}
