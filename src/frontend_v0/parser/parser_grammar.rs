use std::fs::File;
use std::io::{Error, Read};

pub struct ParserGrammar {
    pub rules: Vec<ParserGrammarRule>,
}

pub struct ParserGrammarRule {
    pub name: String,
    pub production: Vec<String>,
}

struct ParserRawGrammar {
    pub rules: Vec<ParserRawGrammarRule>,
}

struct ParserRawGrammarRule {
    pub name: String,
    pub production: Vec<String>,
}

pub fn read_parser_grammar(filename: &String) -> Result<ParserGrammar, Error> {
    let raw_grammar = read_raw_parser_grammar(filename)?;
    let rules = raw_grammar
        .rules
        .into_iter()
        .flat_map(|x| create_rule_variants(x))
        .collect::<Vec<_>>();
    Ok(ParserGrammar { rules })
}

fn create_rule_variants(rule: ParserRawGrammarRule) -> Vec<ParserGrammarRule> {
    let mut production_variants = Vec::new();
    let optionals_count = rule.production.iter().filter(|x| x.ends_with("?")).count();

    if optionals_count == 0 {
        return Vec::from([ParserGrammarRule {
            name: rule.name,
            production: rule.production,
        }]);
    }

    for i in 0..1 << optionals_count {
        let mut production_variant = Vec::<String>::new();
        let mut optional_index = 0;
        for j in 0..rule.production.len() {
            let item = rule.production[j].clone();
            if item.ends_with("?") {
                if i & (1 << optional_index) > 0 {
                    production_variant.push(item.strip_suffix("?").unwrap().to_string());
                }
                optional_index += 1;
            } else {
                production_variant.push(item);
            }
        }
        production_variants.push(production_variant)
    }

    production_variants
        .into_iter()
        .map(|x| ParserGrammarRule {
            name: rule.name.clone(),
            production: x,
        })
        .collect()
}

fn read_raw_parser_grammar(filename: &String) -> Result<ParserRawGrammar, Error> {
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
