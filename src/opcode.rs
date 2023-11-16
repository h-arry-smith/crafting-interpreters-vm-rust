#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Return,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Divide,
    Multiply,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::Return,
            1 => Opcode::Constant,
            2 => Opcode::ConstantLong,
            3 => Opcode::Negate,
            4 => Opcode::Add,
            5 => Opcode::Subtract,
            6 => Opcode::Divide,
            7 => Opcode::Multiply,
            8 => Opcode::Nil,
            9 => Opcode::True,
            10 => Opcode::False,
            11 => Opcode::Not,
            12 => Opcode::Equal,
            13 => Opcode::Greater,
            14 => Opcode::Less,
            _ => panic!("Unknown opcode {}", byte),
        }
    }
}

impl From<Opcode> for Vec<u8> {
    fn from(opcode: Opcode) -> Self {
        vec![opcode as u8]
    }
}
