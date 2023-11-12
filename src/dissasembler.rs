use crate::chunk::Chunk;
use crate::Opcode;

pub struct Dissasembler {}

impl Dissasembler {
    pub fn disassemble(chunk: &Chunk, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("== {} ==", chunk.name);

        let mut offset = 0;
        while offset < chunk.code.len() {
            Self::disassemble_instruction(chunk, &mut offset, f)?;
        }

        Ok(())
    }

    fn disassemble_instruction(
        chunk: &Chunk,
        offset: &mut usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:04} ", offset)?;

        Self::write_line_info(chunk, *offset, f)?;

        let opcode = chunk.code[*offset];
        *offset += 1;

        match opcode.into() {
            Opcode::Return => {
                Self::disassemble_simple_instruction(offset, "Return", f)?;
            }
            Opcode::Constant => {
                Self::dissassemble_constant_instruction(chunk, offset, "Constant", f)?;
            }
            Opcode::ConstantLong => {
                Self::dissassemble_constant_long_instruction(chunk, offset, "ConstantLong", f)?;
            }
        }

        Ok(())
    }

    fn write_line_info(
        chunk: &Chunk,
        offset: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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

    fn disassemble_simple_instruction(
        offset: &mut usize,
        name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        *offset += 1;
        writeln!(f, "{}", name)
    }

    fn dissassemble_constant_instruction(
        chunk: &Chunk,
        offset: &mut usize,
        name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let constant = chunk.code[*offset];
        *offset += 1;
        writeln!(
            f,
            "{:<16} {:4} '{}'",
            name, constant, chunk.constants[constant as usize]
        )
    }

    fn dissassemble_constant_long_instruction(
        chunk: &Chunk,
        offset: &mut usize,
        name: &str,
        f: &mut std::fmt::Formatter<'_>,
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
