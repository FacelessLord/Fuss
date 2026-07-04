mod lexer;
pub mod macros;
pub mod parser;

use log::debug;
use crate::lexer::common::lexer::Lexer;
use crate::lexer::regex_lexer::create_regex_lexer_from_grammar;
use crate::parser::lr1_automata_builder::build_automata_from_grammar;
use crate::parser::lr1_parser::LR1Parser;

fn main() {
    let filename = String::from("grammars/fuss_v0.fusslex");
    let lexer = create_regex_lexer_from_grammar(&filename).unwrap();

    let (tokens, errors) = lexer.tokenize(
        &"tokens.fuss".to_string(),
        "func main() {\r\n let a = 7 ;\r\n let b = \"The result is \\\"; \r\nreturn b+a;\r\n}"
            .to_string(),
    );

    let error_list = errors
        .into_iter()
        .map(|x| x.get_message())
        .collect::<Vec<String>>()
        .join::<_>(&String::from(", "));

    if error_list.len() > 0 {
        panic!("Errors {}", error_list);
    }

    let filename = String::from("grammars/fuss_v0.fussparse");
    let automata = build_automata_from_grammar(&lexer.alphabet, filename).unwrap();
    let mut parser = LR1Parser::new(automata);

    let (parse_tree, errors)  = parser.parse(&tokens);

    debug!("parse_tree: {:?}", parse_tree);
    debug!("errors: {:?}", errors);

    //
    // let token_list = tokens
    //     .into_iter()
    //     .map(|x| x.is_fallback.to_string())
    //     .collect::<Vec<String>>()
    //     .join::<_>(&String::from(", "));
    // println!("Tokens: {token_list}",);
}
