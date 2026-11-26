use std::any::Any;

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

#[derive(Debug)]
pub struct CompileError;

#[rustfmt::skip]
pub fn compile(source: &str) -> Result<Chunk, CompileError> {
    let mut compiler = Compiler::new(source);
    compiler.parser.advance();
    compiler.expression();
    compiler.parser.consume(TokenType::EOF, "Expect end of expression.");
    compiler.finish()
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

impl<'p> Parser<'p> {
    fn new(source: &'p str) -> Parser<'p> {
        Parser {
            scanner: Scanner::new(source),
            current: Token::placeholder(),
            previous: Token::placeholder(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.lexeme);
        }
    }

    fn consume(&mut self, tok_type: TokenType, message: &str) {
        if self.current.token_type == tok_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        let current = self.current;
        self.error_at(&current, message);
    }

    fn error(&mut self, message: &str) {
        let previous = self.previous;
        self.error_at(&previous, message)
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::EOF => eprint!(" at end."),
            TokenType::Error => {}
            _ => eprint!(" at {}", token.lexeme),
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
}

struct Compiler<'c> {
    parser: Parser<'c>,
    chunk: Chunk,
}

impl<'c> Compiler<'c> {
    fn new(source: &'c str) -> Compiler<'c> {
        Compiler {
            parser: Parser::new(source),
            chunk: Chunk::new(),
        }
    }

    fn finish(self) -> Result<Chunk, CompileError> {
        if self.parser.had_error {
            Err(CompileError)
        } else {
            Ok(self.chunk)
        }
    }

    fn emit_byte(&mut self, byte: OpCode) {
        let line = self.parser.previous.line;
        let chunk = self.current_chunk();
        chunk.write(byte, line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn emit_constant(&mut self, value: Value) -> Result<OpCode, CompileError> {
        let index = self.current_chunk().add_constant(value);
        let index_u8 = u8::try_from(index).map_err(|_| CompileError)?;
        return Ok(OpCode::OpConstant(index_u8));
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn number(&mut self) {
        let value: f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        self.expression();

        self.parse_precedence(Precedence::PrecUnary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            _ => todo!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSub),
            TokenType::Star => self.emit_byte(OpCode::OpMul),
            TokenType::Slash => self.emit_byte(OpCode::OpDiv),

            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual, OpCode::OpNot),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess, OpCode::OpNot),
            TokenType::Less => self.emit_byte(OpCode::OpLess),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater, OpCode::OpNot),

            _ => unreachable!(),
        }
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence);
    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            _ => unreachable!(),
        }
    }

    fn parse_precedence(&mut self, prec: Precedence) -> Result<(), CompileError> {
        self.parser.advance();

        let prefix_rule = self
            .get_rule(self.parser.previous.token_type)
            .prefix
            .ok_or(CompileError)?;
        prefix_rule(self);

        loop {
            let previous_type = self.parser.previous.token_type;
            if prec as i8 <= self.get_rule(previous_type).precedence as i8 {
                break;
            }

            self.parser.advance();
            let infix_rule = self
                .get_rule(self.parser.previous.token_type)
                .infix
                .ok_or(CompileError)?;
            infix_rule(self);
        }

        Ok(())
    }

    #[rustfmt::skip]
    fn get_rule(&self, op_type: TokenType) -> ParseRule<'c> {
        match op_type {
            TokenType::LeftParen    => ParseRule{prefix: Some(Self::grouping), infix: None,               precedence: Precedence::PrecNone},       // LeftParen
            TokenType::RightParen   => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // RightParen
            TokenType::LeftBrace    => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // LeftBrace
            TokenType::RightBrace   => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // RightBrace
            TokenType::Comma        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Comma
            TokenType::Dot          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Dot
            TokenType::Minus        => ParseRule{prefix: Some(Self::unary),    infix: Some(Self::binary), precedence: Precedence::PrecTerm},       // Minus
            TokenType::Plus         => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecTerm},       // Plus
            TokenType::Semicolon    => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Semicolon
            TokenType::Slash        => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecFactor},     // Slash
            TokenType::Star         => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecFactor},     // Star
            TokenType::Bang         => ParseRule{prefix: Some(Self::unary),    infix: None,               precedence: Precedence::PrecNone},       // Bang
            TokenType::BangEqual    => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecEquality}, // BangEqual
            TokenType::Equal        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Equal
            TokenType::EqualEqual   => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecEquality}, // EqualEqual
            TokenType::Greater      => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecComparison}, // Greater
            TokenType::GreaterEqual => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecComparison}, // GreaterEqual
            TokenType::Less         => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecComparison}, // Less
            TokenType::LessEqual    => ParseRule{prefix: None,                 infix: Some(Self::binary), precedence: Precedence::PrecComparison}, // LessEqual
            TokenType::Identifier   => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Identifier
            TokenType::String       => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // String
            TokenType::Number       => ParseRule{prefix: Some(Self::number),   infix: None,               precedence: Precedence::PrecNone},       // Number
            TokenType::And          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // And
            TokenType::Class        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Class
            TokenType::Else         => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Else
            TokenType::False        => ParseRule{prefix: Some(Self::literal),  infix: None,               precedence: Precedence::PrecNone},       // False
            TokenType::For          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // For
            TokenType::Fun          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Fun
            TokenType::If           => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // If
            TokenType::Nil          => ParseRule{prefix: Some(Self::literal),  infix: None,               precedence: Precedence::PrecNone},       // Nil
            TokenType::Or           => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Or
            TokenType::Print        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Print
            TokenType::Return       => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Return
            TokenType::Super        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Super
            TokenType::This         => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // This
            TokenType::True         => ParseRule{prefix: Some(Self::literal),  infix: None,               precedence: Precedence::PrecNone},       // True
            TokenType::Var          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Var
            TokenType::While        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // While
            TokenType::Error        => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // Error
            TokenType::EOF          => ParseRule{prefix: None,                 infix: None,               precedence: Precedence::PrecNone},       // EOF
        }
    }
}

#[repr(i8)]
#[derive(Copy, Clone)]
enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,
}

struct ParseRule<'c> {
    prefix: ParseFn<'c>,
    infix: ParseFn<'c>,
    precedence: Precedence,
}

type ParseFn<'c> = Option<fn(&mut Compiler<'c>)>;
