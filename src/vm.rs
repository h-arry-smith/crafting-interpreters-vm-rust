use std::{error::Error, fmt::Display};

use crate::{
    chunk::Chunk, compiler::Compiler, dissasembler::Dissasembler, opcode::Opcode, value::Value,
};

const STACK_MAX: usize = 256;

macro_rules! binary_op {
    ($self:ident, $op:tt) => {
        {
            let b = *$self.pop();
            let a = *$self.pop();
            let result = Value(a.0 $op b.0);
            $self.push(result);
        }
    }
}

pub struct Vm {
    chunk: Option<Chunk>,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

type InterpretResult = Result<(), InterpretError>;

impl Vm {
    pub fn new() -> Self {
        Vm {
            chunk: None,
            ip: 0,
            stack: [Value(0.0); STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        self.ip = 0;
        let chunk = Compiler::compile(source)?;
        self.chunk = Some(chunk);

        // self.run()?;

        Ok(())
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if std::env::var("DEBUG").is_ok() {
                print!("          ");
                for value in self.stack.iter().take(self.stack_top) {
                    print!("[ {} ]", value);
                }
                println!();

                // NOTE: Cloning the IP pointer here prevents the disassembler from moving the offset
                //       forward, which would cause the VM to skip instructions.
                Dissasembler::trace_instruction(self.chunk.as_ref().unwrap(), &mut self.ip.clone());
            }

            let opcode = self.read_opcode()?;

            match opcode {
                Opcode::Return => {
                    println!("{}", self.pop());
                    return Ok(());
                }
                Opcode::Constant => {
                    let constant = *self.read_constant()?;
                    self.push(constant);
                }
                Opcode::Negate => {
                    let negated_value = Value(-(self.pop().0));
                    self.push(negated_value);
                }
                Opcode::Add => binary_op!(self, +),
                Opcode::Subtract => binary_op!(self, -),
                Opcode::Divide => binary_op!(self, /),
                Opcode::Multiply => binary_op!(self, *),
                _ => return Err(InterpretError::CompileError),
            }

            if self.ip >= self.chunk.as_ref().unwrap().code.len() {
                return Ok(());
            }
        }
    }

    fn read_opcode(&mut self) -> Result<Opcode, InterpretError> {
        let opcode = self.chunk.as_ref().unwrap().code[self.ip];
        self.ip += 1;
        Ok(opcode.into())
    }

    fn read_constant(&mut self) -> Result<&Value, InterpretError> {
        let constant = (self.chunk.as_ref().unwrap()).code[self.ip];
        self.ip += 1;
        Ok(&self.chunk.as_ref().unwrap().constants[constant as usize])
    }

    fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> &Value {
        self.stack_top -= 1;
        &self.stack[self.stack_top]
    }
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::CompileError => write!(f, "Compile error"),
            InterpretError::RuntimeError => write!(f, "Runtime error"),
        }
    }
}

impl Error for InterpretError {}
