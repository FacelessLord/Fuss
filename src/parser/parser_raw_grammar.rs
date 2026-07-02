use std::fs::File;
use std::io::{Error, Read};

pub struct ParserRawGrammar {
    pub rules: Vec<ParserRawGrammarRule>,
}

pub struct ParserRawGrammarRule {
    pub name: String,
    pub production: Vec<String>,
}

pub fn read_raw_parser_grammar(filename: &String) -> Result<ParserRawGrammar, Error> {
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
                production: split[1]
                    .split(" ")
                    .map(|x| String::from(x))
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<ParserRawGrammarRule>>();

    Ok(ParserRawGrammar { rules })
}
