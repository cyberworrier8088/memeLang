#[derive(Debug, Clone, PartialEq)] // Debug for printing, Clone for copying, PartialEq for comparison

pub enum TokenKind {
    Let,
    Print,

    Ident(String),
    Number(f64),

    Plus,
    Minus,
    Star,
    Slash,

    Equal,

    EqualEqual,
    BangEqual,

    Greater,
    GreaterEqual,

    Less,
    LessEqual,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,

    If,
    Else,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub pos: usize,
}
