// src\token.rs


#[derive(Debug, Clone, PartialEq)] // Debug for printing, Clone for copying, PartialEq for comparison

pub enum TokenKind {
    Let,
    Print,

    Ident(String),
    Number(f64),
    String(String),

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
    LBracket,
    RBracket,

    If,
    Else,

    While,

    For,
    In,

    Eof,

    Fn,
    Return,
    Comma,

    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub pos: usize,
}


///------------------------------------------------
/// END OF token.rs.rs
///------------------------------------------------