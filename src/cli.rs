use crate::start::run;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("Usage: {NAME} [OPTIONS] [FILE]");
    println!();
    println!("Options:");
    println!("  -h, --help     Show this help message");
    println!("  -v, --version  Show version information");
}

fn print_version() {
    println!("{NAME} version {VERSION}");
}

fn try_run(path: Option<&str>) {
    match run(path) {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(e) => {
            match path {
                Some(path) => eprintln!("Error reading '{path}': {e}"),
                None => eprintln!("Error: {e}"),
            }
            std::process::exit(1);
        }
    }
}

pub fn execute_command(args: &[String]) {
    match args.get(1).map(String::as_str) {
        None => try_run(None),
        Some("-v" | "--version") => print_version(),
        Some("-h" | "--help") => print_help(),
        Some(option) if option.starts_with('-') => {
            eprintln!("Unknown option: {option}");
            std::process::exit(1);
        }
        Some(path) => try_run(Some(path)),
    }
}
