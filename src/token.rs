#[derive(Debug, Clone, PartialEq)] // Debug for printing, Clone for copying, PartialEq for comparison

pub enum TokenKind {
    Let,
    Print,
    Ident(String),
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Lparen,
    Rparen,
    Semicolon,
    Eof,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub pos: usize,
}
