#[derive(Debug)]
pub enum OpCode {
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }
    pub fn write(&mut self, op_code: OpCode) {
        self.code.push(op_code);
    }
}
