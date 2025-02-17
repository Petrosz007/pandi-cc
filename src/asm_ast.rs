use crate::c_ast;

#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: c_ast::Identifier,
    pub instructions: Vec<Instruction>,
}

impl FunctionDefinition {
    pub fn name(&self) -> String {
        let mut name = self.name.clone();
        name.insert(0, '_');
        name
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register,
}
