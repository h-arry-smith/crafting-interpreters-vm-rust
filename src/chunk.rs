use crate::{opcode::Opcode, value::Value};
use std::fmt::Debug;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
    name: String,
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

    pub fn write<O: Into<Vec<u8>> + Copy>(&mut self, value: O, line: usize) {
        self.code.extend_from_slice(&value.into());

        for _ in 0..value.into().len() {
            self.lines.push(line);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble_instruction(
        &self,
        offset: &mut usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:04} ", offset)?;

        if *offset > 0 && self.lines[*offset] == self.lines[*offset - 1] {
            write!(f, "   | ")?;
        } else {
            write!(f, "{:4} ", self.lines[*offset])?;
        }

        let opcode = self.code[*offset];
        *offset += 1;

        match opcode.into() {
            Opcode::Return => {
                self.disassemble_simple_instruction(offset, "OP_RETURN", f)?;
            }
            Opcode::Constant => {
                self.dissassemble_constant_instruction(offset, "OP_CONSTANT", f)?;
            }
        }

        Ok(())
    }

    fn disassemble_simple_instruction(
        &self,
        offset: &mut usize,
        name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        *offset += 1;
        writeln!(f, "{}", name)
    }

    fn dissassemble_constant_instruction(
        &self,
        offset: &mut usize,
        name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let constant = self.code[*offset];
        *offset += 1;
        writeln!(
            f,
            "{:<16} {:4} '{}'",
            name, constant, self.constants[constant as usize]
        )
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== {} ==", self.name)?;
        let mut offset = 0;
        while offset < self.code.len() {
            self.disassemble_instruction(&mut offset, f)?;
        }
        Ok(())
    }
}
