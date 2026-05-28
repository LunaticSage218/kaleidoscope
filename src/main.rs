// lexer
use std::io::{self, Read};


// since we are dealing with tokens a lot, these will be helpful
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eof, 
    Def, 
    Extern, 
    Identifier(String), // this syntax means Indentifier owns the value, whereas Eof is just the literal 0
    Number(f64), 
    Other(char),
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------
 
// Every token the lexer can produce. Unknown bytes are returned as their
// ASCII value cast to i32 (e.g. '+' == 43) so the parser can match them
// directly — matching the original C++ tutorial's approach.

pub struct Lexer<R: Read> {
    input: R, 
    last_char: Option<char>,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Self {
            input, 
            last_char: Some(' '), // prime the pump
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let mut buf = [0u8; 1];
        match self.input.read(&mut buf) {
            Ok(1) => Some(buf[0] as char),
            _ => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.last_char = self.next_char();
        self.last_char
    }

    pub fn next_token(&mut self) -> Token {
        while self.last_char.map_or(false, |c| c.is_whitespace()) {
            self.advance();
        }

        match self.last_char {
            None => Token::Eof, 
            Some(c) if c.is_alphanumeric() => self.lex_identifier(),
            Some(c) if c.is_ascii_digit() => self.lex_number(), 
            Some('#') => self.lex_comment(),
            Some(c) => {
                self.advance();
                Token::Other(c)
            } 
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.last_char {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            }

            else {
                break;
            }
        }

        match ident.as_str() {
            "def" => Token::Def, 
            "extern" => Token::Extern,
            _ => Token::Identifier(ident),
        }
    }

    fn lex_number(&mut self) -> Token {
        let mut num_str = String::new();

        while let Some(c) = self.last_char {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.advance();
            }

            else {
                break;
            }
        }

        let value: f64 = num_str.parse().unwrap_or(0.0);
        Token::Number(value)
    }

    fn lex_comment(&mut self) -> Token {
        while let Some(c) = self.last_char {
            self.advance();

            if c == '\n' || c == '\r' {
                break;
            }
        }

        self.next_token()
    }
}

// ---------------------------------------------------------------------------
// AST
// ---------------------------------------------------------------------------
//
// Prefer an *enum* over `Box<dyn Trait>` for a closed set of node types:
//   - No heap allocation per node (unless the variant itself contains one)
//   - Pattern matching instead of virtual dispatch
//   - Derives (Debug, Clone, PartialEq) come for free

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64), 
    Variable(String), 
    Binary {
        op: char, 
        lhs: Box<Expr>, 
        rhs: Box<Expr>,
    },

    Call {
        callee: String, 
        args: Vec<Expr>,
    }
}

#[derive(Debug, Clone)]
pub struct Prototype {
    pub name: String, 
    pub params: Vec<String>,
}

impl Prototype {
    pub fn new(name: impl Into<String>, params: Vec<String>) -> Self {
        Self {name: name.into(), params}
    }
}

pub struct Function {
    pub proto: Prototype, 
    pub body: Expr, 
}

impl Function {
    pub fn new(proto: Prototype, body: Expr) -> Self {
        Self {proto, body}
    }
}

// ---------------------------------------------------------------------------
// Parser (skeleton — wire up as you extend the tutorial)
// ---------------------------------------------------------------------------



fn main() {
    println!("hi!");
}