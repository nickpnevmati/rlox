use std::collections::HashMap;

use crate::{interpreter::RuntimeError, token::Token};

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(std::string::String),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(RuntimeError {}),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EnvId(pub usize);

struct Scope {
    values: HashMap<String, Value>,
    enclosing: Option<EnvId>,
}

impl Scope {
    pub fn root() -> Scope {
        Scope {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        let key = name.lexeme.to_string();
        match self.values.get(&key) {
            Some(ov) => Ok(ov.clone()),
            None => Err(RuntimeError {}),
        }
    }

    pub fn define(&mut self, name: String, val: Value) {
        self.values.insert(name, val);
    }

    pub fn assign(&mut self, name: String, val: Value) -> Result<(), RuntimeError> {
        if !self.values.contains_key(&name) {
            return Err(RuntimeError {});
        }
        self.values.insert(name, val);
        Ok(())
    }
}

pub struct Environments {
    scopes: Vec<Scope>,
}

impl Environments {
    pub fn new() -> Environments {
        Environments {
            scopes: vec![Scope::root()],
        }
    }

    pub fn new_scope(&mut self, enclosing: EnvId) -> EnvId {
        self.scopes.push(Scope {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        });
        EnvId(self.scopes.len() - 1)
    }

    pub fn get(&self, env: EnvId, name: &str) -> Result<Value, RuntimeError> {
        let mut current = Some(env);
        while let Some(id) = current {
            let scope = &self.scopes[id.0];
            if let Some(val) = scope.values.get(name) {
                return Ok(val.clone());
            }
            current = scope.enclosing;
        }
        Err(RuntimeError {})
    }
}
