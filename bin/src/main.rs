use lib::chunk::{Chunk, OpCode};
use lib::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);

    disassemble_chunk(chunk, "test chunk");
}
