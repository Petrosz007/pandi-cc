use std::num::ParseIntError;

use crate::{
    c_ast::{Expression, FunctionDefinition, Identifier, Program, Statement},
    lexer::{Token, TokenLocation, TokenType},
};

#[derive(Debug)]
pub enum ParseError {
    ParsedParsedAfterEof,
    SyntaxErrorUnexpectedToken {
        expected: TokenType,
        found: TokenType,
        location: TokenLocation,
    },
    IntParseError(ParseIntError, TokenLocation),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    fn advance<'a>(&'a mut self) -> Result<&'a Token, ParseError> {
        match self.tokens.get(self.current) {
            None => Err(ParseError::ParsedParsedAfterEof),
            Some(token) => {
                self.current += 1;
                Ok(token)
            }
        }
    }

    fn consume<'a>(&'a mut self, expected_type: &TokenType) -> Result<&'a Token, ParseError> {
        let token = self.advance()?;

        if token.token_type != *expected_type {
            Err(ParseError::SyntaxErrorUnexpectedToken {
                expected: *expected_type,
                found: token.token_type,
                location: token.location.clone(),
            })
        } else {
            Ok(token)
        }
    }

    fn parse_int(&mut self) -> Result<i32, ParseError> {
        let constant = self.consume(&TokenType::Constant)?;

        match constant.text.parse::<i32>() {
            Err(err) => Err(ParseError::IntParseError(err, constant.location.clone())),
            Ok(int) => Ok(int),
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        let identifier = self.consume(&TokenType::Identifier)?;

        Ok(identifier.text.clone())
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let int = self.parse_int()?;
        Ok(Expression::Constant { int })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(&TokenType::Return)?;
        let expression = self.parse_expression()?;
        self.consume(&TokenType::Semicolon)?;

        Ok(Statement::Return { body: expression })
    }

    fn parse_function_definition(&mut self) -> Result<FunctionDefinition, ParseError> {
        self.consume(&TokenType::Int)?;
        let identifier = self.parse_identifier()?;
        self.consume(&TokenType::ParenOpen)?;
        self.consume(&TokenType::Void)?;
        self.consume(&TokenType::ParenClose)?;
        self.consume(&TokenType::BraceOpen)?;
        let statement = self.parse_statement()?;
        self.consume(&TokenType::BraceClose)?;

        Ok(FunctionDefinition {
            name: identifier.to_string(),
            body: statement,
        })
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let function_definition = self.parse_function_definition()?;

        Ok(Program {
            function_definition,
        })
    }

    pub fn parse(tokens: &[Token]) -> Result<Program, ParseError> {
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }
}
