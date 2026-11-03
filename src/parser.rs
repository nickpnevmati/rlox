use std::fmt::Display;

/* Lox GRAMMAR (lowest to highest precedence)
--------------------------------------------------------------
program        → declaration* EOF
declaration    → classDecl | funcDecl | varDecl | statement
classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}"
funDecl        → "fun" function
function       → IDENTIFIER "(" parameters? ")" block
parameters     → IDENTIFIER ( "," IDENTIFIER )*
varDecl        → "var" IDENTIFIER ( "=" expression )? ";"
statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block
returnStmt     → "return" expression? ";"
exprStmt       → expression ";"?
forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement
ifStmt         → "if" "(" expression ")" statement ( "else" statement )?
printStmt      → "print" expression ";"?
whileStmt      → "while" "(" expression ")" statement
block          → "{" declaration* "}"
expression     → assignment
assignment     → ( call "." )? IDENTIFIER "=" assignment | logic_or
logic_or       → logic_and ( "or" logic_and )*
logic_and      → equality ( "and" equality )*
equality       → comparison ( ( "!=" | "==" ) comparison )*
comparison     → shift ( ( ">" | ">=" | "<" | "<=" ) shift )*
shift          → term ( ( ">>" | "<<" | ">>>" ) term )* --- TODO ---
term           → factor ( ( "-" | "+" ) factor )*
factor         → unary ( ( "/" | "*" ) unary )*
unary          → ( "!" | "-" ) unary | call
call           → primary ( "(" arguments? ")" | "." IDENTIFIER )*
arguments      → expression ( "," expression )*
primary        → "true" | "false" | "nil" | "this" | NUMBER | STRING | IDENTIFIER | "(" expression ")" | "super" "." IDENTIFIER
*/
use crate::ast::{Block, Expr, Function, Stmt};
use crate::token::{
    Literal, Token,
    TokenType::{self, *},
};

#[derive(Clone, Debug)]
struct ParserError {
    message: std::string::String,
    token: Option<Token>,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at {:?}", self.message, self.token)
    }
}

