use crate::errors;
use crate::lexer::tokenize;
use crate::utils::{print_source, print_tokens};
use std::io;

pub fn run(path: Option<&str>) -> io::Result<bool> {
    let path = match path {
        Some(p) => p,
        None if cfg!(debug_assertions) => "examples/hello.cx",
        None => {
            eprintln!("Usage: cx <FILE>");
            return Ok(false);
        }
    };

    let source = std::fs::read_to_string(path)?;
    let lexed = tokenize(&source);

    if !lexed.errors.is_empty() {
        errors::report(path, &source, &lexed.errors);
        return Ok(false);
    }

    print_source(&source, &lexed.tokens);
    print_tokens(&source, &lexed.tokens);
    Ok(true)
}
