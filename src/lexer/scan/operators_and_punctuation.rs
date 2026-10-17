use crate::errors::Error;
use crate::lexer::scan::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};

use TokenKind as T;

pub(crate) fn emit(cursor: &mut Cursor, tokens: &mut Vec<Token>, kind: T) {
    let start = cursor.pos();
    cursor.advance();
    tokens.push(Token::new(kind, start, cursor.pos()));
}

fn emit_or(cursor: &mut Cursor, tokens: &mut Vec<Token>, next: char, two: T, one: T) {
    if cursor.peek_next() == Some(next) {
        emit_two(cursor, tokens, two);
    } else {
        emit(cursor, tokens, one);
    }
}

fn emit_two(cursor: &mut Cursor, tokens: &mut Vec<Token>, kind: T) {
    let start = cursor.pos();
    cursor.advance();
    cursor.advance();
    tokens.push(Token::new(kind, start, cursor.pos()));
}

fn unexpected_char(start: usize, end: usize, ch: char) -> Error {
    let message = if ch.is_control() {
        format!("unexpected character `U+{:04X}`", u32::from(ch))
    } else {
        format!("unexpected character `{ch}`")
    };
    Error::lexer(start, end, message)
}

pub(crate) fn operators_and_punctuation(
    cursor: &mut Cursor,
    tokens: &mut Vec<Token>,
    errors: &mut Vec<Error>,
) {
    let ch = cursor.peek().unwrap_or('\0');
    let start = cursor.pos();
    match ch {
        '(' => emit(cursor, tokens, T::LParen),
        ')' => emit(cursor, tokens, T::RParen),
        '{' => emit(cursor, tokens, T::LCurly),
        '}' => emit(cursor, tokens, T::RCurly),
        '[' => emit(cursor, tokens, T::LSquare),
        ']' => emit(cursor, tokens, T::RSquare),
        ',' => emit(cursor, tokens, T::Comma),
        '*' => emit(cursor, tokens, T::Mul),
        '+' => emit(cursor, tokens, T::Add),
        '-' => emit(cursor, tokens, T::Sub),
        // '/' => emit(cursor, tokens, T::Div), // handled inside `src/lexer/scan/comments.rs`
        '^' => emit(cursor, tokens, T::BitXor),
        ';' => emit(cursor, tokens, T::Semicolon),

        '!' => emit_or(cursor, tokens, '=', T::NotEq, T::Not),
        '<' => emit_or(cursor, tokens, '=', T::LtEq, T::Lt),
        '>' => emit_or(cursor, tokens, '=', T::GtEq, T::Gt),
        '&' => emit_or(cursor, tokens, '&', T::And, T::BitAnd),
        '|' => emit_or(cursor, tokens, '|', T::Or, T::BitOr),

        '=' => match cursor.peek_next() {
            Some('=') => emit_two(cursor, tokens, T::EqEq),
            Some('>') => emit_two(cursor, tokens, T::Arrow),
            _ => emit(cursor, tokens, T::Eq),
        },

        _ => {
            cursor.advance();
            errors.push(unexpected_char(start, cursor.pos(), ch));
        }
    }
}
