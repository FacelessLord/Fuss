use crate::char_len;

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

pub struct LexerError {
    position: Position,
    message: String,
}

impl LexerError {
    pub fn get_message(&self) -> String {
        format!(
            "{message} at {file}:{line}:{column}",
            message = self.message,
            file = self.position.filename,
            line = self.position.line,
            column = self.position.column
        )
    }

    pub fn new(message: String, position: Position) -> LexerError {
        LexerError { position, message }
    }
}
pub trait Lexer {
    fn tokenize(&self, filename: &String, file: String) -> (Vec<Token>, Vec<LexerError>);
}

pub const EOF: &str = "EOF";