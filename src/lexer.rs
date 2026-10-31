use std::fmt::Display;

use crate::lexer::TokenType::*;

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
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(std::string::String),
    Number(f64),
    Bool(bool),
    Nil,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: std::string::String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: std::string::String,
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
    pub fn new(source: std::string::String) -> Lexer {
        Lexer {
            source: source.as_bytes().to_vec(),
            line: 1,
            tokens: Vec::new(),
            cursor: 1,
            start: 0,
            has_error: false,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        while !self.is_done() {
            self.scan_token();
            self.start = self.cursor;
            self.cursor += 1;
        }
        self.tokens.push(Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let byte = self.source[self.start];
        match byte {
            b' ' => {}
            b'\t' => {}
            b'\r' => {}
            b'\n' => self.line += 1,

            b'(' => self.add_token(LeftParen, None),
            b')' => self.add_token(RightParen, None),
            b'{' => self.add_token(LeftBrace, None),
            b'}' => self.add_token(RightBrace, None),
            b',' => self.add_token(Comma, None),
            b'.' => self.add_token(Dot, None),
            b'-' => self.add_token(Minus, None),
            b'+' => self.add_token(Plus, None),
            b';' => self.add_token(Semicolon, None),
            b'/' => self.add_token(Slash, None),
            b'*' => self.add_token(Star, None),

            b'!' => self.next_is_equal(BangEqual, Bang),
            b'=' => self.next_is_equal(EqualEqual, Equal),
            b'<' => self.next_is_equal(LessEqual, Less),
            b'>' => self.next_is_equal(GreaterEqual, Greater),
            b'"' => self.string(),
            _ => {
                if is_numeric(byte) {
                    self.number();
                } else if is_alpha(byte) {
                    self.identifier();
                } else {
                    self.error("Unexpected character".to_string());
                }
            }
        };
    }

    fn next_is_equal(&mut self, type1: TokenType, type2: TokenType) {
        if self.peek() == Some(b'=') {
            self.advance();
            self.add_token(type1, None);
        } else {
            self.add_token(type2, None);
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = self.get_literal_string();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn string(&mut self) {
        while self.peek().is_some_and(|c| c != b'"') {
            self.advance();
        }
        self.advance();

        let literal = Some(Literal::String(self.get_literal_string()));
        self.add_token(String, literal);
    }

    fn is_done(&self) -> bool {
        self.cursor > self.source.len()
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.cursor).copied()
    }

    fn search_reserved(&mut self) -> bool {
        let literal = self.get_literal_string();

        println!("Literal:{literal}:");

        match literal.as_str() {
            "and" => self.add_token(And, None),
            "class" => self.add_token(Class, None),
            "else" => self.add_token(Else, None),
            "false" => self.add_token(False, Some(Literal::Bool(false))),
            "fun" => self.add_token(Fun, None),
            "for" => self.add_token(For, None),
            "if" => self.add_token(If, None),
            "nil" => self.add_token(Nil, Some(Literal::Nil)),
            "or" => self.add_token(Or, None),
            "print" => self.add_token(Print, None),
            "return" => self.add_token(Return, None),
            "super" => self.add_token(Super, None),
            "this" => self.add_token(This, None),
            "true" => self.add_token(True, Some(Literal::Bool(true))),
            "var" => self.add_token(Var, None),
            "while" => self.add_token(While, None),
            _ => {
                return false;
            }
        };
        return true;
    }

    fn number(&mut self) {
        while self.peek().is_some_and(|n| is_numeric(n)) {
            self.advance();
        }

        let str = self.get_literal_string();
        println!("{}", str);

        let literal = Some(Literal::Number(str.parse().unwrap()));
        self.add_token(Number, literal);
    }

    fn identifier(&mut self) {
        let mut valid_termination = true;

        let is_valid_identifier =
            |x| -> bool { is_alpha(x) || is_numeric(x) || x == b'-' || x == b'_' };

        while let Some(c) = self.peek()
            && is_valid_identifier(c)
        {
            self.advance();
            valid_termination = c != b'-' && c != b'_';
        }

        if !valid_termination {
            self.error("Invalid identifier termination".to_string());
        }

        if !self.search_reserved() {
            self.add_token(Identifier, None);
        }
    }

    fn get_literal_string(&self) -> std::string::String {
        std::string::String::from_utf8(self.source[self.start..self.cursor].to_vec()).unwrap()
    }

    fn error(&mut self, message: std::string::String) {
        self.has_error = true;
        let where_is = self.get_literal_string();
        println!("[line {}] Error {}: {}", self.line, where_is, message);
    }
}

fn is_alpha(byte: u8) -> bool {
    // ASCII codes for a-z or A-Z
    (byte >= 0x41 && byte <= 0x5A) || (byte >= 0x61 && byte <= 0x7A)
}

fn is_numeric(byte: u8) -> bool {
    // ASCII codes for 0 through 9
    byte > 0x30 && byte < 0x39
}

#[cfg(test)]
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
    fn lexer_parses_symbol() {
        let tokens = types(">");
        assert!(matches!(tokens[0], Greater));
    }

    #[test]
    fn lexer_parses_number() {
        let tok_types = types("1");
        let type0 = &tok_types[0];
        println!("{type0}");
        assert!(matches!(tok_types[0], Number));
    }

    #[test]
    fn lexer_lexeme_is_correct() {
        let tokens = lex("123");
        let number = &tokens[0];
        println!("{}", &number.lexeme);
        assert!(number.lexeme == "123".to_string())
    }

    #[test]
    fn lexer_parses_string() {
        let tokens = types("\"this is @ str1ng_!@#^*(\"");
        assert!(matches!(tokens[0], String));
    }

    #[test]
    fn lexer_parses_identifier() {
        let types = types("this_is_an_identifier");
        assert!(matches!(types[0], Identifier));
    }

    #[test]
    fn lexer_parses_var_decl() {
        let line = lex("var a = 123;");
        for tok in &line {
            println!("{}", tok);
        }
        let expected: Vec<TokenType> = vec![Var, Identifier, Equal, Number, Semicolon];
        let mut zipped = line.iter().zip(expected);
        assert!(zipped.all(|x| x.0.token_type == x.1));
    }

    #[test]
    fn lexer_parses_single_tokens() {
        let special_tokens = vec![
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
            Bang,
            BangEqual,
            Equal,
            EqualEqual,
            Greater,
            GreaterEqual,
            Less,
            LessEqual,
            EOF,
        ];

        let types = types("() {} , . - + ; / * ! != = == > >= < <=");
        assert_eq!(special_tokens, types);
    }

    #[test]
    fn lexer_parses_function() {
        let expected_tokens = vec![
            Fun,
            Identifier,
            LeftParen,
            RightParen,
            LeftBrace,
            Var,
            Identifier,
            Equal,
            Number,
            Semicolon,
            RightBrace,
            EOF
        ];

        let program = "fun some_function() { var a = 1; }";
        let tokens = types(program);
        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn lexer_no_files_have_errors() {
        for entry in WalkDir::new("lox") {
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
