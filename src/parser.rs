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

        if self.matches(&TokenK::If) {
            return self.if_statement();
        }

        if let Some(assign) = self.assignment()? {
            return Ok(assign);
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

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        self.consume(&TokenK::LBrace, "expected '{' after condition")?;

        let mut body = Vec::new();
        while !self.check(&TokenK::RBrace) {
            body.push(self.statement()?);
        }
        self.consume(&TokenK::RBrace, "expected '}' after if statement")?;

        let else_body = if self.matches(&TokenK::Else) {
            self.consume(&TokenK::LBrace, "expected '{' after else")?;
            let mut eb = Vec::new();
            while !self.check(&TokenK::RBrace) {
                eb.push(self.statement()?);
            }
            self.consume(&TokenK::RBrace, "expected '}' after else block")?;
            Some(eb)
        } else {
            None
        };

        Ok(Stmt::If { condition, body, else_body })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.matches_any(&[
            TokenK::EqualEqual,
            TokenK::BangEqual,
            TokenK::Greater,
            TokenK::GreaterEqual,
            TokenK::Less,
            TokenK::LessEqual,
        ]) {
            let op = match self.previous().kind {
                TokenK::EqualEqual => BinaryOp::Equal,
                TokenK::BangEqual => BinaryOp::NotEqual,
                TokenK::Greater => BinaryOp::Greater,
                TokenK::GreaterEqual => BinaryOp::GreaterEqual,
                TokenK::Less => BinaryOp::Less,
                TokenK::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };

            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Option<Stmt>, String> {
        if self.current + 1 >= self.tokens.len() {
            return Ok(None);
        }

        match (
            &self.tokens[self.current].kind,
            &self.tokens[self.current + 1].kind,
        ) {
            (TokenK::Ident(name), TokenK::Equal) => {
                let name = name.clone();

                self.advance();
                self.advance();

                let value = self.expression()?;

                self.consume(&TokenK::Semicolon, "expected ';' after assignment")?;

                Ok(Some(Stmt::Assign { name, value }))
            }
            _ => Ok(None),
        }
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
                | (TokenK::If, TokenK::If)
                | (TokenK::Else, TokenK::Else)
                | (TokenK::Plus, TokenK::Plus)
                | (TokenK::Minus, TokenK::Minus)
                | (TokenK::Star, TokenK::Star)
                | (TokenK::Slash, TokenK::Slash)
                | (TokenK::EqualEqual, TokenK::EqualEqual)
                | (TokenK::BangEqual, TokenK::BangEqual)
                | (TokenK::Greater, TokenK::Greater)
                | (TokenK::GreaterEqual, TokenK::GreaterEqual)
                | (TokenK::Less, TokenK::Less)
                | (TokenK::LessEqual, TokenK::LessEqual)
                | (TokenK::Equal, TokenK::Equal)
                | (TokenK::LParen, TokenK::LParen)
                | (TokenK::RParen, TokenK::RParen)
                | (TokenK::LBrace, TokenK::LBrace)
                | (TokenK::RBrace, TokenK::RBrace)
                | (TokenK::Semicolon, TokenK::Semicolon)
                | (TokenK::Eof, TokenK::Eof)
                | (TokenK::Number(_), TokenK::Number(_))
                | (TokenK::Ident(_), TokenK::Ident(_))
        )
    }
}
