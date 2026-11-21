use super::color::Style;
use crate::lexer::{Token, TokenKind};

mod theme {
    pub const COMMENT: (u8, u8, u8) = (0x60, 0x72, 0x95);
    pub const KEYWORD: (u8, u8, u8) = (0xcf, 0x65, 0xef);
    pub const NUMBER: (u8, u8, u8) = (0xeb, 0x9e, 0x55);
    pub const VALUE: (u8, u8, u8) = (0xeb, 0x9e, 0x55);
    pub const IDENT: (u8, u8, u8) = (0xf0, 0x5e, 0x91);
    pub const OPERATOR: (u8, u8, u8) = (0xb9, 0xc3, 0xd5);
    pub const PUNCTUATION: (u8, u8, u8) = (0xb9, 0xc3, 0xd5);
    pub const STRING: (u8, u8, u8) = (0x8e, 0xd6, 0x6c);
}

use TokenKind as T;

fn color_of(kind: &T) -> (u8, u8, u8) {
    match kind {
        T::Comment => theme::COMMENT,
        T::Number => theme::NUMBER,
        T::Ident => theme::IDENT,
        T::Let
        | T::Fn
        | T::If
        | T::Else
        | T::For
        | T::While
        | T::In
        | T::Return
        | T::Break
        | T::Continue => theme::KEYWORD,
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
        | T::Arrow => theme::OPERATOR,
        T::LParen
        | T::RParen
        | T::Comma
        | T::Semicolon
        | T::Dot
        | T::Colon
        | T::LCurly
        | T::RCurly
        | T::LSquare
        | T::RSquare
        | T::Eof => theme::PUNCTUATION,
        T::Str => theme::STRING,
        T::True | T::False | T::Null => theme::VALUE,
    }
}

pub fn print_source(source: &str, tokens: &[Token]) {
    let style = Style::stdout();
    let mut at = 0;

    for token in tokens {
        if matches!(token.kind, T::Eof) {
            continue;
        }

        if at < token.start {
            print!("{}", &source[at..token.start]);
        }

        print!("{}", style.rgb(color_of(&token.kind), token.text(source)));
        at = token.end;
    }

    if at < source.len() {
        print!("{}", &source[at..]);
    }
}
