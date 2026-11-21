#[derive(Clone, Copy, PartialEq, Eq)]
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

impl TokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Comment => "Comment",
            Self::Ident => "Ident",
            Self::Let => "Let",
            Self::Fn => "Fn",
            Self::If => "If",
            Self::Else => "Else",
            Self::For => "For",
            Self::While => "While",
            Self::In => "In",
            Self::Return => "Return",
            Self::Break => "Break",
            Self::Continue => "Continue",
            Self::Number => "Number",
            Self::Str => "Str",
            Self::True => "True",
            Self::False => "False",
            Self::Null => "Null",
            Self::And => "And",
            Self::Or => "Or",
            Self::Not => "Not",
            Self::NotEq => "NotEq",
            Self::Eq => "Eq",
            Self::EqEq => "EqEq",
            Self::Gt => "Gt",
            Self::GtEq => "GtEq",
            Self::Lt => "Lt",
            Self::LtEq => "LtEq",
            Self::BitAnd => "BitAnd",
            Self::BitOr => "BitOr",
            Self::BitXor => "BitXor",
            Self::Add => "Add",
            Self::Sub => "Sub",
            Self::Mul => "Mul",
            Self::Div => "Div",
            Self::Mod => "Mod",
            Self::Arrow => "Arrow",
            Self::LParen => "LParen",
            Self::RParen => "RParen",
            Self::LCurly => "LCurly",
            Self::RCurly => "RCurly",
            Self::LSquare => "LSquare",
            Self::RSquare => "RSquare",
            Self::Comma => "Comma",
            Self::Semicolon => "Semicolon",
            Self::Dot => "Dot",
            Self::Colon => "Colon",
            Self::Eof => "Eof",
        }
    }
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
