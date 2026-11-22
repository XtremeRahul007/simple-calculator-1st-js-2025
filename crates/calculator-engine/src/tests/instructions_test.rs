#[cfg(test)]
mod test {
    use crate::{
        BinaryOperation, CompiledExpression, Expression, Instruction, InstructionOperation, Lexer,
        Operand, Parser, UnaryOperation, compile,
    };

    #[test]
    fn compiles_number_literal() {
        let expression = Expression::Number("42".into());

        let compiled = compile(&expression).unwrap();

        assert!(compiled.instructions.is_empty());

        assert_eq!(compiled.result, Operand::Literal("42".into()));
    }

    fn compile_input(input: &str) -> CompiledExpression {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse().unwrap();

        compile(&expression).unwrap()
    }

    #[test]
    fn compiles_basic_precedence() {
        let compiled = compile_input("2 + 3 * 4");

        let expected = vec![
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Multiply),
                operands: vec![Operand::Literal("3".into()), Operand::Literal("4".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("2".into()), Operand::Result(0)],
            },
        ];

        assert_eq!(compiled.instructions, expected);
        assert_eq!(compiled.result, Operand::Result(1));
    }

    #[test]
    fn compiles_two_independent_expressions() {
        let compiled = compile_input("(1 + 2) + (3 * 4)");

        let expected = vec![
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("1".into()), Operand::Literal("2".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Multiply),
                operands: vec![Operand::Literal("3".into()), Operand::Literal("4".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Result(0), Operand::Result(1)],
            },
        ];

        assert_eq!(compiled.instructions, expected);
        assert_eq!(compiled.result, Operand::Result(2));
    }

    #[test]
    fn compiles_independent_operations_on_both_sides() {
        let compiled = compile_input("(1 + 2) * (3 + 4)");

        let expected = vec![
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("1".into()), Operand::Literal("2".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("3".into()), Operand::Literal("4".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Multiply),
                operands: vec![Operand::Result(0), Operand::Result(1)],
            },
        ];

        assert_eq!(compiled.instructions, expected);
        assert_eq!(compiled.result, Operand::Result(2));
    }

    #[test]
    fn compiles_deeply_nested_expression() {
        let compiled = compile_input("(1 + 2) * (3 + 4) + (5 * 6)");

        let expected = vec![
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("1".into()), Operand::Literal("2".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Literal("3".into()), Operand::Literal("4".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Multiply),
                operands: vec![Operand::Result(0), Operand::Result(1)],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Multiply),
                operands: vec![Operand::Literal("5".into()), Operand::Literal("6".into())],
            },
            Instruction {
                operation: InstructionOperation::Binary(BinaryOperation::Add),
                operands: vec![Operand::Result(2), Operand::Result(3)],
            },
        ];

        assert_eq!(compiled.instructions, expected);
        assert_eq!(compiled.result, Operand::Result(4));
    }

    #[test]
    fn compiles_nested_binary_expression() {
        let compiled = compile_input("(1 + 2) * (3 + 4)");

        assert_eq!(compiled.instructions.len(), 3);

        assert!(matches!(
            &compiled.instructions[0].operation,
            InstructionOperation::Binary(BinaryOperation::Add)
        ));

        assert_eq!(
            compiled.instructions[0].operands,
            vec![Operand::Literal("1".into()), Operand::Literal("2".into()),]
        );

        assert!(matches!(
            &compiled.instructions[1].operation,
            InstructionOperation::Binary(BinaryOperation::Add)
        ));

        assert_eq!(
            compiled.instructions[1].operands,
            vec![Operand::Literal("3".into()), Operand::Literal("4".into()),]
        );

        assert!(matches!(
            &compiled.instructions[2].operation,
            InstructionOperation::Binary(BinaryOperation::Multiply)
        ));

        assert_eq!(
            compiled.instructions[2].operands,
            vec![Operand::Result(0), Operand::Result(1),]
        );

        assert_eq!(compiled.result, Operand::Result(2));
    }

    #[test]
    fn compiles_nested_function_expression() {
        let compiled = compile_input("square_root(nth_root(27, 3) + 1)");

        assert_eq!(compiled.instructions.len(), 3);

        assert!(matches!(
            &compiled.instructions[0].operation,
            InstructionOperation::Binary(BinaryOperation::NthRoot)
        ));

        assert_eq!(
            compiled.instructions[0].operands,
            vec![Operand::Literal("27".into()), Operand::Literal("3".into()),]
        );

        assert!(matches!(
            &compiled.instructions[1].operation,
            InstructionOperation::Binary(BinaryOperation::Add)
        ));

        assert_eq!(
            compiled.instructions[1].operands,
            vec![Operand::Result(0), Operand::Literal("1".into()),]
        );

        assert!(matches!(
            &compiled.instructions[2].operation,
            InstructionOperation::Unary(UnaryOperation::SquareRoot)
        ));

        assert_eq!(compiled.instructions[2].operands, vec![Operand::Result(1),]);

        assert_eq!(compiled.result, Operand::Result(2));
    }

    #[test]
    fn compiles_negative_nested_expression() {
        let compiled = compile_input("-(2 + 3)");

        assert_eq!(compiled.instructions.len(), 2);

        assert!(matches!(
            &compiled.instructions[0].operation,
            InstructionOperation::Binary(BinaryOperation::Add)
        ));

        assert_eq!(
            compiled.instructions[0].operands,
            vec![Operand::Literal("2".into()), Operand::Literal("3".into()),]
        );

        assert!(matches!(
            &compiled.instructions[1].operation,
            InstructionOperation::Binary(BinaryOperation::Subtract)
        ));

        assert_eq!(
            compiled.instructions[1].operands,
            vec![Operand::Literal("0".into()), Operand::Result(0),]
        );

        assert_eq!(compiled.result, Operand::Result(1));
    }
}
