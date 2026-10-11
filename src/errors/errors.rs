use crate::utils::Style;

pub struct Error {
    pub start: usize,
    pub end: usize,
    pub message: String,
}

impl Error {
    pub fn lexer(start: usize, end: usize, message: impl Into<String>) -> Self {
        Self {
            start,
            end,
            message: message.into(),
        }
    }
}

pub fn report(path: &str, source: &str, errors: &[Error]) {
    if errors.is_empty() {
        return;
    }

    let style = Style::stderr();
    let cyan = |text: &str| style.ansi("1;36", text);
    let red = |text: &str| style.ansi("1;31", text);
    let white = |text: &str| style.ansi("1;37", text);

    let gutter = errors
        .iter()
        .map(|e| line_of(source, e.start).to_string().len())
        .max()
        .unwrap_or(1);

    let pipe = || eprintln!("{:gutter$} {}", "", cyan("|"));

    for error in errors {
        let (line, column, text) = location(source, error.start);
        let span = span_chars(source, error.start, error.end);
        let caret = format!("{:>pad$}{}", "", "^".repeat(span), pad = column - 1);
        let line_no = format!("{line:>gutter$}");

        eprintln!("{}: {}", red("error"), white(&error.message));
        eprintln!("{} {path}:{line}:{column}", cyan(" -->"));
        pipe();
        eprintln!("{} {} {text}", cyan(&line_no), cyan("|"));
        eprintln!(
            "{:gutter$} {} {} {}",
            "",
            cyan("|"),
            red(&caret),
            white(&error.message)
        );
        pipe();
        eprintln!();
    }

    let label = if errors.len() == 1 { "error" } else { "errors" };
    eprintln!("{}: {} {label} emitted", red("error"), errors.len());
}

fn line_of(source: &str, byte_index: usize) -> usize {
    source[..byte_index.min(source.len())]
        .bytes()
        .filter(|&b| b == b'\n')
        .count()
        + 1
}

fn location(source: &str, byte_index: usize) -> (usize, usize, &str) {
    let mut line = 1;
    let mut column = 1;

    for (i, ch) in source.char_indices() {
        if i >= byte_index {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    let text = source.lines().nth(line - 1).unwrap_or("");
    (line, column, text)
}

fn span_chars(source: &str, start: usize, end: usize) -> usize {
    let end = end.min(source.len());
    let start = start.min(end);
    source[start..end].chars().count().max(1)
}
