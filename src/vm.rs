use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM<'vm> {
    chunk: &'vm Chunk,
    stack: Vec<Value>,
    ip: usize,
}

impl<'vm> VM<'vm> {
    pub fn new(chunk: &'vm Chunk) -> VM {
        VM {
            chunk: chunk,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: &'vm Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for v in &self.stack {
                    print!("[ {} ]", v);
                }
                println!();
                crate::debug::dissasemble_instruction(self.chunk, self.ip);
            }
            match self.read_byte() {
                OpCode::OpConstant(idx_u8) => {
                    // TODO this is unsafe
                    let constant: Value = self.chunk.constants[idx_u8 as usize];
                    self.stack.insert(0, constant);
                    println!("{}", constant);
                }
                OpCode::OpReturn => {
                    println!("{}", self.stack.remove(0));
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpNegate => todo!(),
            }
        }
    }

    fn read_byte(&mut self) -> OpCode {
        self.ip += 1;
        self.chunk.codes[self.ip - 1]
    }
}
