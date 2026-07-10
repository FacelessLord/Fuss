use crate::frontend_v0::errors::common::MessageFactory;
use crate::frontend_v0::lexer::common::lexer::{Position, Token};
use crate::frontend_v0::parser::common::parser::Span;
use std::fmt::{Debug, Display, Formatter};

pub enum AstBuilderError {
    TerminalExpected {
        expected_node: String,
        given_token_kind: String,
        position: Position,
    },
    NonTerminalExpected {
        expected_node: String,
        given_token: Token,
    },
    UnknownRule {
        expected_node: String,
        given_token_amount: usize,
        position: Position,
    },
    UnexpectedTokenKind {
        expected_node: String,
        given_token_kind: String,
        position: Position,
    },
    UnexpectedSyntaxNode {
        expected_node: String,
        given_node: String,
        position: Position,
    },
}

impl MessageFactory for AstBuilderError {
    fn get_message(&self) -> String {
        match self {
            AstBuilderError::TerminalExpected {
                expected_node,
                given_token_kind,
                position,
            } => {
                format!(
                    "Expected terminal {}, but got {} at {}",
                    expected_node, given_token_kind, position
                )
            }
            AstBuilderError::NonTerminalExpected {
                expected_node,
                given_token,
            } => {
                format!(
                    "Expected non-terminal {}, but got {} at {}",
                    expected_node,
                    given_token.kind.clone(),
                    given_token.position
                )
            }
            AstBuilderError::UnknownRule {
                given_token_amount,
                expected_node,
                position,
            } => {
                format!(
                    "Found unknown rule at production size {}, but got {} at {}",
                    given_token_amount, expected_node, position
                )
            }
            AstBuilderError::UnexpectedTokenKind {
                expected_node,
                given_token_kind,
                position,
            } => {
                format!(
                    "Expected {}, but got {} at {}",
                    expected_node, given_token_kind, position
                )
            }
            AstBuilderError::UnexpectedSyntaxNode {
                expected_node,
                given_node,
                position,
            } => {
                format!(
                    "Expected {}, but got {} at {}",
                    expected_node, given_node, position
                )
            }
        }
    }
}

impl Debug for AstBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

pub struct ErrorBuilder {}

impl ErrorBuilder {
    pub fn terminal_expected<T>(
        expected_node: &str,
        given_token_kind: String,
        position: Position,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::TerminalExpected {
            expected_node: expected_node.to_string(),
            given_token_kind,
            position,
        })
    }
    pub fn non_terminal_expected<T>(
        expected_node: &str,
        given_token: Token,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::NonTerminalExpected {
            expected_node: expected_node.to_string(),
            given_token,
        })
    }
    pub fn unknown_rule_for<T>(
        expected_node: &str,
        given_token_amount: usize,
        position: Position,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::UnknownRule {
            expected_node: expected_node.to_string(),
            given_token_amount,
            position,
        })
    }
    pub fn unexpected_token_kind<T>(
        expected_node: &str,
        given_token_kind: &str,
        position: Position,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::UnexpectedTokenKind {
            expected_node: expected_node.to_string(),
            given_token_kind: given_token_kind.to_string(),
            position,
        })
    }
    pub fn unexpected_syntax_node<T>(
        expected_node: &str,
        given_node: &str,
        position: Position,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::UnexpectedSyntaxNode {
            expected_node: expected_node.to_string(),
            given_node: given_node.to_string(),
            position,
        })
    }
}
