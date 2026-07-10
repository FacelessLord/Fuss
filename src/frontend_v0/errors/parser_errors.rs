use crate::frontend_v0::lexer::common::lexer::Position;
use std::collections::HashSet;
use std::fmt::{Debug, Display};

pub enum ParserError {
    UnexpectedToken {
        given_token_kind: String,
        position: Position,
        expected_token_kinds: HashSet<String>,
    },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
                write!(
                    f,
                    "Unexpected token {given_token_kind}. Expected {expected_token_list} at {position}"
                )
            }
        }
    }
}
