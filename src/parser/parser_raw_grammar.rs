use std::fs::File;
use std::io::{Error, Read};

pub struct ParserRawGrammar {
    pub rules: Vec<ParserRawGrammarRule>,
}

pub struct ParserRawGrammarRule {
    pub name: String,
    pub expression: String,
}

pub fn read_raw_lexer_grammar(filename: &String) -> Result<ParserRawGrammar, Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let rules = buffer
        .split("\r\n")
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            let split = x.split(" = ").collect::<Vec<&str>>();
            ParserRawGrammarRule {
                name: split[0].to_string(),
                expression: String::from(split[1]),
            }
        })
        .collect::<Vec<ParserRawGrammarRule>>();

    Ok(ParserRawGrammar { rules })
}
