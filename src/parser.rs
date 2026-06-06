// src\parser.rs


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

        if self.matches(&TokenK::Fn) {
            return self.function_statement();
        }

        if self.matches(&TokenK::Return) {
            return self.return_statement();
        }

        if self.matches(&TokenK::Print) {
            let value = self.expression()?;
            self.optional_semicolon();
            return Ok(Stmt::Print(value));
        }

        if self.matches(&TokenK::If) {
            return self.if_statement();
        }

        if self.matches(&TokenK::While) {
            return self.while_statement();
        }

        if self.matches(&TokenK::For) {
            return self.for_statement();
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

        self.optional_semicolon();

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

        Ok(Stmt::If {
            condition,
            body,
            else_body,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;
        self.consume(&TokenK::LBrace, "expected '{' after condition")?;

        let mut body = Vec::new();
        while !self.check(&TokenK::RBrace) {
            body.push(self.statement()?);
        }
        self.consume(&TokenK::RBrace, "expected '}' after while statement")?;

        Ok(Stmt::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        // syntax: for <ident> in <expression> { <body> }
        let name = match self.advance().kind.clone() {
            TokenK::Ident(name) => name,
            _ => return Err(self.error("expected identifier after `for`")),
        };

        self.consume(&TokenK::In, "expected 'in' after for variable")?;

        let iterable = self.expression()?;

        self.consume(&TokenK::LBrace, "expected '{' after for iterable")?;
        let mut body = Vec::new();
        while !self.check(&TokenK::RBrace) {
            body.push(self.statement()?);
        }
        self.consume(&TokenK::RBrace, "expected '}' after for body")?;

        Ok(Stmt::For { variable: name, iterable, body })
    }

    fn function_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance().kind.clone() {
            TokenK::Ident(name) => name,
            _ => return Err(self.error("expected function name after `fn`")),
        };

        self.consume(&TokenK::LParen, "expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.check(&TokenK::RParen) {
            loop {
                match self.advance().kind.clone() {
                    TokenK::Ident(name) => params.push(name),
                    _ => return Err(self.error("expected parameter name")),
                }
                if !self.matches(&TokenK::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenK::RParen, "expected ')' after function parameters")?;

        self.consume(&TokenK::LBrace, "expected '{' before function body")?;
        let mut body = Vec::new();
        while !self.check(&TokenK::RBrace) {
            body.push(self.statement()?);
        }
        self.consume(&TokenK::RBrace, "expected '}' after function body")?;

        Ok(Stmt::Function { name, params, body })
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.optional_semicolon();
        Ok(Stmt::Return(value))
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

        // Check if current is Ident and next is Equal
        let is_ident = matches!(&self.tokens[self.current].kind, TokenK::Ident(_));
        let next_is_equal = matches!(&self.tokens[self.current + 1].kind, TokenK::Equal);

        if is_ident && next_is_equal {
            let name = match self.advance().kind.clone() {
                TokenK::Ident(n) => n,
                _ => unreachable!(),
            };

            self.advance(); // consume =

            let value = self.expression()?;

            self.optional_semicolon();

            Ok(Some(Stmt::Assign { name, value }))
        } else {
            Ok(None)
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
        let mut expr = match self.advance().kind.clone() {
            TokenK::Number(value) => Expr::Number(value),
            TokenK::String(value) => Expr::String(value),
            TokenK::Ident(name) => Expr::Variable(name),
            TokenK::True => Expr::Bool(true),
            TokenK::False => Expr::Bool(false),
            TokenK::LParen => {
                let expr = self.expression()?;
                self.consume(&TokenK::RParen, "expected `)` after expression")?;
                expr
            }
            TokenK::LBracket => self.list_literal()?,
            _ => return Err(self.error("expected expression")),
        };

        loop {
            if self.matches(&TokenK::LParen) {
                expr = self.finish_call(expr)?;
            } else if self.matches(&TokenK::LBracket) {
                expr = self.finish_index(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn list_literal(&mut self) -> Result<Expr, String> {
        let mut elements = Vec::new();
        if !self.check(&TokenK::RBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.matches(&TokenK::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenK::RBracket, "expected ']' after list literal")?;
        Ok(Expr::List(elements))
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut args = Vec::new();
        if !self.check(&TokenK::RParen) {
            loop {
                args.push(self.expression()?);
                if !self.matches(&TokenK::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenK::RParen, "expected ')' after arguments")?;
        match callee {
            Expr::Variable(name) => Ok(Expr::Call { callee: name, args }),
            _ => Err(self.error("expected function name before call")),
        }
    }

    fn finish_index(&mut self, object: Expr) -> Result<Expr, String> {
        let index = self.expression()?;

        self.consume(&TokenK::RBracket, "expected ']' after index")?;

        Ok(Expr::Index {
            object: Box::new(object),
            index: Box::new(index),
        })
    }

    fn optional_semicolon(&mut self) {
        let _ = self.matches(&TokenK::Semicolon);
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
                | (TokenK::While, TokenK::While)
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
                | (TokenK::LBracket, TokenK::LBracket)
                | (TokenK::RBracket, TokenK::RBracket)
                | (TokenK::Semicolon, TokenK::Semicolon)
                | (TokenK::Eof, TokenK::Eof)
                | (TokenK::Number(_), TokenK::Number(_))
                | (TokenK::String(_), TokenK::String(_))
                | (TokenK::Ident(_), TokenK::Ident(_))
                | (TokenK::Fn, TokenK::Fn)
                | (TokenK::Return, TokenK::Return)
                | (TokenK::Comma, TokenK::Comma)
                | (TokenK::For, TokenK::For)
                | (TokenK::In, TokenK::In)
        )
    }
}
