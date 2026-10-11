use crate::lexer::scan::cursor::Cursor;

pub(crate) fn skip_whitespace(cursor: &mut Cursor) {
    cursor.eat_while(|c| c.is_whitespace());
}
