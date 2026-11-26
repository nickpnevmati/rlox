use std::fmt::Display;

pub type ValueArray = Vec<Value>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil,
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => b.clone(),
            _ => unreachable!(), // This implicitly trusts the programmer (me lol) to use this
            // function responsibly, we should probably panic here instead
        }
    }

    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => n.clone(),
            _ => unreachable!(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            _ => write!(f, "{}", self),
        }
    }
}
