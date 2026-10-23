use crate::lexer::scan::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};

pub(crate) fn ident_or_keyword(cursor: &mut Cursor, tokens: &mut Vec<Token>) {
    let start = cursor.pos();
    cursor.eat_while(|c| c.is_ascii_alphanumeric() || c == '_');

    let kind = match cursor.slice(start) {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Fn,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "while" => TokenKind::While,
        "in" => TokenKind::In,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "null" => TokenKind::Null,
        _ => TokenKind::Ident,
    };

    tokens.push(Token::new(kind, start, cursor.pos()));
}
