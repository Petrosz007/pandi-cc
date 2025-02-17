use crate::{asm_ast, c_ast};

fn convert_expression(expression: &c_ast::Expression) -> asm_ast::Operand {
    match expression {
        c_ast::Expression::Constant { int } => asm_ast::Operand::Imm(*int),
    }
}

fn convert_statement(statement: &c_ast::Statement) -> Vec<asm_ast::Instruction> {
    use asm_ast::Instruction::*;

    match statement {
        c_ast::Statement::Return { body } => vec![
            Mov {
                src: convert_expression(body),
                dst: asm_ast::Operand::Register,
            },
            Ret,
        ],
    }
}

fn convert_function_definition(
    function_definition: &c_ast::FunctionDefinition,
) -> asm_ast::FunctionDefinition {
    asm_ast::FunctionDefinition {
        name: function_definition.name.clone(),
        instructions: convert_statement(&function_definition.body),
    }
}

pub fn c_to_asm(program: &c_ast::Program) -> asm_ast::Program {
    asm_ast::Program {
        function_definition: convert_function_definition(&program.function_definition),
    }
}
