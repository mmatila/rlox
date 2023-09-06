#[derive(Debug)]
enum OpCode {
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }
    fn write(&mut self, op_code: OpCode) {}
}
