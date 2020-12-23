use crate::value::Value;

pub struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<usize>,
    constants: Vec<Value>,
}

pub enum OpCode {
    Constant(u8),
    LongConstant(u32),
    Return,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, instruction: OpCode, line: usize) {
        self.code.push(instruction);
        self.lines.push(line);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let index = self.add_constant(value);
        if index < 256 {
            self.write(OpCode::Constant(index as u8), line);
        } else {
            self.write(OpCode::LongConstant(index as u32), line);
        }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for offset in 0..self.code.len() {
            self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) {
        use OpCode::*;
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = &self.code[offset];
        match instruction {
            Constant(_) | LongConstant(_) => self.constant_instruction(instruction),
            Return => Self::simple_instruction(instruction),
        }
    }

    fn simple_instruction(instruction: &OpCode) {
        println!("{}", instruction.name())
    }

    fn constant_instruction(&self, instruction: &OpCode) {
        let index = match instruction {
            OpCode::Constant(index) => (*index) as usize,
            OpCode::LongConstant(index) => (*index) as usize,
            _ => unreachable!(),
        };

        let value = &self.constants[index];
        println!("{:16} {:04} '{}'", instruction.name(), index, value);
    }
}

impl OpCode {
    fn name(&self) -> &'static str {
        use OpCode::*;
        match self {
            Constant(_) => "OP_CONSTANT",
            LongConstant(_) => "OP_CONSTANT_LONG",
            Return => "OP_RETURN",
        }
    }
}
