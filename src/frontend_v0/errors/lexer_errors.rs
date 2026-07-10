use std::fmt::{Debug, Formatter};
use crate::frontend_v0::errors::common::MessageFactory;
use crate::frontend_v0::lexer::common::lexer::Position;

pub enum LexerError {
    UnknownCharSequence {
        sequence: String,
        position: Position,
    },
}

impl MessageFactory for LexerError {
    fn get_message(&self) -> String {
        match self {
            LexerError::UnknownCharSequence { sequence, position } => {
                format!("Unknown char sequence \"{sequence}\" as {position}")
            }
        }
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}
