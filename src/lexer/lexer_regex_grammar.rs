use crate::lexer::common::lexer_raw_grammar::{LexerRawGrammar, LexerRawGrammarRule};
use regex::Regex;

pub struct LexerRegexGrammar {
    pub rules: Vec<LexerRegexGrammarRule>,
    pub fallback_rules: Vec<LexerRegexGrammarRule>,
}

pub struct LexerRegexGrammarRule {
    pub name: String,
    pub regex: Regex,
}

pub fn process_grammar(grammar: LexerRawGrammar) -> LexerRegexGrammar {
    LexerRegexGrammar {
        rules: grammar
            .rules
            .iter()
            .filter(|x| !x.is_fallback)
            .map(process_rule)
            .collect(),
        fallback_rules: grammar
            .rules
            .iter()
            .filter(|x| x.is_fallback)
            .map(process_rule)
            .collect(),
    }
}

fn process_rule(rule: &LexerRawGrammarRule) -> LexerRegexGrammarRule {
    let expr = Regex::new(format!("^({})", rule.expression).as_str());
    LexerRegexGrammarRule {
        name: rule.name.escape_default().to_string(),
        regex: expr.unwrap(),
    }
}
