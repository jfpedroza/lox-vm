mod chunk;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use vm::VM;

pub fn run() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    chunk.write_constant(1.2, 123);
    chunk.write_constant(3.4, 123);

    chunk.write(OpCode::Add, 123);

    chunk.write_constant(5.6, 123);

    chunk.write(OpCode::Divide, 123);
    chunk.write(OpCode::Negate, 123);

    chunk.write(OpCode::Return, 123);

    chunk.disassemble("test chunk");
    println!();

    vm.interpret(chunk);
}
