use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::vm::InterpretResult::*;

macro_rules! binary_op {
    ($vm:expr, $value_type:expr, $op:tt) => {{
        if !matches!($vm.stack_peek(0), Value::Number(_))
            || !matches!($vm.stack_peek(1), Value::Number(_))
        {
            return InterpretRuntimeError;
        }

        let Value::Number(b) = $vm.stack_pop() else {
            unreachable!()
        };
        let Value::Number(a) = $vm.stack_pop() else {
            unreachable!()
        };

        $vm.stack_push($value_type(a $op b));
    }};
}

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
                    self.stack_push(constant);
                    println!("{}", constant);
                }
                OpCode::OpReturn => {
                    println!("{}", self.stack_pop());
                    return InterpretOk;
                }
                OpCode::OpNegate => {
                    let stack_top = self.stack_peek(0);
                    match *stack_top {
                        Value::Number(n) => {
                            self.stack_push(Value::Number(-n));
                        }
                        _ => {
                            return InterpretRuntimeError; // TODO message?
                        }
                    }
                }
                OpCode::OpAdd => binary_op!(self, Value::Number, +),
                OpCode::OpSub => binary_op!(self, Value::Number, -),
                OpCode::OpMul => binary_op!(self, Value::Number, *),
                OpCode::OpDiv => binary_op!(self, Value::Number, /),
                OpCode::OpNil => self.stack_push(Value::Nil),
                OpCode::OpTrue => self.stack_push(Value::Bool(true)),
                OpCode::OpFalse => self.stack_push(Value::Bool(false)),
                OpCode::OpNot => {
                    let value = self.stack_pop();
                    self.stack_push(Value::Bool(self.is_falsey(value)));
                }
                OpCode::OpEqual => {
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(Value::Bool(values_equal(a, b)));
                }
                OpCode::OpGreater => binary_op!(self, Value::Bool, >),
                OpCode::OpLess => binary_op!(self, Value::Bool, <),
            }
        }
    }

    fn read_byte(&mut self) -> OpCode {
        self.ip += 1;
        self.chunk.codes[self.ip - 1]
    }

    fn stack_push(&mut self, element: Value) {
        self.stack.insert(0, element)
    }

    fn stack_pop(&mut self) -> Value {
        self.stack.remove(0)
    }

    fn stack_peek(&mut self, distance: usize) -> &Value {
        &self.stack.get(distance).unwrap() // TODO unsafeeee
    }

    fn is_falsey(&self, value: Value) -> bool {
        match value {
            Value::Nil => true,
            Value::Bool(b) => b == false,
            _ => false,
        }
    }
}

fn values_equal(a: Value, b: Value) -> bool {
    if !matches!(a, b) {
        return false;
    }

    match a {
        Value::Bool(v) => v == b.as_bool(),
        Value::Number(v) => v == b.as_number(),
        Value::Nil => true,
        _ => unreachable!(),
    }
}
