use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: Chunk, name: &str) {
    println!("== {name} ==");

    for (offset, instruction) in chunk.code.iter().enumerate() {
        disassemble_instruction(instruction, offset);
    }
}

fn disassemble_instruction(instruction: &OpCode, offset: usize) {
    print!("{:0>4} ", offset);

    match instruction {
        OpCode::Return => simple_instruction("RETURN"),
        _ => println!("Unknown opcode {:?}", instruction),
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}
