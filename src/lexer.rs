use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Type: {} - Lexeme: {} - Literal: {:?} - Line: {})",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}

pub struct Lexer {
    source: Vec<u8>,
    line: usize,
    tokens: Vec<Token>,
    cursor: usize,
    start: usize,
    has_error: bool,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.as_bytes().to_vec(),
            line: 1,
            tokens: Vec::new(),
            cursor: 0,
            start: 0,
            has_error: false,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        loop {
            self.scan_token();
            self.advance();
            if self.is_done() {
                break;
            }
        }
        self.add_token(TokenType::EOF, None);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let byte = self.source[self.cursor];
        self.start = self.cursor;
        match byte {
            b' ' => {}
            b'\t' => {}
            b'\r' => {}
            b'\n' => self.line += 1,

            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'/' => self.add_token(TokenType::Slash, None),
            b'*' => self.add_token(TokenType::Star, None),

            b'!' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            b'=' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            }
            b'<' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            b'>' => {
                if self.peek() == Some(b'=') {
                    self.advance();
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            b'"' => {
                self.string();
                return;
            }

            _ => {
                if self.is_numeric(byte) {
                    self.number();
                    return;
                }

                if self.is_alpha(byte) {
                    self.identifier();
                    return;
                }

                self.error("Unexpected character".to_string());
            }
        };
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.source.len() - 1
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        assert!(self.start <= self.cursor);
        let bytes = self.source[self.start..self.cursor].to_vec();
        let lexeme = String::from_utf8(bytes).unwrap();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn string(&mut self) {
        while let Some(c) = self.peek() {
            self.advance();
            if c == b'"' {
                break;
            }
            if self.is_done() {
                self.error("Unterminated String".to_string());
            }
        }

        let literal = Some(Literal::String(self.get_literal_string()));
        self.add_token(TokenType::String, literal);
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.cursor + 1).copied()
    }

    fn search_reserved(&mut self) -> bool {
        let literal = self.get_literal_string();

        match literal.as_str() {
            "and" => self.add_token(TokenType::And, None),
            "class" => self.add_token(TokenType::Class, None),
            "else" => self.add_token(TokenType::Else, None),
            "false" => self.add_token(TokenType::False, Some(Literal::Bool(false))),
            "fun" => self.add_token(TokenType::Fun, None),
            "for" => self.add_token(TokenType::For, None),
            "if" => self.add_token(TokenType::If, None),
            "nil" => self.add_token(TokenType::Nil, Some(Literal::Nil)),
            "or" => self.add_token(TokenType::Or, None),
            "print" => self.add_token(TokenType::Print, None),
            "return" => self.add_token(TokenType::Return, None),
            "super" => self.add_token(TokenType::Super, None),
            "this" => self.add_token(TokenType::This, None),
            "true" => self.add_token(TokenType::True, Some(Literal::Bool(true))),
            "var" => self.add_token(TokenType::Var, None),
            "while" => self.add_token(TokenType::While, None),
            _ => {
                return false;
            }
        };
        return true;
    }

    fn is_numeric(&self, byte: u8) -> bool {
        // UTF-8 codes for 0 through 9
        byte > 0x30 && byte < 0x39
    }

    fn number(&mut self) {
        let mut decimal = false;
        while let Some(c) = self.peek() {
            if !self.is_numeric(c) {
                break;
            }
            self.advance();

            if c == b'.' {
                decimal = true;
                self.advance();
                while let Some(c1) = self.peek() {
                    if !self.is_numeric(c1) {
                        break;
                    }
                    self.advance();
                }
            }

            if decimal {
                break;
            }
        }

        let str = String::from_utf8(self.source[self.start..self.cursor + 1].to_vec()).unwrap();

        let literal = Some(Literal::Number(str.parse().unwrap()));
        self.add_token(TokenType::Number, literal);
    }

    fn is_alpha(&mut self, byte: u8) -> bool {
        (byte >= 0x41 && byte <= 0x5A) || (byte >= 0x61 && byte <= 0x7A)
    }

    fn identifier(&mut self) {
        let mut valid_termination = true;

        while let Some(c) = self.peek() {
            if !(self.is_alpha(c) || self.is_numeric(c) || c == b'-' || c == b'_') {
                break;
            }
            valid_termination = true;
            if c == b'-' || c == b'_' {
                valid_termination = false;
            }
            self.advance();
        }

        if !valid_termination {
            self.error("Invalid identifier termination".to_string());
        }

        let reserved = self.search_reserved();
        if reserved {
            return;
        }

        self.add_token(TokenType::Identifier, None);
    }

    fn get_literal_string(&self) -> String {
        String::from_utf8(self.source[self.start..self.cursor].to_vec()).unwrap()
    }

    fn error(&mut self, message: String) {
        self.has_error = true;
        let where_is = self.get_literal_string();
        println!("[line {}] Error {}: {}", self.line, where_is, message);
    }
}

#[cfg(test)]
#[path = "../unittest/lexer_test.rs"]
mod tests {
    use super::*;
    use std::fs;
    use walkdir::WalkDir;

    fn lex(src: &str) -> Vec<Token> {
        Lexer::new(src.to_string()).scan()
    }

    fn types(src: &str) -> Vec<TokenType> {
        lex(src).into_iter().map(|t| t.token_type).collect()
    }

    #[test]
    fn lexer_parses_number() {
        let tok_types = types("1");
        assert!(matches!(tok_types[0], TokenType::Number));
    }

    #[test]
    fn lexer_parses_string() {
        let tokens = types("\"this is @ str1ng_!@#^*(\"");
        assert!(matches!(tokens[0], TokenType::String));
    }

    #[test]
    fn parses_identifier() {
        let types = types("this_is_an_identifier");
        assert!(matches!(types[0], TokenType::Identifier));
    }

    #[test]
    fn parses_single_tokens() {
        let special_tokens = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Slash,
            TokenType::Star,
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::Equal,
            TokenType::EqualEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::EOF,
        ];

        let types = types("() {} , . - + ; / * ! != = == > >= < <=");
        assert_eq!(special_tokens, types);
    }

    #[test]
    fn no_files_have_errors() {
        for entry in WalkDir::new("./test_cases") {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_str().unwrap();
            if filename.len() <= 3
                || filename[filename.len() - 3..filename.len()] != "lox".to_string()
            {
                continue;
            }
            println!("Lexing {}", filename);
            let source = fs::read_to_string(entry.path()).unwrap();
            let lexer = Lexer::new(source);
            assert!(!lexer.has_error);
        }
    }
}
