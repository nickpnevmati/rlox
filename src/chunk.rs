use crate::value::{Value, ValueArray};
use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    OpConstant(u8),
    OpNegate,
    OpReturn,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Chunk {
    pub codes: Vec<OpCode>,
    pub lines: Vec<usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            codes: vec![],
            lines: vec![],
            constants: vec![],
        }
    }

    pub fn add_constant(&mut self, val: Value) -> usize {
        self.constants.push(val);
        self.constants.len()
    }

    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.codes.push(byte);
        self.lines.push(line as usize);
    }
}
