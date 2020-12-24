mod chunk;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use value::Value;
use vm::VM;

pub fn run() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    chunk.write_constant(Value(3.5), 123);
    chunk.write(OpCode::Return, 123);
    chunk.disassemble("test chunk");
    println!();

    vm.interpret(chunk);
}
