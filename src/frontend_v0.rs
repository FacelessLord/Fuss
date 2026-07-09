use std::fmt::{Debug, Formatter};
use crate::frontend_v0::ast_builder::nodes::CodeNode;
use crate::frontend_v0::ast_builder::visitors::{AstBuilder, AstBuilderError};
use crate::frontend_v0::lexer::common::lexer::{Lexer, LexerError};
use crate::frontend_v0::lexer::regex_lexer::{RegexLexer, create_regex_lexer_from_grammar};
use crate::frontend_v0::parser::lr1_automata_builder::build_lr1_parser;
use crate::frontend_v0::parser::lr1_parser::{LR1Parser, ParserError};
use std::fs::File;
use std::io;
use std::io::Read;

pub mod ast_builder;
pub mod import_resolver;
pub mod lexer;
pub mod parser;

pub struct FileAnalyzer {
    lexer: RegexLexer,
    parser: LR1Parser,
    ast_builder: AstBuilder,
}

pub enum FileAnalyzerError {
    IoError(io::Error),
    LexerErrors(Vec<LexerError>),
    ParserErrors(Vec<ParserError>),
    AstBuilderErrors(Vec<AstBuilderError>),
}

// TODO normal error formatting
impl Debug for FileAnalyzerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileAnalyzerError::IoError(err) => {err.fmt(f)}
            FileAnalyzerError::LexerErrors(err) => {err.fmt(f)}
            FileAnalyzerError::ParserErrors(err) => {err.fmt(f)}
            FileAnalyzerError::AstBuilderErrors(err) => {err.fmt(f)}
        }
    }
}

impl FileAnalyzer {
    pub fn new() -> Self {
        let lexer_grammar_filename = String::from("grammars/fuss_v0.fusslex");
        let parser_grammar_filename = String::from("grammars/fuss_v0.fussparse");
        let lexer = create_regex_lexer_from_grammar(&lexer_grammar_filename).unwrap();
        let parser = build_lr1_parser(&lexer.alphabet, parser_grammar_filename);

        FileAnalyzer {
            lexer,
            parser,
            ast_builder: AstBuilder {},
        }
    }
    pub fn build_ast_from_file(&mut self, filename: String) -> Result<CodeNode, FileAnalyzerError> {
        let file_text = self
            .read_file(filename.clone())
            .map_err(|e| FileAnalyzerError::IoError(e))?;

        let (tokens, errors) = self.lexer.tokenize(&filename, file_text);
        if errors.len() > 0 {
            return Err(FileAnalyzerError::LexerErrors(errors));
        }
        let tokens = tokens
            .into_iter()
            .filter(|x| x.kind != "COMMENT")
            .collect::<Vec<_>>();

        let (parse_tree, errors) = self.parser.parse(&tokens);
        self.parser.reset();

        if errors.len() > 0 {
            return Err(FileAnalyzerError::ParserErrors(errors));
        }

        self.ast_builder
            .visit_code(parse_tree)
            .map_err(|x| FileAnalyzerError::AstBuilderErrors(Vec::from([x])))
    }

    fn read_file(&self, filename: String) -> Result<String, io::Error> {
        let code_file = File::open(filename.clone());
        let mut code_text = String::new();

        code_file.expect("").read_to_string(&mut code_text)?;

        Ok(code_text)
    }
}
