use crate::{expr::{*}, lexer::{ Literal, Token, TokenType }};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&self) {

    }

    fn expr(&self) {

    }

    fn equality(&self) {

    }

    fn comparison(&self) {

    }

    fn term(&self) {

    }

    fn factor(&self) {

    }

    fn unary(&self) {

    }

    fn primary(&mut self) -> Expr {
        if self.matches(&[TokenType::False]) {
            let literal = LiteralExpr { value: Literal::Bool(false) };
            return Expr::Literal(literal)
        }
        if self.matches(&[TokenType::True]) {
            return Expr::Literal(LiteralExpr { value: Literal::Bool(true) })
        }
        if self.matches(&[TokenType::Nil]) {
            return Expr::Literal(LiteralExpr { value: Literal::Nil })
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal(LiteralExpr { value: self.previous().literal.unwrap() })
        }

        if self.matches(&[TokenType::Super]) {
            let keyword = self.previous();
            self.consume(TokenType::Dot, "Expect '.' after 'super'.");
            let method = self.consume(TokenType::Identifier, "Expect superclass method name.");
            return Expr::Super(SuperExpr { keyword: keyword, method: method })
        }

        panic!("TODO");
    }

    // Utility Functions
    fn matches(&self, tok_type: &[TokenType]) -> bool {
        for tok in tok_type {
            if self.tokens[self.current].token_type == *tok {
                return true;
            }
        }
        return false;
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
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

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, tok_type: TokenType, message: &str) -> Token {
        if self.check(tok_type) {
            return self.advance();
        }

        panic!("{}", message);
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
}
