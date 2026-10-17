use crate::errors::Error;
use crate::lexer::scan::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};

pub(crate) fn string_literal(
    cursor: &mut Cursor,
    tokens: &mut Vec<Token>,
    errors: &mut Vec<Error>,
) {
    let start = cursor.pos();
    let str_start_char = cursor.peek().unwrap_or('"'); // " or ' character

    cursor.advance();

    while let Some(ch) = cursor.peek() {
        if ch == '\n' || ch == str_start_char {
            break;
        }

        if ch == '\\' {
            // Treat `\"`, `\\`, `\n`, and similar pairs as string content.
            // The character after `\\` cannot close the string.
            cursor.advance();
            if cursor.peek().is_some_and(|next| next != '\n') {
                cursor.advance();
            }
        } else {
            cursor.advance();
        }
    }

    if cursor.peek() == Some(str_start_char) {
        cursor.advance();
    } else {
        errors.push(Error::lexer(
            start,
            cursor.pos(),
            "unterminated string literal",
        ));
        return;
    }

    tokens.push(Token::new(TokenKind::Str, start, cursor.pos()));
}
