mod chunk;
mod value;

use chunk::{Chunk, OpCode};
use value::Value;

pub fn run() {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value(3.5), 123);
    chunk.write(OpCode::Return, 123);
    chunk.disassemble("test chunk");
}
