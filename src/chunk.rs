use crate::{dissasembler::Dissasembler, opcode::Opcode, value::Value};
use std::fmt::Debug;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<(usize, usize)>,
    pub name: String,
}

impl Chunk {
    pub fn new(name: String) -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            name,
        }
    }

    pub fn write<O: Into<Vec<u8>>>(&mut self, value: O, line: usize) {
        self.code.extend_from_slice(&value.into());
        self.add_line(line);
    }

    // NOTE: In the book it asks to support 24bit constants, but why not 32bit :^)
    pub fn write_constant(&mut self, constant: u32, line: usize) {
        if constant < 256 {
            self.write(Opcode::Constant, line);
            self.write([constant as u8], line);
        } else {
            self.write(Opcode::ConstantLong, line);
            self.write(constant.to_be_bytes(), line);
        }
    }

    fn add_line(&mut self, line: usize) {
        if self.lines.is_empty() {
            self.lines.push((line, self.code.len() - 1));
        } else {
            let last_line = self.lines.last().unwrap().0;
            if last_line == line {
                return;
            }
            self.lines.push((line, self.code.len() - 1));
        }
    }

    pub fn line_for_instruction_n(&self, n: usize) -> usize {
        let mut line = 0;
        for (l, i) in &self.lines {
            if n <= *i {
                return line;
            }
            line = *l;
        }
        line
    }

    pub fn add_constant(&mut self, value: Value) -> u32 {
        self.constants.push(value);
        (self.constants.len() - 1) as u32
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Dissasembler::disassemble(self, f)?;
        Ok(())
    }
}
