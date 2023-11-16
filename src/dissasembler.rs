use std::fmt::Write;

use crate::chunk::Chunk;
use crate::Opcode;

pub struct Dissasembler {}

impl Dissasembler {
    pub fn disassemble<W: Write>(chunk: &Chunk, f: &mut W) -> std::fmt::Result {
        println!("== {} ==", chunk.name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            Self::disassemble_instruction(chunk, &mut offset, f)?;
        }

        Ok(())
    }

    pub fn disassemble_instruction<W: Write>(
        chunk: &Chunk,
        offset: &mut usize,
        f: &mut W,
    ) -> std::fmt::Result {
        write!(f, "{:04} ", offset)?;

        Self::write_line_info(chunk, *offset, f)?;

        let opcode = chunk.code[*offset];
        *offset += 1;

        match opcode.into() {
            Opcode::Return => {
                Self::disassemble_simple_instruction("Return", f)?;
            }
            Opcode::Constant => {
                Self::dissassemble_constant_instruction(chunk, offset, "Constant", f)?;
            }
            Opcode::ConstantLong => {
                Self::dissassemble_constant_long_instruction(chunk, offset, "ConstantLong", f)?;
            }
            Opcode::Negate => {
                Self::disassemble_simple_instruction("Negate", f)?;
            }
            Opcode::Add => {
                Self::disassemble_simple_instruction("Add", f)?;
            }
            Opcode::Subtract => {
                Self::disassemble_simple_instruction("Subtract", f)?;
            }
            Opcode::Divide => {
                Self::disassemble_simple_instruction("Divide", f)?;
            }
            Opcode::Multiply => {
                Self::disassemble_simple_instruction("Multiply", f)?;
            }
            Opcode::Nil => {
                Self::disassemble_simple_instruction("Nil", f)?;
            }
            Opcode::True => {
                Self::disassemble_simple_instruction("True", f)?;
            }
            Opcode::False => {
                Self::disassemble_simple_instruction("False", f)?;
            }
            Opcode::Not => {
                Self::disassemble_simple_instruction("Not", f)?;
            }
            Opcode::Equal => {
                Self::disassemble_simple_instruction("Equal", f)?;
            }
            Opcode::Greater => {
                Self::disassemble_simple_instruction("Greater", f)?;
            }
            Opcode::Less => {
                Self::disassemble_simple_instruction("Less", f)?;
            }
        }

        Ok(())
    }

    pub fn trace_instruction(chunk: &Chunk, offset: &mut usize) {
        let mut output = String::new();
        Self::disassemble_instruction(chunk, offset, &mut output)
            .expect("Could not trace instruction");
        print!("{}", output);
    }

    fn write_line_info<W: Write>(chunk: &Chunk, offset: usize, f: &mut W) -> std::fmt::Result {
        if offset == 0 {
            let first_line = chunk.lines.first().unwrap().0;
            write!(f, "{:4} ", first_line)?;
        } else if let Some((line, _start)) = chunk.lines.iter().find(|(_, start)| *start == offset)
        {
            write!(f, "{:4} ", line)?;
        } else {
            write!(f, "   | ")?;
        }
        Ok(())
    }

    fn disassemble_simple_instruction<W: Write>(name: &str, f: &mut W) -> std::fmt::Result {
        writeln!(f, "{}", name)
    }

    fn dissassemble_constant_instruction<W: Write>(
        chunk: &Chunk,
        offset: &mut usize,
        name: &str,
        f: &mut W,
    ) -> std::fmt::Result {
        let constant = chunk.code[*offset];
        *offset += 1;
        writeln!(
            f,
            "{:<16} {:4} '{}'",
            name, constant, chunk.constants[constant as usize]
        )
    }

    fn dissassemble_constant_long_instruction<W: Write>(
        chunk: &Chunk,
        offset: &mut usize,
        name: &str,
        f: &mut W,
    ) -> std::fmt::Result {
        let constant = u32::from_be_bytes([
            chunk.code[*offset],
            chunk.code[*offset + 1],
            chunk.code[*offset + 2],
            chunk.code[*offset + 3],
        ]);
        *offset += 4;
        writeln!(
            f,
            "{:<16} {:4} '{}'",
            name, constant, chunk.constants[constant as usize]
        )
    }
}
