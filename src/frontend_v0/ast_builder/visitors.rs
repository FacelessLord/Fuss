use crate::frontend_v0::ast_builder::nodes::{
    AccessModifier, ClassMemberNode, CodeNode, ExpressionNode, StatementNode,
};
use crate::frontend_v0::errors::ast_errors::{AstBuilderError, ErrorBuilder};
use crate::frontend_v0::parser::common::parser::{join_spans, ParserNode, Span};
use std::cmp::min;

pub struct AstBuilder {}

impl AstBuilder {
    const EMPTY_EXPR_KIND: &str = "empty_expr";

    pub fn visit_code(&self, node: ParserNode) -> Result<CodeNode, AstBuilderError> {
        match node {
            ParserNode::NonTerminal {
                kind,
                mut children,
                span,
            } => {
                if kind != "code" {
                    return ErrorBuilder::unexpected_token_kind("code", kind.as_str(), span.0);
                }
                let stmt_tree = children.remove(0);
                let statement_list = self.visit_stmt_list(stmt_tree)?;
                let mut code = Vec::new();
                let mut imports = Vec::new();

                for stmt in statement_list {
                    match stmt {
                        import_stmt @ StatementNode::ImportStatement { .. } => {
                            imports.push(import_stmt);
                        }
                        other_stmt => code.push(other_stmt),
                    }
                }

                Ok(CodeNode {
                    imports_list: imports,
                    statement_list: code,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("code", token),
        }
    }
    fn visit_stmt_list(&self, node: ParserNode) -> Result<Vec<StatementNode>, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "stmt_list");
        let results = self
            .unwrap_left_recursive_tree(node, "stmt_list".to_string())?
            .into_iter()
            .map(|x| self.visit_stmt(x))
            .collect::<Vec<_>>();

        first_err(results)
    }

    fn visit_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "stmt");
        match node {
            ParserNode::NonTerminal { mut children, .. } => {
                let inner_statement_node = children.remove(0);
                match inner_statement_node.get_node_kind().as_str() {
                    "import_stmt" => self.visit_import_stmt(inner_statement_node),
                    "class_def" => self.visit_class_def(inner_statement_node),
                    "let_def" => self.visit_let_def(inner_statement_node),
                    "func_def" => self.visit_func_def(inner_statement_node),
                    "if_def" => self.visit_if_def(inner_statement_node),
                    "while_stmt" => self.visit_while_stmt(inner_statement_node),
                    "for_stmt" => self.visit_for_stmt(inner_statement_node),
                    "scope_def" => self.visit_scope_def(inner_statement_node),
                    "return_stmt" => self.visit_return_stmt(inner_statement_node),
                    "postfix_expr" => self.visit_postfix_stmt_expr(inner_statement_node),
                    "assign_stmt" => self.visit_assign_stmt(inner_statement_node),
                    "EXTERN" => self.visit_extern_stmt(inner_statement_node),
                    kind => ErrorBuilder::unexpected_token_kind(
                        "something_def or return_stmt or postfix_expr",
                        kind,
                        inner_statement_node.get_node_start(),
                    ),
                }
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("stmt", token),
        }
    }

