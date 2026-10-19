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
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
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

pub struct Lexer {
    source: Vec<u8>,
    line: usize,
    tokens: Vec<Token>,
    cursor: usize,
    start: usize,
    stop: usize
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.as_bytes().to_vec(),
            line: 1,
            tokens: Vec::new(),
            cursor: 0,
            start: 0,
            stop: 0,
        }
    }

    pub fn scan(&mut self) {
        while !self.is_done() {
            self.scan_token();
            self.advance();
        }
    }

    fn scan_token(&mut self) {
        let byte = self.source[self.cursor];
        match byte {
            b' ' => {},
            b'\t' => {},
            b'\r' => {},
            b'\n' => { self.line += 1 },

            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'[' => self.add_token(TokenType::LeftBrace),
            b']' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'/' => self.add_token(TokenType::Slash),
            b'*' => self.add_token(TokenType::Star),

            b'!' => {
                if self.peek() == b'=' {
                    self.advance();
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.peek() == b'=' {
                    self.advance();
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            b'<' => {

            }
            b'>' => {

            }
            b'"' => {
                self.seek(|char: u8| -> bool { char == b'"' }, TokenType::String);
            }

            _ => {

            }
        };
    }

    fn is_done(&self) -> bool {
        self.cursor == self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        // TODO lexeme and literal?
        self.tokens.push(Token::new(token_type, self.source[self.cursor].to_string(), None, self.line));
    }

    fn add_token_2(&mut self, token_type: TokenType) {
        // TODO fix the range thing - and the literal thing
        let bytes = self.source[self.start..self.cursor].to_vec();
        let lexeme = String::from_utf8(bytes).unwrap();
        // Ew
        self.tokens.push(Token::new(token_type, String::from(&lexeme), Some(lexeme), self.line));
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn seek(&mut self, termination_condition: fn(u8) -> bool, token_type: TokenType) {
        self.start = self.cursor;

        while termination_condition(self.peek()) {
            self.advance();
        }

        // TODO this is where we check for reserved words maybe?

        self.add_token_2(token_type);
    }

    fn peek(&self) -> u8 {
        if self.source.len() >= self.cursor {
            // TODO how do we handle this?
            // I assume we need an error - since we're searching for something that doesn't exist
            return b'a'
        }
        self.source[self.cursor + 1]
    }

    fn search_reserved(&self, identifier: String) -> Option<TokenType> {
        match identifier.as_str() {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None
        }
    }
}
