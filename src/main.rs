mod asm_ast;
mod c_ast;
mod c_to_asm;
mod code_emission;
mod error_reporter;
mod lexer;
mod parser;

use std::{
    env, fs,
    path::Path,
    process::{exit, Command, ExitCode},
};

use c_ast::Program;
use c_to_asm::c_to_asm;
use code_emission::emit;
use error_reporter::print_error;
use lexer::Token;
use parser::Parser;

#[derive(Debug)]
struct CliArgs {
    lex: bool,
    parse: bool,
    codegen: bool,
    assembly: bool,
    file_path: Box<Path>,
}

enum CliArgParseError {
    UnknownFlag(String),
    NoInputFile,
    TooManyInputFiles,
}

fn usage(executable: &str) -> String {
    format!("Usage: {executable} [--lex|--parse|--codegen|-S] <input.c>")
}

fn parse_cli_args(args: &[String]) -> Result<CliArgs, CliArgParseError> {
    let (flags, args): (Vec<_>, Vec<_>) = args.iter().partition(|arg| arg.starts_with("-"));

    let (mut lex, mut parse, mut codegen, mut assembly) = (false, false, false, false);
    for flag in flags {
        match flag.as_str() {
            "--lex" => lex = true,
            "--parse" => parse = true,
            "--codegen" => codegen = true,
            "-S" => assembly = true,
            unknown_flag => return Err(CliArgParseError::UnknownFlag(unknown_flag.to_owned())),
        }
    }

    match args.len() {
        0 => Err(CliArgParseError::NoInputFile),
        1 => Ok(CliArgs {
            lex,
            parse,
            codegen,
            assembly,
            file_path: Box::from(Path::new(args[0])),
        }),
        _ => Err(CliArgParseError::TooManyInputFiles),
    }
}

fn lex(preprocessed_file_path: &Path) -> Vec<Token> {
    let source = fs::read_to_string(preprocessed_file_path).expect("File to be able to be read");
    let mut lexer = lexer::Lexer::new(preprocessed_file_path.display().to_string(), &source);
    match lexer.lex() {
        Ok(tokens) => tokens,
        Err(err) => match err {
            lexer::LexerError::InvalidNumber(token_location) => {
                print_error(
                    "Error during lexing, number is invalid",
                    &source,
                    &token_location,
                );
                exit(2);
            }
            lexer::LexerError::UnrecognisedCharacter(token_location) => {
                print_error(
                    "Error during lexing, unrecognised character",
                    &source,
                    &token_location,
                );
                exit(2);
            }
        },
    }
}

fn parse(tokens: Vec<Token>) -> Program {
    match Parser::parse(&tokens) {
        Err(err) => {
            dbg!(err);
            todo!("Add error handling")
        }
        Ok(program) => program,
    }
}

fn compiler_driver(cli_args: &CliArgs, input_file_path: &Path) {
    let preprocessed_file_path = input_file_path.with_extension("i");
    let assembly_file_path = input_file_path.with_extension("s");
    let executable_file_path = input_file_path.with_extension("");

    // Run the preprocessor
    Command::new("gcc")
        .args([
            "-E",
            "-P",
            &input_file_path.display().to_string(),
            "-o",
            &preprocessed_file_path.display().to_string(),
        ])
        .status()
        .expect("gcc to not fail");

    // Run the compiler
    let tokens = lex(&preprocessed_file_path);
    if cli_args.lex {
        return;
    }

    let c_program = parse(tokens);
    if cli_args.parse {
        return;
    }

    let asm_program = c_to_asm(&c_program);
    if cli_args.codegen {
        return;
    }

    let asm_text = emit(&asm_program).join("\n");
    fs::write(&assembly_file_path, asm_text).expect("writing to a file should not fail");

    // Command::new("gcc")
    //     .args([
    //         "-S",
    //         "-O",
    //         &preprocessed_file_path.display().to_string(),
    //         "-o",
    //         &assembly_file_path.display().to_string(),
    //     ])
    //     .status()
    //     .expect("gcc to not fail");

    Command::new("gcc")
        .args([
            &assembly_file_path.display().to_string(),
            "-o",
            &executable_file_path.display().to_string(),
        ])
        .status()
        .expect("gcc to not fail");
}

fn main() -> ExitCode {
    let raw_args = env::args().collect::<Vec<_>>();
    let executable = raw_args[0].clone();

    let cli_args = match parse_cli_args(&raw_args[1..]) {
        Ok(x) => x,
        Err(err) => {
            match err {
                CliArgParseError::UnknownFlag(flag) => {
                    eprintln!("Unknown cli flag: '{flag}'\n{}", usage(&executable))
                }
                CliArgParseError::NoInputFile => {
                    eprintln!("No input file provided\n{}", usage(&executable))
                }
                CliArgParseError::TooManyInputFiles => eprintln!(
                    "Too many input files provided, but only one can be provided\n{}",
                    usage(&executable)
                ),
            };
            return ExitCode::from(1);
        }
    };

    compiler_driver(&cli_args, &cli_args.file_path);

    ExitCode::SUCCESS
}
