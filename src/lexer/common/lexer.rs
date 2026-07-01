use crate::char_len;

pub struct Position {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

pub struct Token {
    pub text: String,
    pub kind: String,
    pub position: Position,
    pub is_fallback: bool,
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

    pub fn create_error(message: String, position: Position) -> LexerError {
        LexerError { position, message }
    }
}
pub trait Lexer {
    fn tokenize(&self, filename: &String, file: String) -> (Vec<Token>, Vec<LexerError>);
}
