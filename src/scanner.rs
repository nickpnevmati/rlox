use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    line: i32,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();
        if is_ditig(c) {
            return self.number();
        }

        if is_alpha(c) {
            return self.identifier();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),

            '!' => self.match_next('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.match_next('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.match_next('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.match_next('=', TokenType::GreaterEqual, TokenType::Greater),

            '"' => self.string(),

            _ => self.error_token("Unexpected character."),
        }
    }

    fn make_token(&mut self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated String");
        }

        // Consume terminating quote.
        self.advance();
        return self.make_token(TokenType::String);
    }

    fn number(&mut self) -> Token<'a> {
        while is_ditig(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_ditig(self.peek_next()) {
            self.advance();

            while is_ditig(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'a> {
        while is_alpha(self.peek()) || is_ditig(self.peek()) {
            self.advance();
        }

        let id_type = self.identifier_type();
        self.make_token(id_type)
    }

    fn identifier_type(&mut self) -> TokenType {
        match &self.source[self.start..self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.as_bytes()[self.current - 1] as char;
    }

    fn match_next(&mut self, next_char: char, if_true: TokenType, if_false: TokenType) -> Token<'a> {
        if self.is_at_end() || self.peek() != next_char {
            return self.make_token(if_false);
        }
        self.current += 1;
        self.make_token(if_true)
    }

    fn peek(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == self.current
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
                continue;
            }

            if c == '\n' {
                self.line += 1;
                self.advance();
                continue;
            }

            if c == '/' {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.advance();
                } else {
                    return;
                }
            }

            return;
        }
    }

    fn error_token(&mut self, message: &'static str) -> Token<'a> {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.scan_token();
        if t.token_type == TokenType::EOF {
            None
        } else {
            Some(t)
        }
    }
}

fn is_ditig(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}
