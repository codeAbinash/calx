#[derive(Debug, Clone, Copy, PartialEq)]
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
    In,
    Return,
    Break,
    Continue,
    Number,
    Str,
    True,
    False,
    Null,
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
    Mod,
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
    Dot,
    Colon,
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
