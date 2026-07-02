// use crate::parser::common::parser::ParserGrammarToken;
// use crate::parser::parser_raw_grammar::ParserRawGrammar;
// use std::collections::HashMap;
// use crate::parser::lr1_automata::build_automata;
// 
// pub struct LR1ParserGrammar<'a> {
//     pub rules: Vec<LR1ParserGrammarRule<'a>>,
//     pub parser_alphabet: Vec<ParserGrammarToken>,
// }
// 
// pub struct LR1ParserGrammarRule<'a> {
//     pub non_terminal: &'a ParserGrammarToken,
//     pub production: Vec<&'a ParserGrammarToken>,
// }
// 
// pub fn get_grammar_alphabet(
//     grammar: &ParserRawGrammar,
//     lexer_alphabet: &Vec<String>,
// ) -> Vec<ParserGrammarToken> {
//     let mut token_cache: HashMap<String, ParserGrammarToken> = HashMap::new();
//     let eof_token = ParserGrammarToken::Terminal("eof".to_string());
//     token_cache.insert("eof".to_string(), eof_token);
// 
//     for token in lexer_alphabet {
//         let terminal = ParserGrammarToken::Terminal(token.clone());
//         token_cache.insert(token.clone(), terminal);
//     }
// 
//     for rule in &grammar.rules {
//         let non_terminal = ParserGrammarToken::NonTerminal(rule.name.clone());
//         token_cache.insert(rule.name.clone(), non_terminal);
//     }
// 
//     token_cache
//         .into_values()
//         .collect::<Vec<ParserGrammarToken>>()
// }
// 
// pub fn process_grammar(
//     grammar: ParserRawGrammar,
//     alphabet: &Vec<ParserGrammarToken>,
// ) -> (&Vec<ParserGrammarToken>, Vec<LR1ParserGrammarRule<'_>>) {
//     let mut processed_rules = Vec::new();
//     let mut token_ref_cache: HashMap<String, &ParserGrammarToken> = HashMap::new();
// 
//     alphabet.iter().for_each(|x| match x {
//         ParserGrammarToken::NonTerminal(name) => {
//             token_ref_cache.insert(name.to_string(), x);
//         }
//         ParserGrammarToken::Terminal(name) => {
//             token_ref_cache.insert(name.to_string(), x);
//         }
//     });
// 
//     for rule in grammar.rules {
//         let mapped_production = rule
//             .production
//             .iter()
//             .map(|x| {
//                 *token_ref_cache
//                     .get(x)
//                     .expect(format!("Had not found definition for {}", x).as_str())
//             })
//             .collect::<Vec<&ParserGrammarToken>>();
//         processed_rules.push(LR1ParserGrammarRule {
//             non_terminal: token_ref_cache.get(&rule.name).unwrap(),
//             production: mapped_production,
//         })
//     }
// 
//     (alphabet, processed_rules)
// }
// 
// fn proc_gram(
//     grammar: ParserRawGrammar,
//     lexer_alphabet: &Vec<String>,
// )  {
//     let alphabet = get_grammar_alphabet(&grammar, lexer_alphabet);
//     let (alphabet, rules) = process_grammar(grammar, &alphabet);
//     build_automata(alphabet, rules);
// }