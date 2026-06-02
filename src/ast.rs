



#[derive(Debug, Clone)]
pub struct Program {
    pub statement: Vec<Stmt>,
}



#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Print(Expr),
    Expr(Expr),
}


#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Variable(String),
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Negative,
}


#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
}