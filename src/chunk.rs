use crate::value::Value;

pub struct Chunk {
    pub code: Vec<OpCode>,
    lines: Vec<usize>,
    pub constants: Vec<Value>,
}

#[derive(Copy, Clone)]
pub enum OpCode {
    Constant(u8),
    LongConstant(u32),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,
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
        self.write_line(line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let index = self.add_constant(value);
        if index < 256 {
            self.write(OpCode::Constant(index as u8), line);
        } else {
            self.write(OpCode::LongConstant(index as u32), line);
        }
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn write_line(&mut self, line: usize) {
        let len = self.lines.len();
        if len == 0 || self.lines[len - 2] != line {
            self.lines.push(line);
            self.lines.push(1);
        } else {
            self.lines[len - 1] += 1;
        }
    }

    fn get_line(&self, offset: usize) -> usize {
        assert!(self.lines.len() % 2 == 0);

        let mut rem = offset + 1;
        let mut iter = self.lines.chunks_exact(2);
        let mut current_line;
        loop {
            match iter.next().unwrap() {
                [line, count] => {
                    current_line = *line;
                    if rem <= *count {
                        break;
                    } else {
                        rem -= *count;
                    }
                }
                _ => unreachable!(),
            }
        }

        current_line
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        for offset in 0..self.code.len() {
            self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) {
        use OpCode::*;
        print!("{:04} ", offset);

        let line = self.get_line(offset);
        if offset > 0 && line == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }

        let instruction = &self.code[offset];
        match instruction {
            Constant(_) | LongConstant(_) => self.constant_instruction(instruction),
            Add | Subtract | Multiply | Divide | Modulo | Negate | Return => {
                Self::simple_instruction(instruction)
            }
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
            Add => "OP_ADD",
            Subtract => "OP_SUBTRACT",
            Multiply => "OP_MULTIPLY",
            Divide => "OP_DIVIDE",
            Modulo => "OP_MODULO",
            Negate => "OP_NEGATE",
            Return => "OP_RETURN",
        }
    }
}
