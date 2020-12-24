use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

const INITIAL_STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: *const OpCode,
    cur_offset: usize,
    // TODO: Improve according to the 3rd challenge of the 15th chapter
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Fail)]
pub struct RuntimeError;

type InterpretResult = Result<(), RuntimeError>;

macro_rules! arithmetic_op {
    ($self:ident, $op:tt) => {
        {
        let b = $self.pop();
        let a = $self.pop();
        $self.push(a $op b);
        }
    }
}

impl VM {
    pub fn new() -> Self {
        let chunk = Chunk::new();
        let ip = chunk.code.as_ptr();
        let stack = Vec::with_capacity(INITIAL_STACK_SIZE);

        Self {
            chunk,
            ip,
            cur_offset: 0,
            stack,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.ip = chunk.code.as_ptr();
        self.chunk = chunk;
        self.cur_offset = 0;

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        use OpCode::*;
        loop {
            #[cfg(debug_assertions)]
            debug_utils::trace_execution(self);

            match self.next_ins() {
                Constant(index) => {
                    let constant = self.get_constant(index as usize);
                    self.push(constant);
                }
                LongConstant(index) => {
                    let constant = self.get_constant(index as usize);
                    self.push(constant);
                }
                Add => arithmetic_op!(self, +),
                Subtract => arithmetic_op!(self, -),
                Multiply => arithmetic_op!(self, *),
                Divide => arithmetic_op!(self, /),
                Modulo => arithmetic_op!(self, %),
                Negate => {
                    let value = -self.pop();
                    self.push(value);
                }
                Return => {
                    println!("{}", self.pop());
                    return Ok(());
                }
            }
        }
    }

    fn next_ins(&mut self) -> OpCode {
        unsafe {
            let instruction = *self.ip;
            self.ip = self.ip.add(1);
            self.cur_offset += 1;
            instruction
        }
    }

    fn get_constant(&self, index: usize) -> Value {
        self.chunk.constants[index].clone()
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

#[cfg(debug_assertions)]
mod debug_utils {
    use super::*;

    pub fn trace_execution(vm: &VM) {
        print_stack(&vm);
        disassemble_instruction(&vm);
    }

    fn print_stack(vm: &VM) {
        print!("          ");
        for value in &vm.stack {
            print!("[{}]", value);
        }

        println!()
    }

    fn disassemble_instruction(vm: &VM) {
        vm.chunk.disassemble_instruction(vm.cur_offset)
    }
}
