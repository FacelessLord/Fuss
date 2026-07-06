use crate::lexer::common::lexer::{Position, Token};
use std::fmt::Debug;

pub enum ParserNode {
    NonTerminal {
        kind: String,
        span: (Position, Position),
        children: Vec<ParserNode>,
    },
    Terminal(Token),
}

impl Debug for ParserNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ParserNode {
    pub(crate) fn clone(&self) -> ParserNode {
        match self {
            ParserNode::NonTerminal {
                kind,
                span,
                children,
            } => ParserNode::NonTerminal {
                kind: kind.clone(),
                span: (span.0.clone(), span.1.clone()),
                children: children.iter().map(|x| x.clone()).collect(),
            },
            ParserNode::Terminal(token) => ParserNode::Terminal(token.clone()),
        }
    }
    pub fn get_node_span(&self) -> (Position, Position) {
        match self {
            ParserNode::NonTerminal { span, .. } => (span.0.clone(), span.1.clone()),
            ParserNode::Terminal(token) => (token.position.clone(), token.get_end_position()),
        }
    }
    pub fn get_node_kind(&self) -> String {
        match self {
            ParserNode::NonTerminal { kind, .. } => kind.clone(),
            ParserNode::Terminal(token) => token.kind.clone(),
        }
    }
    pub fn get_node_start(&self) -> Position {
        match self {
            ParserNode::NonTerminal { span, .. } => span.0.clone(),
            ParserNode::Terminal(token) => token.position.clone(),
        }
    }
}

pub fn join_node_spans(first: &ParserNode, second: &ParserNode) -> (Position, Position) {
    let first_span = first.get_node_span();
    let second_span = second.get_node_span();
    (first_span.0.clone(), second_span.1.clone())
}
