use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

impl Value {
    fn as_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(value) => Ok(*value),
            Value::String(_) | Value::Bool(_) | Value::Function { .. } => {
                Err("expected number, found non-number".to_string())
            }
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Number(value) => *value != 0.0,
            Value::String(value) => !value.is_empty(),
            Value::Bool(value) => *value,
            Value::Function { .. } => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Function { .. } => write!(f, "<function>"),
        }
    }
}

pub struct Interpreter {
    env: HashMap<String, Value>,
}

enum ExecResult {
    Value(Value),
    Return(Value),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<Option<Value>, String> {
        match self.execute_block(&program.statements)? {
            ExecResult::Value(value) | ExecResult::Return(value) => Ok(Some(value)),
        }
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<ExecResult, String> {
        let mut last = Value::Number(0.0);
        for stmt in statements {
            match stmt {
                Stmt::Return(expr) => return Ok(ExecResult::Return(self.eval(expr)?)),
                _ => last = self.execute(stmt)?,
            }
        }
        Ok(ExecResult::Value(last))
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value } => {
                let value = self.eval(value)?;
                self.env.insert(name.clone(), value.clone());
                Ok(value)
            }
            Stmt::Function { name, params, body } => {
                self.env.insert(
                    name.clone(),
                    Value::Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(Value::Number(0.0))
            }
            Stmt::Return(expr) => self.eval(expr),
            Stmt::Print(expr) => {
                let value = self.eval(expr)?;
                println!("{value}");
                Ok(value)
            }
            Stmt::Assign { name, value } => {
                let value = self.eval(value)?;

                if !self.env.contains_key(name) {
                    return Err(format!("undefined variable `{}`", name));
                }

                self.env.insert(name.clone(), value.clone());

                Ok(value)
            }
            Stmt::If {
                condition,
                body,
                else_body,
            } => {
                let result = self.eval(condition)?;
                if result.is_truthy() {
                    match self.execute_block(body)? {
                        ExecResult::Return(value) => return Ok(value),
                        ExecResult::Value(value) => return Ok(value),
                    }
                } else if let Some(else_body) = else_body {
                    match self.execute_block(else_body)? {
                        ExecResult::Return(value) => return Ok(value),
                        ExecResult::Value(value) => return Ok(value),
                    }
                }
                Ok(Value::Number(0.0))
            }

            Stmt::While { condition, body } => {
                let mut last = Value::Number(0.0);
                while self.eval(condition)?.is_truthy() {
                    match self.execute_block(body)? {
                        ExecResult::Return(value) => return Ok(value),
                        ExecResult::Value(value) => last = value,
                    }
                }
                Ok(last)
            }

            Stmt::Expr(expr) => self.eval(expr),
        }
    }

    fn eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Bool(value) => Ok(Value::Bool(*value)),
            Expr::Variable(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("undefined variable `{name}`")),
            Expr::Unary { op, expr } => {
                let value = self.eval(expr)?.as_number()?;
                match op {
                    UnaryOp::Negate => Ok(Value::Number(-value)),
                    UnaryOp::Plus => Ok(Value::Number(value)),
                }
            }
            Expr::Binary { left, op, right } => {
                let left = self.eval(left)?;
                let right = self.eval(right)?;
                match op {
                    BinaryOp::Add => Ok(Value::Number(left.as_number()? + right.as_number()?)),
                    BinaryOp::Subtract => Ok(Value::Number(left.as_number()? - right.as_number()?)),
                    BinaryOp::Multiply => Ok(Value::Number(left.as_number()? * right.as_number()?)),
                    BinaryOp::Divide => Ok(Value::Number(left.as_number()? / right.as_number()?)),
                    BinaryOp::Equal => Ok(Value::Number(if left == right { 1.0 } else { 0.0 })),
                    BinaryOp::NotEqual => Ok(Value::Number(if left != right { 1.0 } else { 0.0 })),
                    BinaryOp::Greater => {
                        Ok(Value::Number(if left.as_number()? > right.as_number()? {
                            1.0
                        } else {
                            0.0
                        }))
                    }
                    BinaryOp::GreaterEqual => {
                        Ok(Value::Number(if left.as_number()? >= right.as_number()? {
                            1.0
                        } else {
                            0.0
                        }))
                    }
                    BinaryOp::Less => {
                        Ok(Value::Number(if left.as_number()? < right.as_number()? {
                            1.0
                        } else {
                            0.0
                        }))
                    }
                    BinaryOp::LessEqual => {
                        Ok(Value::Number(if left.as_number()? <= right.as_number()? {
                            1.0
                        } else {
                            0.0
                        }))
                    }
                }
            }
            Expr::Call { callee, args } => {
                let value = self
                    .env
                    .get(callee)
                    .cloned()
                    .ok_or_else(|| format!("undefined function `{}`", callee))?;

                let (params, body) = match value {
                    Value::Function { params, body } => (params, body),
                    _ => return Err(format!("`{}` is not a function", callee)),
                };

                if params.len() != args.len() {
                    return Err(format!(
                        "expected {} arguments but got {}",
                        params.len(),
                        args.len()
                    ));
                }

                let mut call_env = self.env.clone();
                for (param, arg_expr) in params.iter().zip(args.iter()) {
                    let arg_value = self.eval(arg_expr)?;
                    call_env.insert(param.clone(), arg_value);
                }

                let mut call_interpreter = Interpreter { env: call_env };
                match call_interpreter.execute_block(&body)? {
                    ExecResult::Return(value) | ExecResult::Value(value) => Ok(value),
                }
            }
        }
    }
}
