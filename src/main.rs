use chunk::Chunk;
use opcode::Opcode;

use crate::value::Value;

mod chunk;
mod opcode;
mod value;

fn main() {
    let mut chunk = Chunk::new("test chunk".into());

    let constant = chunk.add_constant(Value(1.2));
    chunk.write(Opcode::Constant, 123);
    chunk.write(u8::to_be_bytes(constant as u8), 123);

    chunk.write(Opcode::Return, 123);

    println!("{:?}", chunk);
}
