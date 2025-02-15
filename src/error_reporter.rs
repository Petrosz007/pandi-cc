use crate::lexer::TokenLocation;

fn highlight_token_position(source: &str, location: &TokenLocation) -> String {
    let line = source
        .lines()
        .nth(location.line - 1)
        .expect("the input to have a line where it had an error");

    let error_line = format!(
        "{}{}",
        " ".repeat(1 + (location.line.ilog10() as usize) + 3 + location.column),
        &"^".repeat(location.length),
    );

    format!(
        " {} {} {line}\n{error_line}",
        (location.line).to_string(),
        "|"
    )
}

pub fn print_error(error_message: &str, source: &str, location: &TokenLocation) -> () {
    eprintln!(
        "{}: {}\n {} {}\n{}",
        "error",
        error_message,
        "-->",
        location,
        highlight_token_position(source, location)
    );
}
