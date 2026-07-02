use crate::char_len;
use crate::lexer::common::lexer::{Lexer, LexerError, Position, Token};
use crate::lexer::common::lexer_raw_grammar::read_raw_lexer_grammar;
use crate::lexer::lexer_regex_grammar::{
    process_grammar, LexerRegexGrammar, LexerRegexGrammarRule,
};
use regex::Regex;
use std::collections::HashSet;
use std::io::Error;
use std::string::ToString;

pub struct RegexLexer {
    grammar: LexerRegexGrammar,
    alphabet: HashSet<String>,
}

impl Lexer for RegexLexer {
    fn tokenize(&self, filename: &String, file: String) -> (Vec<Token>, Vec<LexerError>) {
        let empty_rule: LexerRegexGrammarRule = LexerRegexGrammarRule {
            name: String::from("EMPTY"),
            regex: Regex::new("").unwrap(),
        };

        let lines = file.lines();
        let rules = &self.grammar.rules;
        let fallback_rules = &self.grammar.fallback_rules;
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut line_number = 0;

        for line in lines {
            let mut buffer = line.to_string();
            let mut column = 0;

            while !buffer.is_empty() {
                let trimmed_buffer = buffer.trim_start().to_string();
                column += char_len![buffer] - char_len![trimmed_buffer];
                buffer = trimmed_buffer;

                if buffer.is_empty() {
                    break;
                }

                let position = Position {
                    filename: filename.clone(),
                    line: line_number + 1,
                    column,
                };

                let mut max_rule;
                // size in bytes for matched string
                let mut max_len;
                let mut fallback_happened = false;

                (max_rule, max_len) = match_rules(&buffer, &rules, &empty_rule);

                if max_rule.name == empty_rule.name {
                    (max_rule, max_len) = match_rules(&buffer, &fallback_rules, &empty_rule);
                    fallback_happened = true;
                }

                if max_rule.name == empty_rule.name {
                    errors.push(LexerError::create_error(
                        format!("unknown char sequence \"{0}\"", buffer),
                        position,
                    ));
                    break;
                }

                let matched_string = buffer[..max_len].to_string();
                buffer = buffer.strip_prefix(&matched_string).unwrap().to_string();

                tokens.push(Token {
                    text: matched_string,
                    kind: max_rule.name.clone(),
                    position,
                    is_fallback: fallback_happened,
                });

                column = column + max_len;
            }
            line_number += 1;
        }
        tokens.push(Token {
            text: "".to_string(),
            kind: "eof".to_string(),
            position: Position {
                filename: filename.clone(),
                line: line_number + 1,
                column: 0,
            },
            is_fallback: false,
        });

        (tokens, errors)
    }
}

fn match_rules<'a>(
    buffer: &String,
    rules: &'a Vec<LexerRegexGrammarRule>,
    empty_rule: &'a LexerRegexGrammarRule,
) -> (&'a LexerRegexGrammarRule, usize) {
    let mut max_rule = empty_rule;
    // size in bytes for matched string
    let mut max_len = 0;

    for rule in rules {
        let find_result = rule.regex.find(&buffer);
        if find_result.is_some() && find_result.unwrap().as_str().len() > max_len {
            max_len = find_result.unwrap().end();
            max_rule = rule;
        }
    }

    (max_rule, max_len)
}

pub fn create_regex_lexer_from_grammar(grammar_filename: &String) -> Result<RegexLexer, Error> {
    let filename = String::from(grammar_filename);
    let (grammar, alphabet) = read_raw_lexer_grammar(&filename)?;
    let regex_grammar = process_grammar(grammar);

    Ok(RegexLexer {
        grammar: regex_grammar,
        alphabet,
    })
}
