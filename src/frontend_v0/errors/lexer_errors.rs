use crate::frontend_v0::errors::common::MessageFactory;

pub enum LexerError {
    UnknownCharSequence {
        sequence: String,
    },
}

impl MessageFactory for LexerError {
    fn get_message(&self) -> String {
        match self {
            LexerError::UnknownCharSequence {
                sequence, ..
            } => {
                format!("Unknown char sequence \"{sequence}\"")
            }
        }
    }
}
