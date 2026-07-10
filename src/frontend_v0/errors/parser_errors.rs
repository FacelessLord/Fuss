use crate::frontend_v0::errors::common::MessageFactory;
use crate::frontend_v0::lexer::common::lexer::Position;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use crate::frontend_v0::errors::ast_errors::AstBuilderError;

pub enum ParserError {
    UnexpectedToken {
        given_token_kind: String,
        position: Position,
        expected_token_kinds: HashSet<String>,
    },
}

impl MessageFactory for ParserError {
    fn get_message(&self) -> String {
        match self {
            ParserError::UnexpectedToken {
                given_token_kind,
                expected_token_kinds,
                position,
                ..
            } => {
                let expected_token_list = expected_token_kinds
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!(
                    "Unexpected token {given_token_kind}. Expected {expected_token_list} at {position}"
                )
            }
        }
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}