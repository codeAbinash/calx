use crate::errors;
use crate::lexer::tokenize;
use crate::utils::print_source;
use std::io;

const DEFAULT_FILE: &str = "examples/hello.cx";

pub fn run(path: Option<&str>) -> io::Result<bool> {
    let path = path.unwrap_or(DEFAULT_FILE);
    let source = std::fs::read_to_string(path)?;
    let lexed = tokenize(&source);

    if !lexed.errors.is_empty() {
        errors::report(path, &source, &lexed.errors);
        return Ok(false);
    }

    print_source(&source, &lexed.tokens);
    Ok(true)
}
