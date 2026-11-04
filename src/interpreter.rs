use crate::{
    ast::{Block, Expr, Stmt},
    environment::*,
    token::{Token, TokenType},
};

#[derive(Clone, Debug)]
pub struct RuntimeError {}

pub struct Interpreter {
    env: Environments,
    current_scope: EnvId,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env: Environments::new(),
            current_scope: EnvId(0),
        }
    }
    pub fn evaluate_stmt(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => todo!(),
            Stmt::Function(function) => todo!(),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::Block(block) => self.evaluate_block_stmt(block)?,
            Stmt::Class {
                name,
                superclass,
                methods,
            } => todo!(),
            Stmt::Print(expr) => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Var { name, initializer } => self.evaluate_var_stmt(name, initializer)?,
            Stmt::While { condition, body } => todo!(),
        };
        Ok(())
    }

    pub fn evaluate_block_stmt(&mut self, block: Block) -> Result<(), RuntimeError> {
        // self.execute_block(block, )
        todo!()
    }

    fn execute_block(&mut self, block: Block, env: Environments) -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn evaluate_var_stmt(
        &mut self,
        name: Token,
        initializer: Option<Expr>,
    ) -> Result<(), RuntimeError> {
        let value = match initializer {
            Some(expr) => self.evaluate_expr(&expr)?,
            None => Value::Nil,
        };

        // self.env.define(name.lexeme.to_string(), value);
        Ok(())
    }

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
        self.env.get(token.clone())
    }

    fn evaluate_assign(&mut self, name: &Token, value: &Expr) -> Result<Value, RuntimeError> {
        let value = self.evaluate_expr(value)?;
        self.env.assign(name.lexeme.to_string(), value.clone());
        Ok(value)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
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
        Value::Bool(b1) => match right {
            Value::Bool(b2) => Ok(b1 == b2),
            _ => Err(RuntimeError {}),
        },
        _ => Err(RuntimeError {}),
    }
}
