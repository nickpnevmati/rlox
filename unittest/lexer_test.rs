use std::fs;

use walkdir::WalkDir;

use super::*;

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
        if filename.len() <= 3 || filename[filename.len() - 3..filename.len()] != "lox".to_string() {
            continue;
        }
        println!("Lexing {}", filename);
        let source = fs::read_to_string(entry.path()).unwrap();
        let lexer = Lexer::new(source);
        assert!(!lexer.has_error);

    }
}