type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub errors: Vec<ParserError>,
    pub current: usize,
    expr_start: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            errors: Vec::new(),
            current: 0,
            expr_start: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.extend(self.declaration());
        }

        self.print_errors();
        statements
    }

    fn print_errors(&self) {
        for err in &self.errors {
            println!("{}", err);
        }
    }

    fn synchronize(&mut self) {
        while !self.matches(&[Semicolon]) && !self.is_at_end() {
            self.advance();
        }
    }

    fn declaration(&mut self) -> Option<Stmt> {
        match self.declaration_inner() {
            Ok(stmt) => Some(stmt),
            Err(e) => {
                self.errors.push(e);
                self.synchronize();
                None
            }
        }
    }

    fn declaration_inner(&mut self) -> ParseResult<Stmt> {
        if self.matches(&[Class]) {
            return self.class_declaration();
        }

        if self.matches(&[Fun]) {
            return self.function("function");
        }

        if self.matches(&[Var]) {
            return self.variable_declaration();
        }

        self.statement()
    }

    fn class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = match self.consume(Identifier, "Expected class name to be a valid identifier") {
            Ok(val) => val,
            Err(e) => return self.error(e.message),
        }
        .clone();

        let mut superclass: Option<Expr> = None;
        if self.matches(&[Less]) {
            match self.consume(Identifier, "Expected superclass name.") {
                Ok(_) => {}
                Err(e) => return self.error(e.message),
            };
            superclass = Some(Expr::Variable(self.previous().clone()))
        }

        match self.consume(LeftBrace, "Expect '{' before class body") {
            Ok(_) => {}
            Err(e) => return self.error(e.message),
        }

        let mut methods: Vec<Stmt> = vec![];
        while !self.matches(&[RightBrace]) {
            match self.function("method") {
                Ok(m) => methods.push(m),
                Err(e) => return self.error(e.message),
            };
        }

        Ok(Stmt::Class {
            name: name,
            superclass: superclass,
            methods,
        })
    }

    fn variable_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(Identifier, "Expected variable name")?.clone();

        let mut expr: Option<Expr> = None;
        if self.matches(&[Equal]) {
            expr = Some(self.expr()?);
        }

        self.consume(Semicolon, "Expected Semicolon at end of declaration")?;

        Ok(Stmt::Var {
            name: name,
            initializer: expr,
        })
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.matches(&[If]) {
            return self.if_statement();
        }

        if self.matches(&[For]) {
            return self.for_statement();
        }

        if self.matches(&[Print]) {
            let expr = self.expr()?;
            self.consume(Semicolon, "Expected semicolon at end of print statement")?;
            return Ok(Stmt::Print(expr));
        }

        if self.matches(&[Return]) {
            let expr = self.expr()?;
            self.consume(Semicolon, "Expected semicolon at end of return expression")?;
            return Ok(Stmt::Return {
                keyword: self.peek().clone(),
                value: expr,
            });
        }

        if self.matches(&[While]) {
            return self.while_statement();
        }

        if self.matches(&[LeftBrace]) {
            return self.block_statement();
        }

        self.expression_statement()
    }

    fn block_statement(&mut self) -> ParseResult<Stmt> {
        let mut statements = vec![];
        while !self.matches(&[RightBrace]) {
            statements.extend(self.declaration());
        }
        Ok(Stmt::Block(Block { statements }))
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(LeftParen, "Expected '(' after 'if'")?;

        let condition = self.expr()?;

        self.consume(RightParen, "Expect ')' after if condition")?;

        let then_branch = self.statement()?;

        let mut else_branch: Option<Stmt> = None;
        if self.matches(&[Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expr()?;
        self.consume(RightParen, "Expect ')' after 'while'")?;

        let statements = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(statements),
        })
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(LeftParen, "Expect '(' after 'for'")?;

        let initializer = {
            if self.matches(&[Var]) {
                Some(self.variable_declaration()?)
            } else if self.matches(&[Semicolon]) {
                self.advance();
                None
            } else {
                Some(self.expression_statement()?)
            }
        };

        let increment = {
            if self.matches(&[Semicolon]) {
                self.advance();
                None
            } else {
                Some(self.expression_statement()?)
            }
        };

        let condition = {
            if self.matches(&[Semicolon]) {
                Expr::Literal(Literal::Bool(true))
            } else {
                self.expr()?
            }
        };

        self.consume(LeftParen, "Expect ')' after 'for'")?;

        let statements = self.statement()?;

        let body = {
            if let Some(increment) = increment {
                Stmt::Block(Block {
                    statements: vec![statements, increment],
                })
            } else {
                statements
            }
        };

        let while_loop = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            Ok(Stmt::Block(Block {
                statements: vec![initializer, while_loop],
            }))
        } else {
            Ok(while_loop)
        }
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = match self.expr() {
            Ok(ex) => ex,
            Err(e) => return self.error(e.message),
        };

        self.consume(Semicolon, "Expect semicolon at end of expression")?;
        Ok(Stmt::Expression(expr))
    }

    fn function(&mut self, kind: &'static str) -> ParseResult<Stmt> {
        let name = match self.consume(Identifier, "") {
            Ok(n) => n,
            Err(_) => {
                return self.error(format!("Expect open parent after {} name", kind));
            }
        }
        .clone();

        if self.consume(LeftParen, "").is_err() {
            return self.error(format!("Expect open paren after {} name", kind));
        }

        let params = match self.parameters() {
            Ok(p) => p,
            Err(e) => return self.error(e.message),
        };

        if self.consume(LeftBrace, "").is_err() {
            return self.error(format!("Expected block after {} declaration", kind));
        }

        let block = match self.block() {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        Ok(Stmt::Function(Function {
            name: name,
            params: params,
            body: block,
        }))
    }

    fn parameters(&mut self) -> ParseResult<Vec<Token>> {
        let mut parameters: Vec<Token> = vec![];
        while !self.matches(&[RightParen]) {
            let param = self.consume(Identifier, "Expected identifier as function parameter");
            match param {
                Ok(p) => parameters.push(p.clone()),
                Err(e) => return self.error(e.message),
            }

            if self.matches(&[RightParen]) {
                break;
            }

            match self.consume(Comma, "Expected ',' after parameter") {
                Ok(_) => {}
                Err(e) => return self.error(e.message),
            }
        }

        Ok(parameters)
    }

    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = vec![];

        while !self.matches(&[RightBrace]) {
            let expr = self.declaration_inner();
            match expr {
                Ok(ex) => stmts.push(ex),
                Err(e) => {
                    self.errors.push(ParserError {
                        message: e.message,
                        token: e.token,
                    });
                    self.synchronize();
                }
            }
        }
        Ok(stmts)
    }

    fn expr(&mut self) -> ParseResult<Expr> {
        self.expr_start = self.current;
        match self.assignment() {
            Ok(assign) => Ok(assign),
            Err(e) => {
                let mut new_error = e.clone();
                new_error.message =
                    format!("{} + {}", e.message.clone(), self.gather_lexemes().clone());
                Err(new_error)
            }
        }
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.logic_or()?;

        if self.matches(&[Equal]) {
            let value = self.assignment()?;

            let uexpr = expr;
            match uexpr {
                Expr::Variable(name) => {
                    let name = name;
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                Expr::Get { object, name } => {
                    return Ok(Expr::Set {
                        object,
                        name,
                        value: Box::new(value),
                    });
                }
                _ => return self.error("Invalid assignment".to_string()),
            }
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> ParseResult<Expr> {
        self.binary(Self::logic_and, &[Or])
    }

    fn logic_and(&mut self) -> ParseResult<Expr> {
        self.binary(Self::equality, &[And])
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        self.binary(Self::comparison, &[BangEqual, EqualEqual])
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        self.binary(Self::term, &[Greater, GreaterEqual, Less, LessEqual])
    }

    fn term(&mut self) -> ParseResult<Expr> {
        self.binary(Self::factor, &[Minus, Plus])
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        self.binary(Self::unary, &[Slash, Star])
    }

    fn binary(
        &mut self,
        next: fn(&mut Self) -> ParseResult<Expr>,
        tok_types: &[TokenType],
    ) -> ParseResult<Expr> {
        let mut expr = next(self);
        while self.matches(tok_types) {
            let operator = self.previous().clone();
            let right = next(self)?;
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.call();
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary();

        loop {
            if self.matches(&[LeftParen]) {
                expr = self.finish_call(expr?);
            } else if self.matches(&[Dot]) {
                let name: Token = self
                    .consume(Identifier, "Expect property name after '.'.")?
                    .clone();
                expr = Ok(Expr::Get {
                    object: Box::new(expr?),
                    name: name,
                });
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        let mut arguments: Vec<Expr> = vec![];
        if !self.check(RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.errors.push(ParserError {
                        message: "Can't have more than 255 arguments".to_string(),
                        token: None,
                    });
                }

                arguments.push(self.expr()?);

                if !self.matches(&[Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(RightParen, "Expect ')' after arguments")?
            .clone();

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: paren,
            arguments,
        })
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.matches(&[False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.matches(&[True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.matches(&[Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.matches(&[Number, String]) {
            let literal = match self.previous().clone().literal {
                Some(l) => l,
                None => {
                    return self.error("Expected literal".to_string());
                }
            };
            return Ok(Expr::Literal(literal));
        }
        if self.matches(&[Super]) {
            let keyword = self.previous().clone();
            self.consume(Dot, "Expect '.' after 'super'.")?;
            let method = self
                .consume(Identifier, "Expect superclass method name.")?
                .clone();
            return Ok(Expr::Super {
                keyword: keyword,
                method: method,
            });
        }
        if self.matches(&[This]) {
            return Ok(Expr::This(self.previous().clone()));
        }
        if self.matches(&[Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.matches(&[LeftParen]) {
            let expr: Expr = self.expr()?;
            self.consume(RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        self.error("Expected expression".to_string())
    }

    // Utility Functions
    fn matches(&mut self, tok_type: &[TokenType]) -> bool {
        for tok in tok_type {
            if self.tokens[self.current].token_type == *tok {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, tok_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == tok_type;
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Checks the current token matches the specified type,
    /// If it does, returns a reference to that token & advances, else a ParserError
    fn consume(
        &mut self,
        tok_type: TokenType,
        message: &'static str,
    ) -> Result<&Token, ParserError> {
        if self.check(tok_type) {
            return Ok(self.advance());
        }

        self.error(message.to_string())
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == EOF;
    }

    fn error<T>(&self, message: std::string::String) -> ParseResult<T> {
        Err(ParserError {
            message: message,
            token: Some(self.tokens[self.current].clone()),
        })
    }

    fn gather_lexemes(&self) -> std::string::String {
        self.tokens[self.expr_start..self.current + 1]
            .iter()
            .map(|t| t.lexeme.as_str())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;
    use std::fs;
    use walkdir::WalkDir;

    fn parse(program: &str) -> Parser {
        let tokens = lexer::Lexer::new(program.to_string()).scan();
        Parser::new(tokens)
    }

    #[test]
    fn parses_var_decl() {
        let program = "var a = 1;";
        let mut parser = parse(program);
        parser.parse();
        assert!(parser.errors.len() == 0)
    }

    #[test]
    fn parser_parses_function() {
        let program = "fun some_function() { var a = 1; var b = 2; return a + b; }";
        let mut parser = parse(program);
        parser.parse();
        assert!(parser.errors.len() == 0)
    }

    #[test]
    fn parser_parses_class() {
        let program = "class A { method(arg) { print\"A.method(\" + arg + \")\"; }}";
        let mut parser = parse(program);
        parser.parse();
        assert!(parser.errors.len() == 0)
    }

    #[test]
    fn parser_parses_everything() {
        for entry in WalkDir::new("lox") {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_str().unwrap();
            if filename.len() <= 3
                || filename[filename.len() - 3..filename.len()] != "lox".to_string()
            {
                continue;
            }
            println!("Parsing {}", filename);
            let source = fs::read_to_string(entry.path()).unwrap();
            let result = parse(source.as_str());
            assert!(result.errors.len() == 0);
        }
    }
}
