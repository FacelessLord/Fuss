pub mod collections;
pub mod frontend_v0;
pub mod macros;

use crate::frontend_v0::file_analyzer::FileAnalyzer;

fn main() {
    let filename = String::from("grammars/fibb.fuss");
    let mut file_analyzer = FileAnalyzer::new();
    let ast = file_analyzer.build_ast_from_file(filename);

    ast.unwrap();

    //
    // let token_list = tokens
    //     .into_iter()
    //     .map(|x| x.is_fallback.to_string())
    //     .collect::<Vec<String>>()
    //     .join::<_>(&String::from(", "));
    // println!("Tokens: {token_list}",);
}
