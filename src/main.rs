use std::fs;
use std::env;

mod lexer;
mod ast;
mod expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
    run(file_path);
}

fn run(source_file: &String) {
    let contents = fs::read_to_string(source_file).expect("Failed to read file");
    lexer::Lexer::new(contents).scan();
}
