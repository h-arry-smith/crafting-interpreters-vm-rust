use chunk::Chunk;
use opcode::Opcode;

use crate::value::Value;

mod chunk;
mod dissasembler;
mod opcode;
mod value;

fn main() {
    let mut chunk = Chunk::new("test chunk".into());

    let mut constant = 0;
    for x in 0..100_000 {
        constant = chunk.add_constant(Value(x as f64));
    }

    chunk.write_constant(constant as u32, 122);

    chunk.write(Opcode::Return, 123);

    println!("{:?}", chunk);
}
