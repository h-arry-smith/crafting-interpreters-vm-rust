use std::{error::Error, fmt::Display};

use crate::{
    chunk::Chunk,
    compiler::Compiler,
    dissasembler::Dissasembler,
    opcode::Opcode,
    value::{Obj, ObjType, Value},
};

const STACK_MAX: usize = 256;

macro_rules! binary_op {
    ($self:ident, $op:tt, $value_type:ident) => {
        {
            if !$self.peek(0).is_number() || !$self.peek(1).is_number() {
                $self.runtime_error("Operands must be numbers");
                return Err(InterpretError::RuntimeError);
            }

            let b = $self.pop();
            let a = $self.pop();
            let result = Value::$value_type(a.as_f64().unwrap() $op b.as_f64().unwrap());
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
        const value: Value = Value::Nil;
        Vm {
            chunk: None,
            ip: 0,
            stack: [value; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        self.ip = 0;
        let mut compiler = Compiler::new(source);
        let chunk = compiler.compile()?;
        self.chunk = Some(chunk);

        self.run()?;

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
                    let constant = self.read_constant()?.clone();
                    self.push(constant);
                }
                Opcode::Nil => {
                    self.push(Value::Nil);
                }
                Opcode::True => {
                    self.push(Value::Bool(true));
                }
                Opcode::False => {
                    self.push(Value::Bool(false));
                }
                Opcode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    let result = Value::Bool(a == b);
                    self.push(result);
                }
                Opcode::Greater => binary_op!(self, >, Bool),
                Opcode::Less => binary_op!(self, <, Bool),
                Opcode::Negate => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number");
                        return Err(InterpretError::RuntimeError);
                    }

                    let negated_value = Value::Number(-(self.pop().as_f64().unwrap()));
                    self.push(negated_value);
                }
                Opcode::Add => {
                    if self.peek(0).is_obj_type(ObjType::String)
                        && self.peek(1).is_obj_type(ObjType::String)
                    {
                        self.concatenate();
                    } else if self.peek(0).is_number() && self.peek(1).is_number() {
                        let a = self.pop();
                        let b = self.pop();
                        let result = Value::Number(a.as_f64().unwrap() + b.as_f64().unwrap());
                        self.push(result);
                    } else {
                        self.runtime_error("Operands must be two numbers or two strings");
                        return Err(InterpretError::RuntimeError);
                    }
                }
                Opcode::Subtract => binary_op!(self, -, Number),
                Opcode::Divide => binary_op!(self, /, Number),
                Opcode::Multiply => binary_op!(self, *, Number),
                Opcode::Not => {
                    let result = Value::Bool(self.pop().is_falsey());
                    self.push(result);
                }
                _ => return Err(InterpretError::CompileError),
            }

            if self.ip >= self.chunk.as_ref().unwrap().code.len() {
                return Ok(());
            }
        }
    }

    fn concatenate(&mut self) {
        let b = self.pop();
        let a = self.pop();

        let mut a = a.as_string().unwrap();
        let b = b.as_string().unwrap();

        a.push_str(&b);

        let result = Value::Obj(Obj::String(a));
        self.push(result);
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

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top].clone()
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack_top - 1 - distance]
    }

    fn reset_stack(&mut self) {
        self.stack_top = 0;
    }

    fn runtime_error(&mut self, message: &str) {
        let line = self.chunk.as_ref().unwrap().line_for_instruction_n(self.ip);
        eprintln!("[line {}] Error: {}", line, message);
        self.reset_stack();
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
