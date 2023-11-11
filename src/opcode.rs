#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Return,
    Constant,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::Return,
            1 => Opcode::Constant,
            _ => panic!("Unknown opcode {}", byte),
        }
    }
}

impl From<Opcode> for Vec<u8> {
    fn from(opcode: Opcode) -> Self {
        vec![opcode as u8]
    }
}
