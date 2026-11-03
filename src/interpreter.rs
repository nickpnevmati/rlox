use crate::{ast::Expr, token::Token, token::TokenType};

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(std::string::String),
}

impl Value {
    fn as_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(RuntimeError {}),
        }
    }
}

#[derive(Clone, Debug)]
struct RuntimeError {}

pub struct Interpreter;
impl Interpreter {
    pub fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(match literal {
                crate::token::Literal::String(s) => Value::String(s.clone()),
                crate::token::Literal::Number(n) => Value::Number(n.clone()),
                crate::token::Literal::Bool(b) => Value::Bool(b.clone()),
                crate::token::Literal::Nil => Value::Nil,
            }),
            Expr::Set {
                object,
                name,
                value,
            } => self.evaluate_set(object, name, value),
            Expr::Super { keyword, method } => self.evaluate_super(keyword, method),
            Expr::This(token) => self.evaluate_this(),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            Expr::Get { object, name } => self.evaluate_get(object, name),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => self.evaluate_call(callee, paren, arguments),
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Variable(token) => self.evaluate_variable(token),
            Expr::Assign { name, value } => self.evaluate_assign(name, value),
        }
    }

    fn evaluate_set(
        &mut self,
        object: &Box<Expr>,
        name: &Token,
        value: &Box<Expr>,
    ) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_super(&mut self, keyword: &Token, method: &Token) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_this(&self) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let value = self.evaluate_expr(right)?;

        match operator.token_type {
            TokenType::Minus => Ok(Value::Number(-value.as_number()?)),
            TokenType::Bang => Ok(is_truthy(value)),
            _ => Err(RuntimeError {}),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left = self.evaluate_expr(left)?;
        let right = self.evaluate_expr(right)?;

        match operator.token_type {
            TokenType::Minus => Ok(Value::Number(left.as_number()? - right.as_number()?)),
            TokenType::Slash => Ok(Value::Number(left.as_number()? / right.as_number()?)),
            TokenType::Star => Ok(Value::Number(left.as_number()? * right.as_number()?)),
            TokenType::Plus => match left {
                Value::Number(n1) => match right {
                    Value::Number(n2) => Ok(Value::Number(n1 + n2)),
                    _ => Err(RuntimeError {}),
                },
                Value::String(s1) => match right {
                    Value::String(s2) => Ok(Value::String(format!("{}{}", s1, s2))),
                    _ => Err(RuntimeError {}),
                },
                _ => Err(RuntimeError {}),
            },
            TokenType::Greater => Ok(Value::Bool(left.as_number()? > right.as_number()?)),
            TokenType::GreaterEqual => Ok(Value::Bool(left.as_number()? >= right.as_number()?)),
            TokenType::Less => Ok(Value::Bool(left.as_number()? < right.as_number()?)),
            TokenType::LessEqual => Ok(Value::Bool(left.as_number()? <= right.as_number()?)),
            TokenType::BangEqual => Ok(Value::Bool(!is_equal(left, right)?)),
            TokenType::EqualEqual => Ok(Value::Bool(is_equal(left, right)?)),
            _ => todo!(),
        }
    }

    fn evaluate_get(&mut self, object: &Expr, name: &Token) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_variable(&mut self, token: &Token) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn evaluate_assign(&mut self, name: &Token, value: &Expr) -> Result<Value, RuntimeError> {
        todo!()
    }
}

fn is_truthy(val: Value) -> Value {
    match val {
        Value::Nil => Value::Bool(false),
        Value::Bool(_) => val,
        _ => Value::Bool(true),
    }
}

fn is_equal(left: Value, right: Value) -> Result<bool, RuntimeError> {
    match left {
        Value::Bool(b1) => {
            match right {
                Value::Bool(b2) => Ok(b1 == b2),
                _ => Err(RuntimeError {  })
            }
        },
        _ => Err(RuntimeError {  })
    }
}
