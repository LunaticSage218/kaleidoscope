use core::fmt;
use std::format;
use std::iter::Peekable;
use std::path::Component::Normal;
use std::path::Iter;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Def, 
    Extern, 
    Identifier(String), 
    Number(f64), 
    Other(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize, 
    pub end: usize, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T, 
    pub span: Span, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    BadNumber {text: String, span: Span},
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::BadNumber { text, span } => {
                write!(f, "invalid number `{text}` at byte {}", span.start)
            }
        }
    }
}

impl std::error::Error for LexError {}

pub struct Lexer<'a> {
    src: &'a str, 
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self{
        Self {
            src, 
            chars: src.char_indices().peekable(),
        }
    }

    fn pos(&mut self) -> usize {
        self.chars.peek().map_or(self.src.len(), |&(i, _)| i)
    }

    fn eat_while(&mut self, pred: impl Fn(char) -> bool) -> usize {
        while self.chars.next_if(|&(_, c)| pred(c)).is_some() {}
        self.pos()
    }

    fn skip_trivia(&mut self) {
        self.eat_while(char::is_whitespace);
        while self.chars.next_if(|&(_, c)| c == '#').is_some() {
            self.eat_while(|c: char| c != '\n');
            self.eat_while(char::is_whitespace);
        }
    }

    fn lex_token(&mut self, start: usize, c: char) -> Result<Spanned<Token>, LexError> {
        let node: Token = if c.is_ascii_alphabetic() {
            let end: usize = self.eat_while(|c: char| c.is_ascii_alphanumeric() || c == '_');
            match &self.src[start..end] {
                "def" => Token::Def,
                "extern" => Token::Extern,
                ident => Token::Identifier(ident.to_string()),
            }
        } else if c.is_ascii_digit() {
            let end: usize = self.eat_while(|c: char| c.is_ascii_digit() || c == '.');
            let text: &str = &self.src[start..end];
            let n: f64 = text.parse().map_err(|_| LexError::BadNumber {
                text: text.to_string(),
                span: Span { start, end },
            })?;
            Token::Number(n)
        } else {
            Token::Other(c)
        };

        let end: usize = self.pos();
        let span: Span = Span { start, end };
        Ok(Spanned { node, span })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Spanned<Token>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_trivia();
        let (start, c) = self.chars.next()?;
        Some(self.lex_token(start, c))
    }
}