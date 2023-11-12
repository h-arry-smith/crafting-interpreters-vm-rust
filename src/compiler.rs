use crate::{
    chunk::Chunk,
    scanner::{Scanner, TokenType},
    vm::InterpretError,
};

pub struct Compiler;

impl Compiler {
    pub fn compile(source: &str) -> Result<Chunk, InterpretError> {
        let chunk = Chunk::new("main".to_string());

        let mut scanner = Scanner::new(source);

        let mut line = 0;
        while let Ok(token) = scanner.scan_token() {
            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!(
                "{:?} '{}'",
                token.token_type,
                &source[token.start..token.start + token.length]
            );

            if token.token_type == TokenType::Eof {
                break;
            }
        }

        Ok(chunk)
    }
}
