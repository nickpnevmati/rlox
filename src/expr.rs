// The expression grammar from paragraph 6
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
//
// Example
// LITERAL EQUALEQUAL LITERAL
use crate::lexer::{Literal, Token};

pub enum Expr {
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Set(SetEpxr),
    Super(SuperExpr),
    This(ThisExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

pub struct LiteralExpr {
    pub value: Literal,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct SetEpxr {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct SuperExpr {
    pub keyword: Token,
    pub method: Token,
}

pub struct ThisExpr {
    pub keyword: Token,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Token,
}

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}
