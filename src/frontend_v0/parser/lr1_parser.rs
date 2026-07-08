use crate::frontend_v0::lexer::common::lexer::{Position, Token};
use crate::frontend_v0::parser::common::parser::{join_node_spans, ParserNode};
use crate::frontend_v0::parser::lr1_automata_builder::{LR1Automata, LR1ParserAction};
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;

pub struct LR1Parser {
    automata: LR1Automata,
    state: usize,
    stack: Vec<(usize, ParserNode)>,
}

pub enum ParserError {
    UnexpectedToken(ParserNode, Position, HashSet<String>),
}

impl ParserError {
    pub fn get_message(&self) -> String {
        match self {
            &ParserError::UnexpectedToken(ref token, ref position, ref _expected_tokens) => {
                format!(
                    "{message} at {file}:{line}:{column}",
                    message = format!(
                        "Unexpected token {0}. Expected ",
                        token.get_node_kind(),
                        // expected_tokens
                        //     .iter()
                        //     .map(|x| x.clone())
                        //     .collect::<Vec<String>>()
                        //     .join(", ")
                    ),
                    file = position.filename,
                    line = position.line,
                    column = position.column
                )
            }
        }
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl LR1Parser {
    pub(crate) fn new(automata: LR1Automata) -> LR1Parser {
        LR1Parser {
            automata,
            state: 0,
            stack: vec![],
        }
    }
    
    pub fn reset(&mut self) {
        self.state = 0;
        self.stack = vec![];
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> (ParserNode, Vec<ParserError>) {
        let mut errors = Vec::new();
        let mut real_tokens = VecDeque::from(
            tokens
                .iter()
                .map(|x| ParserNode::Terminal(x.clone()))
                .collect::<Vec<_>>(),
        );

        while real_tokens.len() > 0 {
            // Peek at the current token
            let current_token = real_tokens[0].clone();

            let node_kind = current_token.get_node_kind();
            let action = self.automata.goto_table[self.state].get(&node_kind).ok_or(
                ParserError::UnexpectedToken(
                    current_token.clone(),
                    current_token.get_node_start(),
                    self.automata.goto_table[self.state]
                        .iter()
                        .map(|(x, _)| x.clone())
                        .collect::<HashSet<String>>(),
                ),
            );
            match action {
                Ok(parser_action) => match parser_action {
                    LR1ParserAction::Shift(next_state) => {
                        real_tokens.pop_front();
                        self.stack.push((self.state, current_token));
                        self.state = *next_state
                    }
                    LR1ParserAction::Reduce(rule) => {
                        let (new_node, last_state) = self.reduce(*rule);
                        real_tokens.push_front(new_node);
                        self.state = last_state;
                    }
                },
                Err(err) => {
                    // Skipping unknown token
                    real_tokens.pop_front();
                    errors.push(err);
                }
            }
        }

        let (tree, _) = self.reduce(0);

        (tree, errors)
    }

    fn reduce(&mut self, rule: usize) -> (ParserNode, usize) {
        let rule = &self.automata.grammar.rules[rule];
        let mut consumed_tokens = Vec::new();
        let mut last_state = self.state;
        for _i in 0..rule.production.len() {
            let (state, token) = self.stack.pop().unwrap();
            consumed_tokens.push(token);
            last_state = state;
        }
        consumed_tokens.reverse();
        let span = join_node_spans(
            &consumed_tokens[0],
            &consumed_tokens[consumed_tokens.len() - 1],
        );
        let new_node = ParserNode::NonTerminal {
            kind: rule.name.clone(),
            children: consumed_tokens,
            span: span,
        };

        (new_node, last_state)
    }
}
