use crate::frontend_v0::ast_builder::nodes::{CodeNode, ExpressionNode, StatementNode};
use crate::frontend_v0::lexer::common::lexer::Token;
use crate::frontend_v0::parser::common::parser::{ParserNode, Span};
use std::cmp::min;
use std::fmt::{Debug, Formatter};

pub enum AstBuilderError {
    TerminalExpected {
        expected_node: String,
        given_token_kind: String,
        span: Span,
    },
    NonTerminalExpected {
        expected_node: String,
        given_token: Token,
    },
    UnknownRule {
        expected_node: String,
        given_token_amount: usize,
        span: Span,
    },
    UnexpectedTokenKind {
        expected_node: String,
        given_token_kind: String,
        span: Span,
    },
}

impl Debug for AstBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AstBuilderError::TerminalExpected {
                expected_node,
                given_token_kind,
                span,
            } => {
                write!(
                    f,
                    "Expected terminal {}, but got {} at {}",
                    expected_node, given_token_kind, span.0
                )
            }
            AstBuilderError::NonTerminalExpected {
                expected_node,
                given_token,
            } => {
                write!(
                    f,
                    "Expected non-terminal {}, but got {} at {}",
                    expected_node,
                    given_token.kind.clone(),
                    given_token.position
                )
            }
            AstBuilderError::UnknownRule {
                given_token_amount,
                expected_node,
                span,
            } => {
                write!(
                    f,
                    "Found unknown rule at production size {}, but got {} at {}",
                    given_token_amount, expected_node, span.0
                )
            }
            AstBuilderError::UnexpectedTokenKind {
                expected_node,
                given_token_kind,
                span,
            } => {
                write!(
                    f,
                    "Expected {}, but got {} at {}",
                    expected_node, given_token_kind, span.0
                )
            }
        }
    }
}

// TODO move to separate file
pub struct ErrorBuilder {}

