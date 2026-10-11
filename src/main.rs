mod cli;
mod errors;
mod lexer;
mod start;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::execute_command(&args);
}
