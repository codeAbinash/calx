use crate::errors::Error;
use crate::lexer::scan::cursor::Cursor;
use crate::lexer::scan::operators_and_punctuation::emit;
use crate::lexer::token::{Token, TokenKind};

pub(crate) fn slash_or_comment(
    cursor: &mut Cursor,
    tokens: &mut Vec<Token>,
    errors: &mut Vec<Error>,
) {
    match cursor.peek_next() {
        Some('/') => single_line_comment(cursor, tokens),
        Some('*') => multi_line_comment(cursor, tokens, errors),
        _ => emit(cursor, tokens, TokenKind::Div),
    }
}

fn single_line_comment(cursor: &mut Cursor, tokens: &mut Vec<Token>) {
    let start = cursor.pos();
    cursor.advance();
    cursor.advance();
    cursor.eat_while(|ch| ch != '\n');
    tokens.push(Token::new(TokenKind::Comment, start, cursor.pos()));
}

fn multi_line_comment(cursor: &mut Cursor, tokens: &mut Vec<Token>, errors: &mut Vec<Error>) {
    let start = cursor.pos();
    cursor.advance();
    cursor.advance();

    let mut closed = false;
    while let Some(ch) = cursor.peek() {
        if ch == '*' && cursor.peek_next() == Some('/') {
            cursor.advance();
            cursor.advance();
            closed = true;
            break;
        }
        cursor.advance();
    }

    if closed {
        tokens.push(Token::new(TokenKind::Comment, start, cursor.pos()));
    } else {
        errors.push(Error::lexer(
            start,
            cursor.pos(),
            "unterminated block comment",
        ));
    }
}
