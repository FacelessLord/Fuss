use crate::lexer::common::lexer::{Position, Token};

pub enum ParserNode {
    NonTerminal {
        kind: String,
        span: (Position, Position),
        children: Vec<ParserNode>,
    },
    Terminal(Token),
}

pub enum ParserGrammarToken {
    NonTerminal(String),
    Terminal(String),
}
