#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return { body: Expression },
}

#[derive(Debug)]
pub struct Return {
    pub body: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Constant { int: i32 },
}

pub type Identifier = String;
