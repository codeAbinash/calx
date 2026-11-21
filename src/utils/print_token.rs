use super::color::Style;
use super::table::{Cell, Table};
use crate::lexer::{Token, TokenKind as T};

mod theme {
    pub const COMMENT: (u8, u8, u8) = (0x60, 0x72, 0x95);
    pub const KEYWORD: (u8, u8, u8) = (0xcf, 0x65, 0xef);
    pub const NUMBER: (u8, u8, u8) = (0xeb, 0x9e, 0x55);
    pub const IDENT: (u8, u8, u8) = (0xf0, 0x5e, 0x91);
    pub const OPERATOR: (u8, u8, u8) = (0xb9, 0xc3, 0xd5);
    pub const PUNCT: (u8, u8, u8) = (0xb9, 0xc3, 0xd5);
    pub const STRING: (u8, u8, u8) = (0x8e, 0xd6, 0x6c);
}

fn color_of(kind: T) -> (u8, u8, u8) {
    use theme::*;
    match kind {
        T::Comment => COMMENT,
        T::Ident => IDENT,
        T::Str => STRING,
        T::Number | T::True | T::False | T::Null => NUMBER,
        T::Let
        | T::Fn
        | T::If
        | T::Else
        | T::For
        | T::While
        | T::In
        | T::Return
        | T::Break
        | T::Continue => KEYWORD,
        T::Add
        | T::Sub
        | T::Mul
        | T::Div
        | T::Mod
        | T::Eq
        | T::Gt
        | T::GtEq
        | T::Lt
        | T::LtEq
        | T::EqEq
        | T::NotEq
        | T::Not
        | T::And
        | T::Or
        | T::BitAnd
        | T::BitOr
        | T::BitXor
        | T::Arrow => OPERATOR,
        _ => PUNCT,
    }
}

pub fn print_source(source: &str, tokens: &[Token]) {
    let style = Style::stdout();
    let mut at = 0;
    for token in tokens.iter().filter(|t| t.kind != T::Eof) {
        if at < token.start {
            print!("{}", &source[at..token.start]);
        }
        print!("{}", style.rgb(color_of(token.kind), token.text(source)));
        at = token.end;
    }
    if at < source.len() {
        print!("{}", &source[at..]);
    }
}

pub fn print_tokens(source: &str, tokens: &[Token]) {
    let mut table = Table::new(["KIND", "LEXEME"])
        .width(0, 16, 24)
        .width(1, 20, 28)
        .flow();

    for token in tokens {
        let color = color_of(token.kind);
        let lexeme = if token.kind == T::Eof {
            ""
        } else {
            token.text(source)
        };
        table.push([
            Cell::colored(token.kind.as_str(), color),
            Cell::colored(lexeme, color),
        ]);
    }

    table.print();
}
