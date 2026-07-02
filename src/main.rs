mod lexer;
pub mod macros;
pub mod parser;

use crate::lexer::common::lexer::Lexer;

fn main() {
    let a = Vec::from([1,2,3]);
    let b = Vec::from([1,2,3]);
    println!("{}", a == b)

    // let filename = String::from("grammars/fuss_v0.fusslex");
    // let lexer = create_regex_lexer_from_grammar(&filename).unwrap();
    //
    // let (tokens, errors) = lexer.tokenize(
    //     &"tokens.fuss".to_string(),
    //     "func main() {\r\n let a = 7 ;\r\n let b = \"The result is \\\"; \r\nreturn b+a;\r\n}".to_string(),
    // );
    //
    // let token_list = tokens
    //     .into_iter()
    //     .map(|x| x.is_fallback.to_string())
    //     .collect::<Vec<String>>()
    //     .join::<_>(&String::from(", "));
    // let error_list = errors
    //     .into_iter()
    //     .map(|x| x.get_message())
    //     .collect::<Vec<String>>()
    //     .join::<_>(&String::from(", "));
    //
    // println!("Tokens: {token_list}",);
    // println!("Errors: {error_list}",);
}
