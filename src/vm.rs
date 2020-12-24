use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub struct VM {
    chunk: Chunk,
    ip: *const OpCode,
    cur_offset: usize,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        let chunk = Chunk::new();
        let ip = chunk.code.as_ptr();
        Self {
            chunk,
            ip,
            cur_offset: 0,
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
                    println!("{}", constant);
                }
                LongConstant(index) => {
                    let constant = self.get_constant(index as usize);
                    println!("{}", constant);
                }
                Return => return InterpretResult::Ok,
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

    fn get_constant(&self, index: usize) -> &Value {
        &self.chunk.constants[index]
    }
}

#[cfg(debug_assertions)]
mod debug_utils {
    use super::*;

    pub fn trace_execution(vm: &VM) {
        disassemble_instruction(&vm);
    }

    fn disassemble_instruction(vm: &VM) {
        vm.chunk.disassemble_instruction(vm.cur_offset)
    }
}
