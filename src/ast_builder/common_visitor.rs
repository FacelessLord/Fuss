// use crate::ast_builder::nodes::ExpressionNode;
// use crate::ast_builder::visitors::*;
// use crate::parser::common::parser::ParserNode;
//
// fn visit(node: ParserNode) {
//     match node {
//         ParserNode::NonTerminal { kind, children, span} => {
//             let node = ParserNode::NonTerminal {
//                 kind: kind.clone(),
//                 children,
//                 span
//             };
//             match kind.as_str() {
//                 "code" => visit_code(node),
//                 "stmt" => visit_stmt(node),
//                 "let_def" => visit_let_def(node),
//                 "func_def" => visit_func_def(node),
//                 "if_def" => visit_if_def(node),
//                 "while_stmt" => visit_while_stmt(node),
//                 "scope_def" => visit_scope_def(node),
//                 "return_stmt" => visit_return_stmt(node),
//                 "postfix_expr" => visit_postfix_expr(node),
//                 "assign_stmt" => visit_assign_stmt(node),
//                 "expr" => visit_expr(node),
//                 "literal" => visit_literal(node),
//                 "and_expr" => visit_and_expr(node),
//                 "or_expr" => visit_or_expr(node),
//                 "comparison_expr" => visit_comparison_expr(node),
//                 "addition_expr" => visit_addition_expr(node),
//                 "multiplication_expr" => visit_multiplication_expr(node),
//                 "unary_expr" => visit_unary_expr(node),
//                 "primary_expr" => visit_primary_expr(node),
//                 "array_def" => visit_array_def(node),
//                 &_ => todo!(),
//             }
//         },
//         ParserNode::Terminal(token) => {
//             match token.kind.as_str() {
//                 "IDENTIFIER" => ExpressionNode::Variable(token.text),
//
//                 _ => todo!(),
//             }
//         }
//     }
// }
