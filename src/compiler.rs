use crate::{
    chunk::Chunk,
    opcode::Opcode,
    scanner::{CompilerError, Scanner, Token, TokenType},
    value::Value,
    vm::InterpretError,
};

pub struct Compiler<'src> {
    chunk: Option<Chunk>,
    parser: Parser<'src>,
    scanner: Scanner<'src>,
}

impl<'src> Compiler<'src> {
    pub fn new(source: &'src str) -> Self {
        Compiler {
            chunk: Some(Chunk::new("main".to_string())),
            parser: Parser::new(source),
            scanner: Scanner::new(source),
        }
    }

    pub fn compile(&mut self) -> Result<Chunk, InterpretError> {
        self.parser.advance(&mut self.scanner);

        expression(self);

        self.parser.consume(
            &mut self.scanner,
            TokenType::Eof,
            "Expect end of expression.",
        );

        self.end_compiler();

        match self.parser.had_error {
            true => Err(InterpretError::CompileError),
            false => Ok(self.chunk.take().unwrap()),
        }
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        if std::env::var("DUMP").is_ok() {
            if let Some(chunk) = &self.chunk {
                eprintln!("{:?}", chunk);
            }
        }
    }

    fn emit_bytes<O>(&mut self, bytes: O)
    where
        O: Into<Vec<u8>>,
    {
        let previous_line = self.parser.previous.line;
        self.current_chunk().write(bytes, previous_line);
    }

    fn emit_pair(&mut self, bytes: (Opcode, u8)) {
        self.emit_bytes(bytes.0);
        self.emit_bytes([bytes.1]);
    }

    fn emit_return(&mut self) {
        self.emit_bytes(Opcode::Return);
    }

    fn emit_constant(&mut self, constant: u32) {
        let line = self.parser.previous.line;
        self.current_chunk().write_constant(constant, line)
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunk.as_mut().unwrap()
    }

    fn make_constant(&mut self, value: Value) -> u32 {
        let constant = self.current_chunk().add_constant(value);
        // FIXME: Is this right?
        if constant > u32::MAX - 1 {
            self.parser.error_at_current(&CompilerError {
                message: "Too many constants in one chunk.".to_string(),
                line: self.parser.previous.line,
            });
            return 0;
        }
        constant
    }
}

struct Parser<'src> {
    source: &'src str,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        Parser {
            source,
            current: Token::new(TokenType::Eof, 0, 0, 0),
            previous: Token::new(TokenType::Eof, 0, 0, 0),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self, scanner: &mut Scanner) {
        self.previous = self.current;

        loop {
            match scanner.scan_token() {
                Ok(token) => {
                    self.current = token;
                    break;
                }
                Err(error) => self.error_at_current(&error),
            }
        }
    }

    fn consume(&mut self, scanner: &mut Scanner, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance(scanner);
            return;
        }

        self.error_at_current(&CompilerError {
            message: message.to_string(),
            line: self.current.line,
        });
    }

    fn error_at_current(&mut self, message: &CompilerError) {
        self.print_error_message(&self.current, message);
        self.had_error = true;
    }

    fn error_at(&mut self, token: &Token, message: &CompilerError) {
        self.print_error_message(token, message);
        self.had_error = true;
    }

    fn print_error_message(&self, token: &Token, message: &CompilerError) {
        if self.panic_mode {
            return;
        }

        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message.message);
        } else {
            eprintln!(
                "[line {}] Error at '{}': {}",
                token.line,
                token.lexeme(self.source),
                message.message
            );
        }
    }
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::None,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

struct ParseRule {
    prefix: Option<fn(&mut Compiler)>,
    infix: Option<fn(&mut Compiler)>,
    precedence: Precedence,
}

fn parse_precedence(compiler: &mut Compiler, precedence: Precedence) {
    compiler.parser.advance(&mut compiler.scanner);
    let prefix_rule = get_rule(compiler.parser.previous.token_type).prefix;

    match prefix_rule {
        Some(prefix) => prefix(compiler),
        None => compiler.parser.error_at_current(&CompilerError {
            message: "Expect expression.".to_string(),
            line: compiler.parser.previous.line,
        }),
    }

    while precedence <= get_rule(compiler.parser.current.token_type).precedence {
        compiler.parser.advance(&mut compiler.scanner);
        let infix_rule = get_rule(compiler.parser.previous.token_type).infix;

        if let Some(infix) = infix_rule {
            infix(compiler);
        }
    }
}

#[rustfmt::skip]
fn get_rule(token_type: TokenType) -> ParseRule {
    match token_type {
         TokenType::LeftParen => ParseRule { prefix: Some(grouping), infix: None, precedence: Precedence::None },
        TokenType::RightParen => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
         TokenType::LeftBrace => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
        TokenType::RightBrace => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Comma => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::Dot => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Minus => ParseRule { prefix: Some(unary), infix: Some(binary), precedence: Precedence::Term },
              TokenType::Plus => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Term },
         TokenType::Semicolon => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Slash => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
              TokenType::Star => ParseRule { prefix: None, infix: Some(binary), precedence: Precedence::Factor },
              TokenType::Bang => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
         TokenType::BangEqual => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Equal => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
        TokenType::EqualEqual => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
           TokenType::Greater => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
      TokenType::GreaterEqual => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
              TokenType::Less => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
         TokenType::LessEqual => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
        TokenType::Identifier => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::String => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Number => ParseRule { prefix: Some(number), infix: None, precedence: Precedence::None },
               TokenType::And => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Class => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
              TokenType::Else => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::False => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::For => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::Fun => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                TokenType::If => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::Nil => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                TokenType::Or => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Print => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            TokenType::Return => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Super => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
              TokenType::This => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
              TokenType::True => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::Var => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::While => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
             TokenType::Error => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
               TokenType::Eof => ParseRule { prefix: None, infix: None, precedence: Precedence::None },
    }
}

fn expression(compiler: &mut Compiler) {
    parse_precedence(compiler, Precedence::Assignment);
}

fn grouping(compiler: &mut Compiler) {
    expression(compiler);
    compiler.parser.consume(
        &mut compiler.scanner,
        TokenType::RightParen,
        "Expect ')' after expression.",
    );
}

fn unary(compiler: &mut Compiler) {
    let operator_type = compiler.parser.previous.token_type;

    // Compile the operand.
    parse_precedence(compiler, Precedence::Unary);

    // Emit the operator instruction.
    match operator_type {
        TokenType::Minus => compiler.emit_bytes(Opcode::Negate),
        _ => unreachable!(),
    }
}

fn binary(compiler: &mut Compiler) {
    let operator_type = compiler.parser.previous.token_type;

    // Compile the right operand.
    let rule = get_rule(operator_type);
    parse_precedence(compiler, rule.precedence.next());

    // Emit the operator instruction.
    match operator_type {
        TokenType::Plus => compiler.emit_bytes(Opcode::Add),
        TokenType::Minus => compiler.emit_bytes(Opcode::Subtract),
        TokenType::Star => compiler.emit_bytes(Opcode::Multiply),
        TokenType::Slash => compiler.emit_bytes(Opcode::Divide),
        _ => unreachable!(),
    }
}

fn number(compiler: &mut Compiler) {
    let value = compiler.parser.previous.lexeme(compiler.parser.source);
    let value = value.parse::<f64>().unwrap();
    let constant = compiler.make_constant(Value(value));

    compiler.emit_constant(constant);
}
