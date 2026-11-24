#[warn(elided_lifetimes_in_paths)]
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
    println!("------------------------------\n{}\n-------------------------------", source_string);

    let source = source_string.as_str();
    let scanner = scanner::Scanner::new(&source);

    for token in scanner {
        println!("{}", token);
    }
}
