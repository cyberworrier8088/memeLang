#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },

    Assign {
        name: String,
        value: Expr,
    },

    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },

    While {
        condition: Expr,
        body: Vec<Stmt>,
    },

    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },


    Return(Expr),
    Print(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Bool(bool),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Plus,
    Negate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,

    Greater,
    GreaterEqual,

    Less,
    LessEqual,
}
