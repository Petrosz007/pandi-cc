use std::{
    env, path::Path, process::{Command, ExitCode}
};

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

fn compiler_driver(input_file_path: &Path) {
    // Run the preprocessor
    Command::new("gcc")
        .args([
            "-E",
            "-P",
            &input_file_path.display().to_string(),
            "-o",
            &input_file_path.with_extension("i").display().to_string(),
        ])
        .status()
        .expect("gcc to not fail");

    // Run the compiler
    // TODO: Replace this with my own compiler
    Command::new("gcc")
        .args([
            "-S",
            "-O",
            &input_file_path.with_extension("i").display().to_string(),
            "-o",
            &input_file_path.with_extension("s").display().to_string(),
        ])
        .status()
        .expect("gcc to not fail");

    Command::new("gcc")
        .args([
            &input_file_path.with_extension("s").display().to_string(),
            "-o",
            &input_file_path.with_extension("").display().to_string(),
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

    compiler_driver(&cli_args.file_path);

    ExitCode::SUCCESS
}
