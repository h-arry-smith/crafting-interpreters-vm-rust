use std::io::{Read, Write};

use clap::Parser;
use opcode::Opcode;

use crate::vm::Vm;

mod chunk;
mod compiler;
mod dissasembler;
mod opcode;
mod scanner;
mod value;
mod vm;

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli { path: Some(path) } => {
            run_file(path);
        }
        Cli { path: None } => {
            repl();
        }
    }
}

fn repl() {
    let mut vm = Vm::new();

    loop {
        print!("> ");
        std::io::stdout().flush().expect("Could not flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        vm.interpret(&input).expect("Could not interpret input");
    }
}

fn run_file(path: String) {
    let mut vm = Vm::new();

    let mut file = std::fs::File::open(path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    vm.interpret(&contents).expect("Could not interpret file");
}
