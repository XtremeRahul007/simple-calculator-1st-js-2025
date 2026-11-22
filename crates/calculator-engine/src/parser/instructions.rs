use super::parser::Expression;
use crate::{BinaryOperation, BinaryOperator, UnaryOperation, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Literal(String),
    Result(usize),
}

#[derive(Debug, PartialEq)]
pub enum InstructionOperation {
    Binary(BinaryOperation),
    Unary(UnaryOperation),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub operation: InstructionOperation,
    pub operands: Vec<Operand>,
}

#[derive(Debug)]
pub struct CompiledExpression {
    pub instructions: Vec<Instruction>,
    pub result: Operand,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionCompileError {
    UnsupportedExpression,

    UnsupportedFunction {
        name: String,
    },

    InvalidArgumentCount {
        name: String,
        expected: usize,
        found: usize,
    },
}

enum ResolvedFunction {
    Unary(UnaryOperation),
    Binary(BinaryOperation),
}

pub fn compile(expression: &Expression) -> Result<CompiledExpression, InstructionCompileError> {
    match expression {
        Expression::Number(value) => Ok(CompiledExpression {
            instructions: Vec::new(),
            result: Operand::Literal(value.clone()),
        }),

        Expression::Binary {
            operator,
            left,
            right,
        } => {
            let mut instructions = Vec::new();

            let left_result = append_compiled(&mut instructions, compile(left)?);

            let right_result = append_compiled(&mut instructions, compile(right)?);

            instructions.push(Instruction {
                operation: InstructionOperation::Binary(map_binary_operator(*operator)),
                operands: vec![left_result, right_result],
            });

            let result_index = instructions.len() - 1;

            Ok(CompiledExpression {
                instructions,
                result: Operand::Result(result_index),
            })
        }

        Expression::Unary { operator, operand } => {
            let mut instructions = Vec::new();

            let operand = append_compiled(&mut instructions, compile(operand)?);

            match operator {
                UnaryOperator::Positive => {
                    let result = operand;

                    Ok(CompiledExpression {
                        instructions,
                        result,
                    })
                }

                UnaryOperator::Negative => {
                    instructions.push(Instruction {
                        operation: InstructionOperation::Binary(BinaryOperation::Subtract),
                        operands: vec![Operand::Literal("0".into()), operand],
                    });

                    let result_index = instructions.len() - 1;

                    Ok(CompiledExpression {
                        instructions,
                        result: Operand::Result(result_index),
                    })
                }
            }
        }

        Expression::FunctionCall { name, arguments } => {
            let function = resolve_function(name)?;

            validate_argument_count(name, &function, arguments.len())?;

            let mut instructions = Vec::new();
            let mut operands = Vec::with_capacity(arguments.len());

            for argument in arguments {
                let compiled = compile(argument)?;

                let result = append_compiled(&mut instructions, compiled);

                operands.push(result);
            }

            let operation = match function {
                ResolvedFunction::Unary(operation) => InstructionOperation::Unary(operation),

                ResolvedFunction::Binary(operation) => InstructionOperation::Binary(operation),
            };

            instructions.push(Instruction {
                operation,
                operands,
            });

            let result_index = instructions.len() - 1;

            Ok(CompiledExpression {
                instructions,
                result: Operand::Result(result_index),
            })
        }
    }
}

fn map_binary_operator(operator: BinaryOperator) -> BinaryOperation {
    match operator {
        BinaryOperator::Add => BinaryOperation::Add,
        BinaryOperator::Subtract => BinaryOperation::Subtract,
        BinaryOperator::Multiply => BinaryOperation::Multiply,
        BinaryOperator::Divide => BinaryOperation::Divide,
        BinaryOperator::Modulo => BinaryOperation::Modulo,
        BinaryOperator::Power => BinaryOperation::Power,
    }
}

fn resolve_function(name: &str) -> Result<ResolvedFunction, InstructionCompileError> {
    if let Ok(operation) = UnaryOperation::try_from(name) {
        return Ok(ResolvedFunction::Unary(operation));
    }

    if let Ok(operation) = BinaryOperation::try_from(name) {
        return Ok(ResolvedFunction::Binary(operation));
    }

    Err(InstructionCompileError::UnsupportedFunction {
        name: name.to_string(),
    })
}

fn validate_argument_count(
    name: &str,
    function: &ResolvedFunction,
    found: usize,
) -> Result<(), InstructionCompileError> {
    let expected = match function {
        ResolvedFunction::Unary(_) => 1,
        ResolvedFunction::Binary(_) => 2,
    };

    if found != expected {
        return Err(InstructionCompileError::InvalidArgumentCount {
            name: name.to_string(),
            expected,
            found,
        });
    }

    Ok(())
}

fn offset_operand(operand: &mut Operand, offset: usize) {
    if let Operand::Result(index) = operand {
        *index += offset;
    }
}

fn offset_instruction(instruction: &mut Instruction, offset: usize) {
    for operand in &mut instruction.operands {
        offset_operand(operand, offset);
    }
}

fn append_compiled(target: &mut Vec<Instruction>, mut compiled: CompiledExpression) -> Operand {
    let offset = target.len();

    for instruction in &mut compiled.instructions {
        offset_instruction(instruction, offset);
    }

    let mut result = compiled.result;
    offset_operand(&mut result, offset);

    target.extend(compiled.instructions);

    result
}
