use std::collections::HashSet;
use std::fs::File;
use std::io::{Error, Read};
use crate::frontend_v0::lexer::common::lexer::EOF;

pub struct LexerRawGrammar {
    pub rules: Vec<LexerRawGrammarRule>,
}

pub struct LexerRawGrammarRule {
    pub name: String,
    pub expression: String,
    pub is_fallback: bool,
}

const FALLBACK_MODIFIER: &str = "#FALLBACK ";

pub fn read_raw_lexer_grammar(
    filename: &String,
) -> Result<(LexerRawGrammar, HashSet<String>), Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let rules = buffer
        .split("\r\n")
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            let split = x.split(" = ").collect::<Vec<&str>>();
            let is_fallback = split[0].starts_with(FALLBACK_MODIFIER);
            LexerRawGrammarRule {
                name: split[0]
                    .strip_prefix(FALLBACK_MODIFIER)
                    .get_or_insert(split[0])
                    .to_string(),
                expression: String::from(split[1]),
                is_fallback,
            }
        })
        .collect::<Vec<LexerRawGrammarRule>>();

    let mut alphabet = rules
        .iter()
        .map(|x| x.name.clone())
        .collect::<HashSet<String>>();
    alphabet.insert(String::from(EOF));

    Ok((LexerRawGrammar { rules }, alphabet))
}
