use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{Token, TokenK};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Self {tokens, current: 0}
    }

    pub fn parse_program(&mut self) -> Self {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?)
        }
        Ok(Program { statements })

    }
}