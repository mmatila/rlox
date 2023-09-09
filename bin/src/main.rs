use lib::chunk::{Chunk, OpCode};
use lib::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant(constant), 123);
    chunk.write(OpCode::Return, 123);

    disassemble_chunk(&chunk, "test chunk");
}
