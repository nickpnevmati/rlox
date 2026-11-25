#![feature(variant_count)]
#[warn(elided_lifetimes_in_paths)]
use std::env;
use std::fs;

use crate::vm::VM;

mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(p) => p,
        None => panic!("Expected file argument"),
    };

    let source_string = fs::read_to_string(file_path).unwrap();

    #[cfg(feature = "debug_trace_execution")]
    {
        println!(
            "------------------------------\n{}\n-------------------------------",
            source_string
        );
    }

    let source = source_string.as_str();
    let chunk = match compiler::compile(source) {
        Ok(chunk) => chunk,
        Err(_) => std::process::exit(65),
    };
    let mut vm = VM::new(&chunk);
    vm.run();
}
