use super::color::Style;
use crate::lexer::{Token, TokenKind};

mod theme {
    pub const COMMENT: (u8, u8, u8) = (0x5c, 0x63, 0x70);
    pub const KEYWORD: (u8, u8, u8) = (0xc6, 0x78, 0xdd);
    pub const NUMBER: (u8, u8, u8) = (0xd1, 0x9a, 0x66);
    pub const VALUE: (u8, u8, u8) = (0xd1, 0x9a, 0x66);
    pub const IDENT: (u8, u8, u8) = (0xe0, 0x6c, 0x95);
    pub const OPERATOR: (u8, u8, u8) = (0xab, 0xb2, 0xbf);
    pub const PUNCTUATION: (u8, u8, u8) = (0xab, 0xb2, 0xbf);
    pub const STRING: (u8, u8, u8) = (0x98, 0xc3, 0x79);
}

use TokenKind as T;

fn color_of(kind: &T) -> (u8, u8, u8) {
    match kind {
        T::Comment => theme::COMMENT,
        T::Number => theme::NUMBER,
        T::Ident => theme::IDENT,
        T::Let | T::Fn | T::If | T::Else | T::For | T::While => theme::KEYWORD,
        T::Add
        | T::Sub
        | T::Mul
        | T::Div
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
        | T::LCurly
        | T::RCurly
        | T::LSquare
        | T::RSquare
        | T::Eof => theme::PUNCTUATION,
        T::Str => theme::STRING,
        T::True | T::False => theme::VALUE,
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