    fn visit_extern_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "EXTERN");
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("EXTERN", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(StatementNode::ExternStatement {
                span: (token.position.clone(), token.get_end_position()),
            }),
        }
    }

    fn visit_import_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "import_stmt");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //IMPORT STRING
                let file_path = self.visit_string_literal(children.remove(1))?;
                Ok(StatementNode::ImportStatement {
                    imported_file_name: file_path,
                    span,
                })
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("import_stmt", token)
            }
        }
    }

    fn visit_class_def(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "class_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                self.handle_optional_node(&mut children, 0, "access_modifier".to_string());
                self.handle_optional_node(&mut children, 1, "STATIC".to_string());

                // access_modifier? STATIC? CLASS IDENTIFIER class_scope
                let access_modifier_node = children.remove(0);
                // STATIC? CLASS IDENTIFIER class_scope
                let static_modifier_node = children.remove(0);
                // CLASS IDENTIFIER class_scope
                let identifier_node = children.remove(1);
                // CLASS class_scope
                let class_scope_node = children.remove(1);

                let access_modifier = self
                    .visit_empty_expr_or(access_modifier_node, |ast_builder, x| {
                        ast_builder.visit_access_modifier(x)
                    })?
                    .unwrap_or(AccessModifier::Private);
                let is_static = self
                    .visit_empty_expr_or(static_modifier_node, |ast_builder, x| {
                        ast_builder.visit_static_modifier(x)
                    })?
                    .unwrap_or(false);

                let class_name = self.visit_variable_name(identifier_node)?;

                let class_body = self.visit_class_scope(class_scope_node)?;

                Ok(StatementNode::ClassDefStatement {
                    class_name,
                    access_modifier,
                    is_static,
                    body: class_body,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("class_def", token),
        }
    }

    fn visit_access_modifier(&self, node: ParserNode) -> Result<AccessModifier, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "access_modifier");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                // PUBLIC
                // PROTECTED
                // PRIVATE
                let modifier = children.remove(0);
                match modifier {
                    ParserNode::NonTerminal { kind, .. } => ErrorBuilder::terminal_expected(
                        "PUBLIC or PROTECTED or PRIVATE",
                        kind,
                        span.0,
                    ),
                    ParserNode::Terminal(token) => match token.kind.as_str() {
                        "PUBLIC" => Ok(AccessModifier::Public),
                        "PROTECTED" => Ok(AccessModifier::Protected),
                        "PRIVATE" => Ok(AccessModifier::Private),
                        given_kind => ErrorBuilder::unexpected_token_kind(
                            "PUBLIC or PROTECTED or PRIVATE",
                            given_kind,
                            span.0,
                        ),
                    },
                }
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("access_modifier", token)
            }
        }
    }
    fn visit_static_modifier(&self, node: ParserNode) -> Result<bool, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "STATIC");
        match node {
            ParserNode::NonTerminal { span, kind, .. } => {
                ErrorBuilder::terminal_expected("STATIC", kind, span.0)
            }
            ParserNode::Terminal(token) => {
                if token.kind.as_str() == "STATIC" {
                    return Ok(true);
                }

                ErrorBuilder::unexpected_token_kind(
                    "STATIC",
                    token.kind.as_str(),
                    token.position.clone(),
                )
            }
        }
    }

    fn visit_class_scope(&self, node: ParserNode) -> Result<Vec<ClassMemberNode>, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "class_scope");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                // OPEN_BRACE class_members_list? CLOSED_BRACE
                self.handle_optional_node(&mut children, 1, "class_members_list".to_string());
                // OPEN_BRACE class_members_list CLOSED_BRACE
                let class_member_list_node = children.remove(1);
                Ok(self
                    .visit_empty_expr_or(class_member_list_node, |ast_builder, x| {
                        ast_builder.visit_class_members_list(x)
                    })?
                    .unwrap_or(Vec::new()))
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("class_scope", token)
            }
        }
    }

    fn visit_class_members_list(
        &self,
        node: ParserNode,
    ) -> Result<Vec<ClassMemberNode>, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "class_members_list");
        first_err(
            self.unwrap_left_recursive_tree(node, "class_members_list".to_string())?
                .into_iter()
                .map(|x| self.visit_class_member(x))
                .collect::<Vec<_>>(),
        )
    }
    fn visit_class_member(&self, node: ParserNode) -> Result<ClassMemberNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "class_member");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                self.handle_optional_node(&mut children, 0, "access_modifier".to_string());
                self.handle_optional_node(&mut children, 1, "STATIC".to_string());

                //access_modifier? STATIC? IDENTIFIER ...
                let access_modifier_node = children.remove(0);
                //STATIC? IDENTIFIER ...
                let static_modifier_node = children.remove(0);
                //IDENTIFIER ...
                let identifier_node = children.remove(0);
                //SET expr SEMICOLON
                //SEMICOLON
                //OPEN_PAREN

                let access_modifier = self
                    .visit_empty_expr_or(access_modifier_node, |ast_builder, x| {
                        ast_builder.visit_access_modifier(x)
                    })?
                    .unwrap_or(AccessModifier::Private);
                let is_static = self
                    .visit_empty_expr_or(static_modifier_node, |ast_builder, x| {
                        ast_builder.visit_static_modifier(x)
                    })?
                    .unwrap_or(false);

                if identifier_node.get_node_kind() == "CLASS" {
                    // IDENTIFIER class_scope
                    let identifier_node = children.remove(0);
                    let name = self.visit_variable_name(identifier_node)?;
                    let class_scope = self.visit_class_scope(children.remove(0))?;
                    return Ok(ClassMemberNode::Class {
                        access_modifier,
                        is_static,
                        name,
                        body: class_scope,
                        span,
                    });
                }
                if identifier_node.get_node_kind() == "CONSTRUCTOR" {
                    // OPEN_PAREN args_list? CLOSED_PAREN scope_def
                    self.handle_optional_node(&mut children, 1, "args_list".to_string());

                    // OPEN_PAREN args_list CLOSED_PAREN scope_def
                    let arguments_node = children.remove(1);
                    let arguments = self
                        .visit_empty_expr_or(arguments_node, |ast_builder, x| {
                            ast_builder.visit_args_list(x)
                        })?
                        .unwrap_or(Vec::new());

                    // OPEN_PAREN CLOSED_PAREN scope_def
                    let constructor_body = self.handle_scope_node(children.remove(2))?;
                    return Ok(ClassMemberNode::Constructor {
                        access_modifier,
                        is_static,
                        arguments,
                        body: constructor_body,
                        span,
                    });
                }

                let name = self.visit_variable_name(identifier_node)?;

                match children[0].get_node_kind().as_str() {
                    "SET" => {
                        // SET expr SEMICOLON
                        let value_node = children.remove(1);
                        let value = self.visit_expr(value_node)?;
                        Ok(ClassMemberNode::Field {
                            name,
                            value: Some(value),
                            access_modifier,
                            is_static,
                            span,
                        })
                    }
                    "SEMICOLON" => Ok(ClassMemberNode::Field {
                        name,
                        value: None,
                        access_modifier,
                        is_static,
                        span,
                    }),
                    "OPEN_PAREN" => {
                        // OPEN_PAREN args_list? CLOSED_PAREN scope_def
                        self.handle_optional_node(&mut children, 1, "args_list".to_string());
                        // OPEN_PAREN args_list CLOSED_PAREN scope_def
                        let args_node = children.remove(1);
                        // OPEN_PAREN CLOSED_PAREN scope_def
                        let scope_node = children.remove(2);

                        let arguments = self
                            .visit_empty_expr_or(args_node, |ast_builder, x| {
                                ast_builder.visit_args_list(x)
                            })?
                            .unwrap_or(Vec::new());
                        let body = self.handle_scope_node(scope_node)?;

                        Ok(ClassMemberNode::Method {
                            access_modifier,
                            is_static,
                            name,
                            arguments,
                            body,
                            span,
                        })
                    }
                    given_kind => ErrorBuilder::unexpected_token_kind(
                        "STATIC",
                        given_kind,
                        children[0].get_node_start(),
                    ),
                }
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("class_member", token)
            }
        }
    }

    fn visit_let_def(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "let_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                let variable_node = children.remove(1);
                let expr_node = children.remove(2);

                let variable = self.visit_variable_name(variable_node)?;
                let expr = self.visit_expr(expr_node)?;

                Ok(StatementNode::LetStatement {
                    var_name: variable,
                    value: expr,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("let_def", token),
        }
    }
    fn visit_func_def(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "func_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //FUNC IDENTIFIER OPEN_PAREN args_list CLOSED_PAREN scope_def
                let func_name_node = children.remove(1);

                let args_list_node;
                //FUNC OPEN_PAREN args_list? CLOSED_PAREN scope_def
                if children[2].get_node_kind() == "args_list" {
                    args_list_node = Some(children.remove(2));
                } else {
                    args_list_node = None;
                }
                //FUNC OPEN_PAREN CLOSED_PAREN scope_def
                let body_node = children.remove(3);

                let func_name = self.visit_variable_name(func_name_node)?;
                let body = self.handle_scope_node(body_node)?;

                let arguments = args_list_node
                    .map(|x| self.visit_args_list(x))
                    .unwrap_or(Ok(Vec::new()))?;

                Ok(StatementNode::FunctionDefStatement {
                    func_name,
                    body,
                    arguments,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("func_def", token),
        }
    }
    fn visit_args_list(&self, node: ParserNode) -> Result<Vec<String>, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "args_list");
        let result_list = self
            .unwrap_left_recursive_tree(node, "args_list".to_string())?
            .into_iter()
            .map(|x| match x {
                ParserNode::NonTerminal { kind, span, .. } => {
                    ErrorBuilder::terminal_expected("IDENTIFIER", kind, span.0)
                }
                ParserNode::Terminal(token) => Ok(token.text),
            })
            .collect::<Vec<_>>();
        first_err(result_list)
    }
    fn visit_if_def(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "if_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //IF OPEN_PAREN expr CLOSED_PAREN scope_def (ELSE scope_def)?
                let condition_node = children.remove(2);

                //IF OPEN_PAREN CLOSED_PAREN scope_def (ELSE scope_def)?
                let then_node = children.remove(3);

                //IF OPEN_PAREN CLOSED_PAREN (ELSE scope_def)?
                let else_node = match children.len() > 3 {
                    true => Some(children.remove(4)),
                    false => None,
                };

                let condition = self.visit_expr(condition_node)?;
                let then_branch = self.handle_scope_node(then_node)?;

                let else_branch = match else_node {
                    None => None,
                    Some(node) => Some(self.handle_scope_node(node)?),
                };

                Ok(StatementNode::IfStatement {
                    condition,
                    then_branch,
                    else_branch,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("if_def", token),
        }
    }
    fn visit_while_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "while_stmt");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //WHILE OPEN_PAREN expr CLOSED_PAREN scope_def
                let condition_node = children.remove(2);
                //IF OPEN_PAREN CLOSED_PAREN scope_def (ELSE scope_def)?
                let body_node = children.remove(3);

                let condition = self.visit_expr(condition_node)?;
                let body = self.handle_scope_node(body_node)?;

                Ok(StatementNode::WhileStatement {
                    condition,
                    body,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("while_stmt", token),
        }
    }
    fn visit_for_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "for_stmt");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                self.handle_optional_node(&mut children, 2, "let_def".to_string());
                self.handle_optional_node(&mut children, 4, "expr".to_string());
                self.handle_optional_node(&mut children, 6, "expr".to_string());

                let let_def_node = children.remove(2);
                //FOR OPEN_PAREN SEMICOLON expr SEMICOLON expr CLOSED_PAREN scope_def
                let condition_expr_node = children.remove(3);
                //FOR OPEN_PAREN SEMICOLON SEMICOLON expr CLOSED_PAREN scope_def
                let update_expr_node = children.remove(4);
                //FOR OPEN_PAREN SEMICOLON SEMICOLON CLOSED_PAREN scope_def
                let scope_def = children.remove(5);

                let indexer_def = self
                    .visit_empty_expr_or(let_def_node, |ast_builder, x| {
                        ast_builder.visit_let_def(x)
                    })?
                    .map(|x| Box::from(x));

                let condition = self
                    .visit_empty_expr_or(condition_expr_node, |ast_builder, x| {
                        ast_builder.visit_expr(x)
                    })?
                    .map(|x| Box::from(x));
                let update_expr = self
                    .visit_empty_expr_or(update_expr_node, |ast_builder, x| {
                        ast_builder.visit_expr(x)
                    })?
                    .map(|x| Box::from(x));
                let body = self.handle_scope_node(scope_def)?;

                Ok(StatementNode::ForStatement {
                    indexer_def,
                    condition,
                    update_expr,
                    body,
                    span,
                })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("while_stmt", token),
        }
    }
    fn visit_scope_def(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "scope_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //OPEN_BRACE stmt_list? CLOSED_BRACE
                self.handle_optional_node(&mut children, 1, "stmt_list".to_string());
                //OPEN_BRACE stmt_list? CLOSED_BRACE
                let stmt_list_node = children.remove(1);

                let body = self
                    .visit_empty_expr_or(stmt_list_node, |ast_builder, x| {
                        ast_builder.visit_stmt_list(x)
                    })?
                    .unwrap_or(Vec::new());

                Ok(StatementNode::ScopeStatement { body, span })
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("scope_def", token),
        }
    }
    fn visit_return_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "return_stmt");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //RETURN expr
                let returned_expr_node = children.remove(1);

                let returned_expr = self.visit_expr(returned_expr_node)?;

                Ok(StatementNode::ReturnStatement {
                    value: returned_expr,
                    span,
                })
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("return_stmt", token)
            }
        }
    }
    fn visit_postfix_stmt_expr(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "postfix_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                match children.len() {
                    2 => {
                        let expr = children.remove(0);
                        let operation = children.remove(0);
                        match operation.get_node_kind().as_str() {
                            "INCREMENT" => Ok(StatementNode::IncrementStatement {
                                source: self.visit_postfix_expr(expr)?,
                                span,
                            }),
                            "DECREMENT" => Ok(StatementNode::DecrementStatement {
                                source: self.visit_postfix_expr(expr)?,
                                span,
                            }),
                            node_kind => ErrorBuilder::unexpected_token_kind(
                                "INCREMENT or DECREMENT",
                                node_kind,
                                children[1].get_node_start(),
                            ),
                        }
                    }
                    3 => {
                        let expr_kind = children[1].get_node_kind();
                        if expr_kind == "DOT" {
                            //postfix_expr DOT IDENTIFIER
                            let source = self.visit_postfix_expr(children.remove(0))?;
                            //DOT IDENTIFIER
                            let symbol = self.visit_variable_name(children.remove(1))?;

                            return Ok(StatementNode::SymbolAccessStatement {
                                source,
                                symbol,
                                span,
                            });
                        } else if expr_kind == "OPEN_PAREN" {
                            //postfix_expr OPEN_PAREN CLOSED_PAREN
                            let source = self.visit_postfix_expr(children.remove(0))?;

                            return Ok(StatementNode::FunctionCallStatement {
                                source,
                                arguments: Vec::new(),
                                span,
                            });
                        }
                        ErrorBuilder::unexpected_token_kind(
                            "DOT or OPEN_PAREN",
                            expr_kind.as_str(),
                            children[1].get_node_start(),
                        )
                    }
                    4 => {
                        //postfix_expr OPEN_PAREN expression_list CLOSED_PAREN
                        let source = self.visit_postfix_expr(children.remove(0))?;

                        //OPEN_PAREN expression_list CLOSED_PAREN
                        //OPEN_BRACKET expression_list CLOSED_BRACKET
                        let args_node = children.remove(1);
                        let args_list = self.visit_expr_list(args_node)?;

                        let expr_kind = children[0].get_node_kind();
                        if expr_kind == "OPEN_PAREN" {
                            return Ok(StatementNode::FunctionCallStatement {
                                source,
                                arguments: args_list,
                                span,
                            });
                        } else if expr_kind == "OPEN_BRACKET" {
                            return Ok(StatementNode::ArrayAccessStatement {
                                source,
                                index: args_list,
                                span,
                            });
                        }
                        ErrorBuilder::unexpected_token_kind(
                            "OPEN_PAREN or OPEN_BRACKET",
                            expr_kind.as_str(),
                            children[0].get_node_start(),
                        )
                    }
                    default => ErrorBuilder::unknown_rule_for("postfix_expr stmt", default, span.0),
                }
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("postfix_expr stmt", token)
            }
        }
    }
    fn visit_assign_stmt(&self, node: ParserNode) -> Result<StatementNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "assign_stmt");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                //postfix_expr SET expr
                let postfix_node = self.visit_postfix_expr(children.remove(0))?;
                //SET expr
                let value_node = self.visit_expr(children.remove(1))?;

                Ok(StatementNode::AssignmentStatement {
                    left_side: postfix_node,
                    right_side: value_node,
                    span,
                })
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("assign_stmt", token)
            }
        }
    }
    fn visit_expr_list(&self, node: ParserNode) -> Result<Vec<ExpressionNode>, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "expression_list");
        first_err(
            self.unwrap_left_recursive_tree(node, "expression_list".to_string())?
                .into_iter()
                .map(|x| self.visit_expr(x))
                .collect::<Vec<_>>(),
        )
    }

    fn visit_literal(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "literal");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                let item = children.remove(0);
                match item.get_node_kind().as_str() {
                    "NUMBER" => self.visit_number_literal(item),
                    "BOOLEAN" => self.visit_boolean_literal(item),
                    "STRING" => self.visit_string_literal(item),
                    default => ErrorBuilder::unexpected_token_kind(
                        "NUMBER or BOOLEAN or STRING",
                        default,
                        span.0,
                    ),
                }
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("literal", token),
        }
    }

    fn visit_string_literal(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "STRING");
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("STRING", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(ExpressionNode::StringLiteral {
                value: token.text.clone(),
                span: (token.position.clone(), token.get_end_position()),
            }),
        }
    }
    fn visit_boolean_literal(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "BOOLEAN");
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("BOOLEAN", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(ExpressionNode::BooleanLiteral {
                value: token.text.clone().parse::<bool>().unwrap(),
                span: (token.position.clone(), token.get_end_position()),
            }),
        }
    }
    fn visit_number_literal(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "NUMBER");
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("NUMBER", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(ExpressionNode::NumberLiteral {
                value: token.text.clone().parse::<i32>().unwrap(),
                span: (token.position.clone(), token.get_end_position()),
            }),
        }
    }

    fn visit_variable(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("IDENTIFIER", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(ExpressionNode::Variable {
                name: token.text.clone(),
                span: (token.position.clone(), token.get_end_position()),
            }),
        }
    }
    fn visit_variable_name(&self, node: ParserNode) -> Result<String, AstBuilderError> {
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("IDENTIFIER", kind, span.0)
            }
            ParserNode::Terminal(token) => Ok(token.text),
        }
    }
    fn visit_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "expr");
        match node {
            ParserNode::NonTerminal { mut children, .. } => self.visit_and_expr(children.remove(0)),
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("expr", token),
        }
    }
    fn visit_and_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "and_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_or_expr(children.remove(0));
                }
                // and_expr WIDE_AND or_expr
                // and_expr AND or_expr
                let left_arg = self.visit_and_expr(children.remove(0))?;
                // WIDE_AND or_expr
                // AND or_expr
                let right_arg = self.visit_or_expr(children.remove(1))?;
                let expr_kind = children[0].get_node_kind();
                if expr_kind == "WIDE_AND" {
                    return Ok(ExpressionNode::WideAndExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "AND" {
                    return Ok(ExpressionNode::AndExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }

                ErrorBuilder::unexpected_token_kind(
                    "WIDE_AND or AND",
                    expr_kind.as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("and_expr", token),
        }
    }
    fn visit_or_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "or_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_comparison_expr(children.remove(0));
                }
                // and_expr WIDE_OR or_expr
                // and_expr OR or_expr
                // and_expr XOR or_expr
                let left_arg = self.visit_or_expr(children.remove(0))?;
                // WIDE_AND or_expr
                // AND or_expr
                // XOR or_expr
                let right_arg = self.visit_comparison_expr(children.remove(1))?;
                let expr_kind = children[0].get_node_kind();
                if expr_kind == "WIDE_OR" {
                    return Ok(ExpressionNode::WideOrExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "OR" {
                    return Ok(ExpressionNode::OrExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "XOR" {
                    return Ok(ExpressionNode::XorExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }

                ErrorBuilder::unexpected_token_kind(
                    "WIDE_OR or OR or XOR",
                    expr_kind.as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("or_expr", token),
        }
    }
    fn visit_comparison_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "comparison_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_addition_expr(children.remove(0));
                }
                // and_expr WIDE_OR or_expr
                // and_expr OR or_expr
                // and_expr XOR or_expr
                let left_arg = self.visit_addition_expr(children.remove(0))?;
                // WIDE_AND or_expr
                // AND or_expr
                // XOR or_expr
                let right_arg = self.visit_addition_expr(children.remove(1))?;

                let expr_kind = children[0].get_node_kind();
                if expr_kind == "EQUAL_COMP" {
                    return Ok(ExpressionNode::IsEqualComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "NOT_EQUAL_COMP" {
                    return Ok(ExpressionNode::NotEqualComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "GT_COMP" {
                    return Ok(ExpressionNode::GreaterComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "GT_EQUAL_COMP" {
                    return Ok(ExpressionNode::GreaterOrEqualComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "LT_COMP" {
                    return Ok(ExpressionNode::LessComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "LT_EQUAL_COMP" {
                    return Ok(ExpressionNode::LessOrEqualComparison {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }

                ErrorBuilder::unexpected_token_kind(
                    "comparison operators",
                    expr_kind.as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("comparison_expr", token)
            }
        }
    }
    fn visit_addition_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "addition_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_multiplication_expr(children.remove(0));
                }
                // addition_expr PLUS multiplication_expr
                // addition_expr MINUS multiplication_expr
                let left_arg = self.visit_addition_expr(children.remove(0))?;
                // PLUS multiplication_expr
                // MINUS multiplication_expr
                let right_arg = self.visit_multiplication_expr(children.remove(1))?;
                let expr_kind = children[0].get_node_kind();
                if expr_kind == "PLUS" {
                    return Ok(ExpressionNode::AddExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "MINUS" {
                    return Ok(ExpressionNode::SubtractExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }

                ErrorBuilder::unexpected_token_kind(
                    "PLUS or MINUS",
                    expr_kind.as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("addition_expr", token)
            }
        }
    }
    fn visit_multiplication_expr(
        &self,
        node: ParserNode,
    ) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "multiplication_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_unary_expr(children.remove(0));
                }
                // multiplication_expr ASTERISK unary_expr
                // multiplication_expr SLASH unary_expr
                let left_arg = self.visit_multiplication_expr(children.remove(0))?;
                // ASTERISK unary_expr
                // SLASH unary_expr
                let right_arg = self.visit_unary_expr(children.remove(1))?;
                let expr_kind = children[0].get_node_kind();
                if expr_kind == "ASTERISK" {
                    return Ok(ExpressionNode::MultiplyExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                if expr_kind == "SLASH" {
                    return Ok(ExpressionNode::DivideExpression {
                        left: Box::from(left_arg),
                        right: Box::from(right_arg),
                        span,
                    });
                }
                ErrorBuilder::unexpected_token_kind(
                    "ASTERISK or SLASH",
                    expr_kind.as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("multiplication_expr", token)
            }
        }
    }
    fn visit_unary_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "unary_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    return self.visit_postfix_expr(children.remove(0));
                }
                // NOT unary_expr
                // MINUS unary_expr
                let arg = self.visit_unary_expr(children.remove(1))?;
                // NOT
                // MINUS
                // INVERSE
                if children[0].get_node_kind() == "NOT" {
                    return Ok(ExpressionNode::NotExpression {
                        source: Box::from(arg),
                        span,
                    });
                }
                if children[0].get_node_kind() == "MINUS" {
                    return Ok(ExpressionNode::NegateExpression {
                        source: Box::from(arg),
                        span,
                    });
                }
                if children[0].get_node_kind() == "INVERSE" {
                    return Ok(ExpressionNode::InverseExpression {
                        source: Box::from(arg),
                        span,
                    });
                }
                if children[0].get_node_kind() == "NEW" {
                    return self.visit_class_constructor_call(arg, children[0].get_node_span());
                }
                ErrorBuilder::unexpected_token_kind(
                    "NOT or MINUS or INVERSE or NEW",
                    children[0].get_node_kind().as_str(),
                    children[0].get_node_start(),
                )
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("unary_expr", token),
        }
    }

    fn visit_class_constructor_call(
        &self,
        node: ExpressionNode,
        new_keyword_span: Span,
    ) -> Result<ExpressionNode, AstBuilderError> {
        match node {
            ExpressionNode::FunctionCallExpression {
                source,
                arguments,
                span,
            } => Ok(ExpressionNode::NewExpression {
                class: source,
                arguments,
                span: join_spans(new_keyword_span, span),
            }),
            given_node => ErrorBuilder::unexpected_syntax_node(
                "FunctionCallExpression",
                given_node.name(),
                given_node.span().0,
            ),
        }
    }

    fn visit_postfix_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "postfix_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                match children.len() {
                    1 => self.visit_primary_expr(children.remove(0)),
                    2 => {
                        let expr = children.remove(0);
                        let operation = children.remove(0);
                        match operation.get_node_kind().as_str() {
                            "INCREMENT" => Ok(ExpressionNode::IncrementExpression {
                                source: Box::from(self.visit_postfix_expr(expr)?),
                                span,
                            }),
                            "DECREMENT" => Ok(ExpressionNode::DecrementExpression {
                                source: Box::from(self.visit_postfix_expr(expr)?),
                                span,
                            }),
                            node_kind => ErrorBuilder::unexpected_token_kind(
                                "INCREMENT or DECREMENT",
                                node_kind,
                                children[1].get_node_start(),
                            ),
                        }
                    }
                    3 => {
                        if children[1].get_node_kind() == "DOT" {
                            //postfix_expr DOT IDENTIFIER
                            let source = self.visit_postfix_expr(children.remove(0))?;
                            //DOT IDENTIFIER
                            let symbol = self.visit_variable_name(children.remove(1))?;

                            return Ok(ExpressionNode::SymbolAccessExpression {
                                source: Box::from(source),
                                symbol,
                                span,
                            });
                        } else if children[1].get_node_kind() == "OPEN_PAREN" {
                            //postfix_expr OPEN_PAREN CLOSED_PAREN
                            let source = self.visit_postfix_expr(children.remove(0))?;

                            return Ok(ExpressionNode::FunctionCallExpression {
                                source: Box::from(source),
                                arguments: Vec::new(),
                                span,
                            });
                        }
                        ErrorBuilder::unexpected_token_kind(
                            "primary_expr",
                            children[1].get_node_kind().as_str(),
                            span.0,
                        )
                    }
                    4 => {
                        //postfix_expr OPEN_PAREN expression_list CLOSED_PAREN
                        let source = self.visit_postfix_expr(children.remove(0))?;

                        //OPEN_PAREN expression_list CLOSED_PAREN
                        //OPEN_BRACKET expression_list CLOSED_BRACKET
                        let args_node = children.remove(1);
                        let args_list = self.visit_expr_list(args_node)?;

                        if children[0].get_node_kind() == "OPEN_PAREN" {
                            return Ok(ExpressionNode::FunctionCallExpression {
                                source: Box::from(source),
                                arguments: args_list,
                                span,
                            });
                        } else if children[0].get_node_kind() == "OPEN_BRACKET" {
                            return Ok(ExpressionNode::IndexAccessExpression {
                                source: Box::from(source),
                                index: args_list,
                                span,
                            });
                        }
                        ErrorBuilder::unexpected_token_kind(
                            "primary_expr",
                            children[0].get_node_kind().as_str(),
                            span.0,
                        )
                    }
                    default => ErrorBuilder::unknown_rule_for("primary_expr", default, span.0),
                }
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("postfix_expr", token)
            }
        }
    }
    fn visit_primary_expr(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "primary_expr");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 1 {
                    let item = children.remove(0);
                    return match item.get_node_kind().as_str() {
                        "literal" => self.visit_literal(item),
                        "array_def" => self.visit_array_def(item),
                        "IDENTIFIER" => self.visit_variable(item),
                        other => ErrorBuilder::unexpected_token_kind(
                            "literal or array_def or IDENTIFIER",
                            other,
                            span.0,
                        ),
                    };
                } else if children.len() == 3 {
                    let item = children.remove(1);
                    return self.visit_expr(item);
                }
                ErrorBuilder::unknown_rule_for("primary_expr", children.len(), span.0)
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("primary_expr", token)
            }
        }
    }
    fn visit_array_def(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        assert_eq!(node.get_node_kind(), "array_def");
        match node {
            ParserNode::NonTerminal {
                mut children, span, ..
            } => {
                if children.len() == 2 {
                    return Ok(ExpressionNode::ArrayDefinition {
                        elements: Vec::new(),
                        span,
                    });
                }
                if children.len() == 3 {
                    let elements = self.visit_expr_list(children.remove(1))?;
                    return Ok(ExpressionNode::ArrayDefinition { elements, span });
                }
                ErrorBuilder::unknown_rule_for("array_def", children.len(), span.0)
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("array_def", token),
        }
    }

    fn unwrap_left_recursive_tree(
        &self,
        node: ParserNode,
        tree_node_kind: String,
    ) -> Result<Vec<ParserNode>, AstBuilderError> {
        let mut items = Vec::new();
        let mut tree = node;

        while tree.get_node_kind() == tree_node_kind {
            match tree {
                ParserNode::NonTerminal { mut children, .. } => {
                    if children.len() == 1 {
                        let item = children.remove(0);
                        items.push(item);
                        break;
                    }

                    let next_tree_node = children.remove(0);
                    // Taking last item in children, in case there is a separator between items
                    let item = children.remove(children.len() - 1);

                    items.push(item);

                    tree = next_tree_node;
                }
                ParserNode::Terminal(token) => {
                    return ErrorBuilder::non_terminal_expected(tree_node_kind.as_str(), token);
                }
            }
        }
        items.reverse();
        Ok(items)
    }
    fn handle_scope_node(&self, node: ParserNode) -> Result<Vec<StatementNode>, AstBuilderError> {
        let node_kind = node.get_node_kind();
        let node_start = node.get_node_start();
        match self.visit_scope_def(node)? {
            StatementNode::ScopeStatement { body, .. } => Ok(body),
            _ => ErrorBuilder::unexpected_token_kind("scope_def", node_kind.as_str(), node_start),
        }
    }
    fn handle_optional_node(
        &self,
        parent_node_children: &mut Vec<ParserNode>,
        mut child_index: usize,
        expected_node_kind: String,
    ) {
        if child_index >= parent_node_children.len()
            || parent_node_children[child_index].get_node_kind() != expected_node_kind
        {
            child_index = min(child_index, parent_node_children.len() - 1);
            let unexpected_node_span = parent_node_children[child_index].get_node_span();
            parent_node_children.insert(
                child_index,
                ParserNode::NonTerminal {
                    kind: Self::EMPTY_EXPR_KIND.to_string(),
                    children: Vec::new(),
                    span: (
                        unexpected_node_span.0.clone(),
                        unexpected_node_span.0.clone(),
                    ),
                },
            );
        }
    }

    fn handle_empty_expr(&self, node: ParserNode) -> Result<Option<ParserNode>, AstBuilderError> {
        if node.get_node_kind() == Self::EMPTY_EXPR_KIND {
            return Ok(None);
        }
        Ok(Some(node))
    }

    fn visit_empty_expr_or<T>(
        &self,
        node: ParserNode,
        or_fn: fn(&Self, ParserNode) -> Result<T, AstBuilderError>,
    ) -> Result<Option<T>, AstBuilderError> {
        let def_result = self.handle_empty_expr(node)?.map(|x| or_fn(self, x));
        reorder_option_and_result(def_result)
    }
}

fn reorder_option_and_result<T>(
    opt: Option<Result<T, AstBuilderError>>,
) -> Result<Option<T>, AstBuilderError> {
    match opt {
        Some(indexer_def) => Ok(Some(indexer_def?)),
        None => Ok(None),
    }
}

pub fn first_err<T>(mut list: Vec<Result<T, AstBuilderError>>) -> Result<Vec<T>, AstBuilderError> {
    let err_pos = list.iter().position(|x| x.is_err());
    if err_pos.is_some() {
        let error = list.remove(err_pos.unwrap());
        match error {
            Ok(_) => {
                panic!("error is somehow ok")
            }
            Err(ast_builder_error) => return Err(ast_builder_error),
        }
    }
    Ok(list.into_iter().map(Result::unwrap).collect::<Vec<_>>())
}
