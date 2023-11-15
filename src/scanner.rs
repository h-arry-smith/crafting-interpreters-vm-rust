pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

macro_rules! match_or {
    ($match:tt, $self:ident, $true:ident, $false:ident) => {{
        let token_type = if $self.match_char($match) {
            TokenType::$true
        } else {
            TokenType::$false
        };
        Ok($self.make_token(token_type))
    }};
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, CompilerError> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.make_token(TokenType::Eof));
        }

        let c = self.advance();

        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '!' => match_or!('=', self, BangEqual, Bang),
            '=' => match_or!('=', self, EqualEqual, Equal),
            '<' => match_or!('=', self, LessEqual, Less),
            '>' => match_or!('=', self, GreaterEqual, Greater),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => Err(CompilerError {
                line: self.line,
                message: format!("Unexpected character: {}", c),
            }),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current - self.start, self.line)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<Token, CompilerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(CompilerError {
                line: self.line,
                message: "Unterminated string".to_string(),
            });
        }

        self.advance();

        Ok(self.make_token(TokenType::String))
    }

    fn number(&mut self) -> Result<Token, CompilerError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        Ok(self.make_token(TokenType::Number))
    }

    fn identifier(&mut self) -> Result<Token, CompilerError> {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) || self.peek() == '_' {
            self.advance();
        }
        Ok(self.make_token(self.identifier_type()))
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start) {
            Some(c) => match c {
                'a' => self.check_keyword(1, 2, "nd", TokenType::And),
                'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
                'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
                'f' => {
                    if self.current - self.start > 1 {
                        match self.source.chars().nth(self.start + 1).unwrap() {
                            'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                            'o' => self.check_keyword(2, 1, "r", TokenType::For),
                            'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                            _ => TokenType::Identifier,
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                'i' => self.check_keyword(1, 1, "f", TokenType::If),
                'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
                'o' => self.check_keyword(1, 1, "r", TokenType::Or),
                'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
                'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
                's' => self.check_keyword(1, 4, "uper", TokenType::Super),
                't' => {
                    if self.current - self.start > 1 {
                        match self.source.chars().nth(self.start + 1).unwrap() {
                            'h' => self.check_keyword(2, 2, "is", TokenType::This),
                            'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                            _ => TokenType::Identifier,
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
                'w' => self.check_keyword(1, 4, "hile", TokenType::While),
                _ => TokenType::Identifier,
            },
            None => TokenType::Error,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        dbg!("heck_keyword");
        if dbg!(self.current - self.start == start + length)
            && dbg!(&self.source[self.start + start..self.start + start + length] == rest)
        {
            token_type
        } else {
            TokenType::Identifier
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, length: usize, line: usize) -> Self {
        Token {
            token_type,
            start,
            length,
            line,
        }
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.start + self.length]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character .
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

pub struct CompilerError {
    pub line: usize,
    pub message: String,
}
