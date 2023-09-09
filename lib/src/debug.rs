use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    for (offset, _) in chunk.code.iter().enumerate() {
        disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:0>4} ", offset);
    if offset > 0 && chunk.lines.get(offset) == chunk.lines.get(offset - 1) {
        print!("   | ")
    } else {
        print!("{:4} ", chunk.lines.get(offset).unwrap());
    }

    let instruction = chunk.code.get(offset).unwrap();
    match instruction {
        OpCode::Return => simple_instruction("RETURN"),
        OpCode::Constant(index) => constant_instruction("CONSTANT", chunk, offset, index),
        _ => println!("Unknown opcode {:?}", instruction),
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize, constant_index: &usize) {
    print!("{:<16} {:4?} '", name, constant_index);
    print_value(chunk.constants.get(offset).unwrap());
    println!("'");
}

fn print_value(value: &Value) {
    print!("{}", value);
}
