use crate::asm_ast::{FunctionDefinition, Instruction, Operand, Program};

fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(int) => format!("#{int}"),
        Operand::Register => "w0".to_owned(),
    }
}

fn emit_instruction(instruction: &Instruction) -> String {
    let mut instr = match instruction {
        Instruction::Mov { src, dst } => {
            format!("mov {}, {}", emit_operand(dst), emit_operand(src))
        }
        Instruction::Ret => "ret".to_owned(),
    };

    // Prefix the instructions with a tab to make them pretty
    instr.insert(0, '\t');
    instr
}

fn emit_function_definition(function_definition: &FunctionDefinition) -> Vec<String> {
    vec![
        vec![format!("\t.globl {}", function_definition.name())],
        vec![format!("{}:", function_definition.name())],
        function_definition
            .instructions
            .iter()
            .map(emit_instruction)
            .collect(),
    ]
    .concat()
}

pub fn emit(program: &Program) -> Vec<String> {
    vec![emit_function_definition(&program.function_definition)].concat()
}
