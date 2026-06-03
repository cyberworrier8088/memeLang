use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{Token, TokenKind as TokenK};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(&TokenK::Let) {
            return self.let_statement();
        }

        if self.matches(&TokenK::Print) {
            let value = self.expression()?;
            self.consume(&TokenK::Semicolon, "Expected ';' after print statement")?;
            return Ok(Stmt::Print(value));
        }

        let expr = self.expression()?;
        self.consume(&TokenK::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }
    fn let_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance().kind.clone() {
            TokenK::Ident(name) => name,
            _ => return Err(self.error("expected identifier after `let`")),
        };

        self.consume(&TokenK::Equal, "expected `=` after variable name")?;

        let value = self.expression()?;

        self.consume(&TokenK::Semicolon, "expected `;` after let statement")?;

        Ok(Stmt::Let { name, value })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.matches_any(&[TokenK::Plus, TokenK::Minus]) {
            let op = match self.previous().kind {
                TokenK::Plus => BinaryOp::Add,
                TokenK::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.matches_any(&[TokenK::Star, TokenK::Slash]) {
            let op = match self.previous().kind {
                TokenK::Star => BinaryOp::Multiply,
                TokenK::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };

            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches_any(&[TokenK::Minus, TokenK::Plus]) {
            let op = match self.previous().kind {
                TokenK::Minus => UnaryOp::Negate,
                TokenK::Plus => UnaryOp::Plus,
                _ => unreachable!(),
            };
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance().kind.clone() {
            TokenK::Number(value) => Ok(Expr::Number(value)),
            TokenK::Ident(name) => Ok(Expr::Variable(name)),
            TokenK::LParen => {
                let expr = self.expression()?;
                self.consume(&TokenK::RParen, "expected `)` after expression")?;
                Ok(expr)
            }
            _ => Err(self.error("expected expression")),
        }
    }

    fn consume(&mut self, kind: &TokenK, message: &str) -> Result<(), String> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn matches(&mut self, kind: &TokenK) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn matches_any(&mut self, kinds: &[TokenK]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenK) -> bool {
        if self.is_at_end() {
            return false;
        }

        Self::token_kind_matches(&self.peek().kind, kind)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenK::Eof)
    }

    fn error(&self, message: &str) -> String {
        format!("{message} at token {:?}", self.peek())
    }

    fn token_kind_matches(a: &TokenK, b: &TokenK) -> bool {
        matches!(
            (a, b),
            (TokenK::Let, TokenK::Let)
                | (TokenK::Print, TokenK::Print)
                | (TokenK::Plus, TokenK::Plus)
                | (TokenK::Minus, TokenK::Minus)
                | (TokenK::Star, TokenK::Star)
                | (TokenK::Slash, TokenK::Slash)
                | (TokenK::Equal, TokenK::Equal)
                | (TokenK::LParen, TokenK::LParen)
                | (TokenK::RParen, TokenK::RParen)
                | (TokenK::Semicolon, TokenK::Semicolon)
                | (TokenK::Eof, TokenK::Eof)
                | (TokenK::Number(_), TokenK::Number(_))
                | (TokenK::Ident(_), TokenK::Ident(_))
        )
    }
}
