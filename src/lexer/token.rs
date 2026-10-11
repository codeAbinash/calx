pub enum TokenKind {
    Comment,
    Ident,
    // Keywords
    Let,
    Fn,
    If,
    Else,
    For,
    While,
    Number,
    Str,
    True,
    False,
    // Logical operators
    And,
    Or,
    Not,
    // Comparison operators
    NotEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    // Bitwise operators
    BitAnd,
    BitOr,
    BitXor,
    // Arithmetic operators
    Add,
    Sub,
    Mul,
    Div,
    // Punctuation
    Arrow,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    Comma,
    Semicolon,
    Eof,
}

pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}
