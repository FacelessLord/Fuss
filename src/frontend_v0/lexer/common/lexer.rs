use crate::char_len;
use std::fmt::{Display, Formatter};
use crate::frontend_v0::errors::lexer_errors::LexerError;
use crate::frontend_v0::message_controller::MessageController;

#[derive(Debug, Clone)]
pub struct Position {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn clone(&self) -> Position {
        Position {
            filename: self.filename.clone(),
            line: self.line,
            column: self.column,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.filename,
            line = self.line,
            column = self.column
        )
    }
}

pub struct Token {
    pub text: String,
    pub kind: String,
    pub position: Position,
    pub is_fallback: bool,
}

impl Token {
    pub(crate) fn clone(&self) -> Token {
        Token {
            text: self.text.clone(),
            kind: self.kind.clone(),
            position: self.position.clone(),
            is_fallback: self.is_fallback,
        }
    }
}

impl Token {
    pub fn get_end_position(&self) -> Position {
        Position {
            filename: self.position.filename.clone(),
            line: self.position.line,
            column: self.position.column + char_len!(self.text),
        }
    }
}

pub const EOF: &str = "EOF";

pub trait Lexer {
    fn tokenize(&mut self, filename: &String, file: String, message_controller: &MessageController) -> (Vec<Token>, Vec<LexerError>);
}