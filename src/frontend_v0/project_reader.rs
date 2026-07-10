use crate::frontend_v0::ast_builder::nodes::{CodeNode, ExpressionNode, StatementNode};
use crate::frontend_v0::errors::ast_errors::AstBuilderError;
use crate::frontend_v0::errors::ast_errors::ErrorBuilder;
use crate::frontend_v0::file_analyzer::{FileAnalyzer, FileAnalyzerError};
use crate::frontend_v0::message_controller::MessageController;
use std::collections::{HashMap, VecDeque};

struct ProjectReader {
    file_analyzer: FileAnalyzer,
    message_controller: MessageController,
}

impl ProjectReader {
    pub fn new() -> ProjectReader {
        ProjectReader {
            file_analyzer: FileAnalyzer::new(),
            message_controller: MessageController::new(),
        }
    }
    pub fn read_project(
        &mut self,
        main_path: String,
    ) -> HashMap<String, CodeNode> {
        let mut file_cache = HashMap::new();

        let mut queue = VecDeque::new();
        queue.push_back(main_path);

        while let Some(path) = queue.pop_front() {
            let ast_result = self.file_analyzer.build_ast_from_file(path.clone());

            match ast_result {
                Ok(ast) => {
                    let imports = ast
                        .imports_list
                        .clone()
                        .into_iter()
                        .map(get_imported_file_path)
                        .collect::<Vec<_>>();

                    let mut errors = Vec::new();

                    for import in imports {
                        match import {
                            Ok(path) => queue.push_back(path),
                            Err(error) => {
                                errors.push(error);
                            }
                        }
                    }
                    // TODO more error control
                    // FileAnalyzerError::AstBuilderErrors(errors);

                    file_cache.insert(path, Ok(ast));
                }
                Err(error) => {
                    file_cache.insert(path, Err(error));
                }
            }
        }

        file_cache
    }
}

fn get_imported_file_path(node: StatementNode) -> Result<String, AstBuilderError> {
    if let StatementNode::ImportStatement {
        imported_file_name, ..
    } = node
    {
        if let ExpressionNode::StringLiteral { value, .. } = imported_file_name {
            Ok(value.to_string())
        } else {
            ErrorBuilder::unexpected_syntax_node(
                "StringLiteral",
                imported_file_name.name(),
                imported_file_name.span().0,
            )
        }
    } else {
        ErrorBuilder::unexpected_syntax_node("StringLiteral", node.name(), node.span().0)
    }
}
