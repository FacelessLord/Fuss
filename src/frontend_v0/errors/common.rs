use crate::frontend_v0::lexer::common::lexer::Position;

pub trait MessageFactory {
    fn get_message(&self) -> String;
}
