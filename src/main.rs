use std::env;
use std::fs;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(p) => p,
        None => panic!("Expected file argument"),
    };

    let source_string = fs::read_to_string(file_path).unwrap();
    println!("{}", source_string);

    let source = source_string.into_bytes();
    let mut scanner = scanner::Scanner::new(&source);

    loop {
        let token = scanner.scan_token();
        println!("{}", token);

        if token.token_type == scanner::TokenType::EOF {
            break;
        }
    }
}
