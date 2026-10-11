use crate::errors::Error;
use crate::lexer::scan::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};

// TODO: Define and implement decimal edge cases and `0b`, `0o`, and `0x` literals together,
// including clear errors for invalid digits and malformed prefixes.
pub(crate) fn number_literal(
    cursor: &mut Cursor,
    tokens: &mut Vec<Token>,
    errors: &mut Vec<Error>,
) {
    let start = cursor.pos;
    cursor.eat_while(|c| c.is_ascii_digit());

    if cursor.peek() == Some('.') && cursor.peek_next().is_some_and(|c| c.is_ascii_digit()) {
        cursor.advance();
        cursor.eat_while(|c| c.is_ascii_digit());
    }

    if cursor
        .peek()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
    {
        cursor.eat_while(|c| c.is_ascii_alphanumeric() || c == '_');
        errors.push(Error::lexer(start, cursor.pos, "invalid number literal"));
        return;
    }

    tokens.push(Token::new(TokenKind::Number, start, cursor.pos));
}
