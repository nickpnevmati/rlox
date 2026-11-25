use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.codes.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:0>5}", offset);

    match chunk
        .codes
        .get(offset)
        .expect("Out of bounds access to OpReturn")
    {
        OpCode::OpConstant(_) => todo!(),
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpNegate => todo!(),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk
        .codes
        .get(offset + 1)
        .expect("Out of bounds access to OpConstant");
    print!("{} {}", name, constant);
    println!();
    offset + 2
}