impl ErrorBuilder {
    pub fn terminal_expected<T>(
        expected_node: &str,
        given_token_kind: String,
        span: Span,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::TerminalExpected {
            expected_node: expected_node.to_string(),
            given_token_kind,
            span,
        })
    }
    pub fn non_terminal_expected<T>(
        expected_node: &str,
        given_token: Token,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::NonTerminalExpected {
            expected_node: expected_node.to_string(),
            given_token,
        })
    }
    pub fn unknown_rule_for<T>(
        expected_node: &str,
        given_token_amount: usize,
        span: Span,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::UnknownRule {
            expected_node: expected_node.to_string(),
            given_token_amount,
            span,
        })
    }
    pub fn unexpected_token_kind<T>(
        expected_node: &str,
        given_token_kind: &str,
        span: Span,
    ) -> Result<T, AstBuilderError> {
        Err(AstBuilderError::UnexpectedTokenKind {
            expected_node: expected_node.to_string(),
            given_token_kind: given_token_kind.to_string(),
            span,
        })
    }
}

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
                    return ErrorBuilder::unexpected_token_kind("code", kind.as_str(), span);
                }
                let stmt_tree = children.remove(0);
                let statement_list = self.visit_stmt_list(stmt_tree)?;

                Ok(CodeNode {
                    statement_list,
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
                    "let_def" => self.visit_let_def(inner_statement_node),
                    "func_def" => self.visit_func_def(inner_statement_node),
                    "if_def" => self.visit_if_def(inner_statement_node),
                    "while_stmt" => self.visit_while_stmt(inner_statement_node),
                    "for_stmt" => self.visit_for_stmt(inner_statement_node),
                    "scope_def" => self.visit_scope_def(inner_statement_node),
                    "return_stmt" => self.visit_return_stmt(inner_statement_node),
                    "postfix_expr" => self.visit_postfix_stmt_expr(inner_statement_node),
                    "assign_stmt" => self.visit_assign_stmt(inner_statement_node),
                    kind => ErrorBuilder::unexpected_token_kind(
                        "something_def or return_stmt or postfix_expr",
                        kind,
                        inner_statement_node.get_node_span(),
                    ),
                }
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("stmt", token),
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
                    ErrorBuilder::terminal_expected("IDENTIFIER", kind, span)
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
                //OPEN_BRACE stmt_list CLOSED_BRACE
                let stmt_list_node = children.remove(1);

                let body = self.visit_stmt_list(stmt_list_node)?;

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
                    1 => ErrorBuilder::unknown_rule_for(
                        "postfix_expr stmt",
                        1,
                        children[0].get_node_span(),
                    ),
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
                                children[1].get_node_span(),
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
                            children[1].get_node_span(),
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
                            children[0].get_node_span(),
                        )
                    }
                    default => ErrorBuilder::unknown_rule_for("postfix_expr", default, span),
                }
            }
            ParserNode::Terminal(token) => {
                ErrorBuilder::non_terminal_expected("postfix_expr", token)
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
                kind,
                span,
                mut children,
                ..
            } => {
                let item = children.remove(0);
                match item {
                    ParserNode::NonTerminal { .. } => {
                        ErrorBuilder::terminal_expected("literal", kind, span)
                    }
                    ParserNode::Terminal(token) => match token.kind.as_str() {
                        "NUMBER" => Ok(ExpressionNode::NumberLiteral {
                            value: token.text.clone().parse::<i32>().unwrap(),
                            span: (token.position.clone(), token.get_end_position()),
                        }),
                        "BOOLEAN" => Ok(ExpressionNode::BooleanLiteral {
                            value: token.text.clone().parse::<bool>().unwrap(),
                            span: (token.position.clone(), token.get_end_position()),
                        }),
                        "STRING" => Ok(ExpressionNode::StringLiteral {
                            value: token.text.clone(),
                            span: (token.position.clone(), token.get_end_position()),
                        }),
                        default => ErrorBuilder::unexpected_token_kind(
                            "NUMBER or BOOLEAN or STRING",
                            default,
                            (token.position.clone(), token.get_end_position()),
                        ),
                    },
                }
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("literal", token),
        }
    }
    fn visit_variable(&self, node: ParserNode) -> Result<ExpressionNode, AstBuilderError> {
        match node {
            ParserNode::NonTerminal { kind, span, .. } => {
                ErrorBuilder::terminal_expected("IDENTIFIER", kind, span)
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
                ErrorBuilder::terminal_expected("IDENTIFIER", kind, span)
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
                    children[0].get_node_span(),
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
                    children[0].get_node_span(),
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
                    children[0].get_node_span(),
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
                    children[0].get_node_span(),
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
                    children[0].get_node_span(),
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
                ErrorBuilder::unexpected_token_kind(
                    "NOT or MINUS",
                    children[0].get_node_kind().as_str(),
                    children[0].get_node_span(),
                )
            }
            ParserNode::Terminal(token) => ErrorBuilder::non_terminal_expected("unary_expr", token),
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
                                children[1].get_node_span(),
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
                            span,
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
                            span,
                        )
                    }
                    default => ErrorBuilder::unknown_rule_for("primary_expr", default, span),
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
                            span,
                        ),
                    };
                } else if children.len() == 3 {
                    let item = children.remove(1);
                    return self.visit_expr(item);
                }
                ErrorBuilder::unknown_rule_for("primary_expr", children.len(), span)
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
                ErrorBuilder::unknown_rule_for("array_def", children.len(), span)
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
        let node_span = node.get_node_span();
        match self.visit_scope_def(node)? {
            StatementNode::ScopeStatement { body, .. } => Ok(body),
            _ => ErrorBuilder::unexpected_token_kind("scope_def", node_kind.as_str(), node_span),
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

fn first_err<T>(mut list: Vec<Result<T, AstBuilderError>>) -> Result<Vec<T>, AstBuilderError> {
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
