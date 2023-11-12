use chunk::Chunk;
use opcode::Opcode;

use crate::{value::Value, vm::Vm};

mod chunk;
mod dissasembler;
mod opcode;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new("test chunk".into());

    let constant = chunk.add_constant(Value(1.2));
    chunk.write_constant(constant, 123);

    let constant = chunk.add_constant(Value(3.4));
    chunk.write_constant(constant, 123);

    chunk.write(Opcode::Add, 123);

    let constant = chunk.add_constant(Value(5.6));
    chunk.write_constant(constant, 123);

    chunk.write(Opcode::Divide, 123);
    chunk.write(Opcode::Negate, 123);

    chunk.write(Opcode::Return, 123);

    if std::env::var("DEBUG").is_ok() {
        print!("{:?}", chunk);
    }

    let mut vm = Vm::new(&chunk);
    vm.interpret(None).expect("Could not interpret chunk");
}
