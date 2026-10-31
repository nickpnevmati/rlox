use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_cmd(),
        2 => {
            let file_path = &args[0];
            run(file_path);
        }
        _ => panic!("Invalid arguments"),
    }
    let file_path = &args[0];
    run(file_path);
}

fn run_cmd() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let tokens = lexer::Lexer::new(line.unwrap()).scan();
        for tok in tokens {
            print!("{} ", tok);
        }
        println!();
    }
}

fn run(source_file: &String) {
    let contents = fs::read_to_string(source_file).expect("Failed to read file");
    let tokens = lexer::Lexer::new(contents).scan();
    let stmts = parser::Parser::new(tokens.clone()).parse();
    for tok in &tokens {
        println!("{}", tok);
    }
}
