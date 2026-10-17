/// Byte-index cursor over source text.
/// `pos` is a UTF-8 byte offset into `source`.
pub struct Cursor<'a> {
    pub(crate) source: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    pub(crate) fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.pos..].chars();
        chars.next()?;
        chars.next()
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        // `pos` is a byte offset. Move past the full UTF-8 character so emojis
        // and other Unicode characters are never split in the middle.
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub(crate) fn is_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    pub(crate) fn eat_while(&mut self, pred: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&pred) {
            self.advance();
        }
    }

    pub(crate) fn slice(&self, start: usize) -> &'a str {
        &self.source[start..self.pos]
    }
}
