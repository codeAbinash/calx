mod output;
mod scan;
mod token;

use output::LexOutput;
use scan::{
    Cursor, ident_or_keyword, number_literal, operators_and_punctuation, skip_whitespace,
    slash_or_comment, string_literal,
};
pub use token::{Token, TokenKind};

pub fn tokenize(source: &str) -> LexOutput {
    let mut cursor = Cursor::new(source);
    let mut tokens = Vec::with_capacity(source.len() / 5);
    let mut errors = Vec::new();

    if cursor.peek() == Some('\u{FEFF}') {
        cursor.advance();
    }

    while !cursor.is_end() {
        skip_whitespace(&mut cursor);

        let Some(ch) = cursor.peek() else {
            break;
        };

        match ch {
            '/' => slash_or_comment(&mut cursor, &mut tokens, &mut errors),
            '\'' | '"' => string_literal(&mut cursor, &mut tokens, &mut errors),
            '0'..='9' => number_literal(&mut cursor, &mut tokens, &mut errors),
            'a'..='z' | 'A'..='Z' | '_' => ident_or_keyword(&mut cursor, &mut tokens),
            _ => operators_and_punctuation(&mut cursor, &mut tokens, &mut errors),
        }
    }

    tokens.push(Token::new(TokenKind::Eof, cursor.pos, cursor.pos));

    LexOutput { tokens, errors }
}
