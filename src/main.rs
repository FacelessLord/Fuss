pub mod ast_builder;
mod lexer;
pub mod macros;
pub mod parser;

use crate::ast_builder::visitors::AstBuilder;
use crate::lexer::common::lexer::Lexer;
use crate::lexer::regex_lexer::create_regex_lexer_from_grammar;
use crate::parser::lr1_automata_builder::build_automata_from_grammar;
use crate::parser::lr1_parser::LR1Parser;
use log::debug;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = String::from("grammars/fuss_v0.fusslex");
    let lexer = create_regex_lexer_from_grammar(&filename).unwrap();

    let parser_grammar_filename = String::from("grammars/fuss_v0.fussparse");
    let automata = build_automata_from_grammar(&lexer.alphabet, parser_grammar_filename).unwrap();
    let mut parser = LR1Parser::new(automata);

    let filename = String::from("grammars/fibb.fuss");
    let mut code_file = File::open(filename.clone());
    let mut code_text = String::new();

    code_file.expect("").read_to_string(&mut code_text).unwrap();

    let (tokens, errors) = lexer.tokenize(&filename, code_text.to_string());

    let lexing_error_list = errors
        .into_iter()
        .map(|x| x.get_message())
        .collect::<Vec<String>>()
        .join::<_>(&String::from(", "));

    if lexing_error_list.len() > 0 {
        panic!("Errors {}", lexing_error_list);
    }

    let (parse_tree, errors) = parser.parse(&tokens);

    let parse_error_list = errors
        .into_iter()
        .map(|x| x.get_message())
        .collect::<Vec<String>>()
        .join::<_>(&String::from(",\n"));

    if parse_error_list.len() > 0 {
        panic!("Errors {}", parse_error_list);
    }
    let ast_builder = AstBuilder {};

    let ast = ast_builder.visit_code(parse_tree).unwrap();

    debug!("ast: {:?}", ast.span.0.filename);

    //
    // let token_list = tokens
    //     .into_iter()
    //     .map(|x| x.is_fallback.to_string())
    //     .collect::<Vec<String>>()
    //     .join::<_>(&String::from(", "));
    // println!("Tokens: {token_list}",);
}
