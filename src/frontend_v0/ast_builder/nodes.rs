use crate::frontend_v0::parser::common::parser::Span;

pub struct CodeNode {
    pub imports_list: Vec<StatementNode>,
    pub statement_list: Vec<StatementNode>,
    pub span: Span,
}

#[derive(Clone)]
pub enum AccessModifier {
    Public,
    Protected,
    Private,
}

#[derive(Clone)]
pub enum ClassMemberNode {
    Field {
        access_modifier: AccessModifier,
        is_static: bool,
        name: String,
        value: Option<ExpressionNode>,
        span: Span,
    },
    Method {
        access_modifier: AccessModifier,
        is_static: bool,
        name: String,
        arguments: Vec<String>,
        body: Vec<StatementNode>,
        span: Span,
    },
    Constructor {
        access_modifier: AccessModifier,
        is_static: bool,
        arguments: Vec<String>,
        body: Vec<StatementNode>,
        span: Span,
    },
    Class {
        access_modifier: AccessModifier,
        is_static: bool,
        name: String,
        body: Vec<ClassMemberNode>,
        span: Span,
    },
}

#[derive(Clone)]
pub enum StatementNode {
    ImportStatement {
        imported_file_name: ExpressionNode,
        span: Span,
    },
    LetStatement {
        var_name: String,
        value: ExpressionNode,
        span: Span,
    },
    FunctionDefStatement {
        func_name: String,
        arguments: Vec<String>,
        body: Vec<StatementNode>,
        span: Span,
    },
    ClassDefStatement {
        class_name: String,
        access_modifier: AccessModifier,
        is_static: bool,
        body: Vec<ClassMemberNode>,
        span: Span,
    },
    IfStatement {
        condition: ExpressionNode,
        then_branch: Vec<StatementNode>,
        else_branch: Option<Vec<StatementNode>>,
        span: Span,
    },
    WhileStatement {
        condition: ExpressionNode,
        body: Vec<StatementNode>,
        span: Span,
    },
    ForStatement {
        indexer_def: Option<Box<StatementNode>>,
        condition: Option<Box<ExpressionNode>>,
        update_expr: Option<Box<ExpressionNode>>,
        body: Vec<StatementNode>,
        span: Span,
    },
    ScopeStatement {
        body: Vec<StatementNode>,
        span: Span,
    },
    ReturnStatement {
        value: ExpressionNode,
        span: Span,
    },
    FunctionCallStatement {
        source: ExpressionNode,
        arguments: Vec<ExpressionNode>,
        span: Span,
    },
    SymbolAccessStatement {
        source: ExpressionNode,
        symbol: String,
        span: Span,
    },
    ArrayAccessStatement {
        source: ExpressionNode,
        index: Vec<ExpressionNode>,
        span: Span,
    },
    IncrementStatement {
        source: ExpressionNode,
        span: Span,
    },
    DecrementStatement {
        source: ExpressionNode,
        span: Span,
    },
    AssignmentStatement {
        left_side: ExpressionNode,
        right_side: ExpressionNode,
        span: Span,
    },
    ExternStatement {
        span: Span,
    },
}

#[derive(Clone)]
pub enum ExpressionNode {
    AndExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    WideAndExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    OrExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    WideOrExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    XorExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    IsEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    NotEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    GreaterComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    GreaterOrEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    LessComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    LessOrEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    AddExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    SubtractExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    MultiplyExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    DivideExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: Span,
    },
    NotExpression {
        source: Box<ExpressionNode>,
        span: Span,
    },
    NewExpression {
        class: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>,
        span: Span,
    },
    InverseExpression {
        source: Box<ExpressionNode>,
        span: Span,
    },
    NegateExpression {
        source: Box<ExpressionNode>,
        span: Span,
    },
    SymbolAccessExpression {
        source: Box<ExpressionNode>,
        symbol: String,
        span: Span,
    },
    FunctionCallExpression {
        source: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>,
        span: Span,
    },
    IndexAccessExpression {
        source: Box<ExpressionNode>,
        index: Vec<ExpressionNode>,
        span: Span,
    },
    IncrementExpression {
        source: Box<ExpressionNode>,
        span: Span,
    },
    DecrementExpression {
        source: Box<ExpressionNode>,
        span: Span,
    },
    Variable {
        name: String,
        span: Span,
    },
    // TODO float
    NumberLiteral {
        value: i32,
        span: Span,
    },
    BooleanLiteral {
        value: bool,
        span: Span,
    },
    StringLiteral {
        value: String,
        span: Span,
    },
    ArrayDefinition {
        elements: Vec<ExpressionNode>,
        span: Span,
    },
}

impl StatementNode {
    pub fn span(&self) -> Span {
        match self {
            StatementNode::ImportStatement { span, .. } => span.clone(),
            StatementNode::LetStatement { span, .. } => span.clone(),
            StatementNode::FunctionDefStatement { span, .. } => span.clone(),
            StatementNode::ClassDefStatement { span, .. } => span.clone(),
            StatementNode::IfStatement { span, .. } => span.clone(),
            StatementNode::WhileStatement { span, .. } => span.clone(),
            StatementNode::ForStatement { span, .. } => span.clone(),
            StatementNode::ScopeStatement { span, .. } => span.clone(),
            StatementNode::ReturnStatement { span, .. } => span.clone(),
            StatementNode::FunctionCallStatement { span, .. } => span.clone(),
            StatementNode::SymbolAccessStatement { span, .. } => span.clone(),
            StatementNode::ArrayAccessStatement { span, .. } => span.clone(),
            StatementNode::IncrementStatement { span, .. } => span.clone(),
            StatementNode::DecrementStatement { span, .. } => span.clone(),
            StatementNode::AssignmentStatement { span, .. } => span.clone(),
            StatementNode::ExternStatement { span, .. } => span.clone(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StatementNode::ImportStatement { .. } => "ImportStatement",
            StatementNode::LetStatement { .. } => "LetStatement",
            StatementNode::FunctionDefStatement { .. } => "FunctionDefStatement",
            StatementNode::ClassDefStatement { .. } => "ClassDefStatement",
            StatementNode::IfStatement { .. } => "IfStatement",
            StatementNode::WhileStatement { .. } => "WhileStatement",
            StatementNode::ForStatement { .. } => "ForStatement",
            StatementNode::ScopeStatement { .. } => "ScopeStatement",
            StatementNode::ReturnStatement { .. } => "ReturnStatement",
            StatementNode::FunctionCallStatement { .. } => "FunctionCallStatement",
            StatementNode::SymbolAccessStatement { .. } => "SymbolAccessStatement",
            StatementNode::ArrayAccessStatement { .. } => "ArrayAccessStatement",
            StatementNode::IncrementStatement { .. } => "IncrementStatement",
            StatementNode::DecrementStatement { .. } => "DecrementStatement",
            StatementNode::AssignmentStatement { .. } => "AssignmentStatement",
            StatementNode::ExternStatement { .. } => "ExternStatement",
        }
    }
}

impl ExpressionNode {
    pub fn span(&self) -> Span {
        match self {
            ExpressionNode::AndExpression { span, .. } => span.clone(),
            ExpressionNode::WideAndExpression { span, .. } => span.clone(),
            ExpressionNode::OrExpression { span, .. } => span.clone(),
            ExpressionNode::WideOrExpression { span, .. } => span.clone(),
            ExpressionNode::XorExpression { span, .. } => span.clone(),
            ExpressionNode::IsEqualComparison { span, .. } => span.clone(),
            ExpressionNode::NotEqualComparison { span, .. } => span.clone(),
            ExpressionNode::GreaterComparison { span, .. } => span.clone(),
            ExpressionNode::GreaterOrEqualComparison { span, .. } => span.clone(),
            ExpressionNode::LessComparison { span, .. } => span.clone(),
            ExpressionNode::LessOrEqualComparison { span, .. } => span.clone(),
            ExpressionNode::AddExpression { span, .. } => span.clone(),
            ExpressionNode::SubtractExpression { span, .. } => span.clone(),
            ExpressionNode::MultiplyExpression { span, .. } => span.clone(),
            ExpressionNode::DivideExpression { span, .. } => span.clone(),
            ExpressionNode::NotExpression { span, .. } => span.clone(),
            ExpressionNode::NewExpression { span, .. } => span.clone(),
            ExpressionNode::InverseExpression { span, .. } => span.clone(),
            ExpressionNode::NegateExpression { span, .. } => span.clone(),
            ExpressionNode::SymbolAccessExpression { span, .. } => span.clone(),
            ExpressionNode::FunctionCallExpression { span, .. } => span.clone(),
            ExpressionNode::IndexAccessExpression { span, .. } => span.clone(),
            ExpressionNode::IncrementExpression { span, .. } => span.clone(),
            ExpressionNode::DecrementExpression { span, .. } => span.clone(),
            ExpressionNode::Variable { span, .. } => span.clone(),
            ExpressionNode::NumberLiteral { span, .. } => span.clone(),
            ExpressionNode::BooleanLiteral { span, .. } => span.clone(),
            ExpressionNode::StringLiteral { span, .. } => span.clone(),
            ExpressionNode::ArrayDefinition { span, .. } => span.clone(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ExpressionNode::AndExpression { .. } => "AndExpression",
            ExpressionNode::WideAndExpression { .. } => "WideAndExpression",
            ExpressionNode::OrExpression { .. } => "OrExpression",
            ExpressionNode::WideOrExpression { .. } => "WideOrExpression",
            ExpressionNode::XorExpression { .. } => "XorExpression",
            ExpressionNode::IsEqualComparison { .. } => "IsEqualComparison",
            ExpressionNode::NotEqualComparison { .. } => "NotEqualComparison",
            ExpressionNode::GreaterComparison { .. } => "GreaterComparison",
            ExpressionNode::GreaterOrEqualComparison { .. } => "GreaterOrEqualComparison",
            ExpressionNode::LessComparison { .. } => "LessComparison",
            ExpressionNode::LessOrEqualComparison { .. } => "LessOrEqualComparison",
            ExpressionNode::AddExpression { .. } => "AddExpression",
            ExpressionNode::SubtractExpression { .. } => "SubtractExpression",
            ExpressionNode::MultiplyExpression { .. } => "MultiplyExpression",
            ExpressionNode::DivideExpression { .. } => "DivideExpression",
            ExpressionNode::NotExpression { .. } => "NotExpression",
            ExpressionNode::NewExpression { .. } => "NewExpression",
            ExpressionNode::InverseExpression { .. } => "InverseExpression",
            ExpressionNode::NegateExpression { .. } => "NegateExpression",
            ExpressionNode::SymbolAccessExpression { .. } => "SymbolAccessExpression",
            ExpressionNode::FunctionCallExpression { .. } => "FunctionCallExpression",
            ExpressionNode::IndexAccessExpression { .. } => "IndexAccessExpression",
            ExpressionNode::IncrementExpression { .. } => "IncrementExpression",
            ExpressionNode::DecrementExpression { .. } => "DecrementExpression",
            ExpressionNode::Variable { .. } => "Variable",
            ExpressionNode::NumberLiteral { .. } => "NumberLiteral",
            ExpressionNode::BooleanLiteral { .. } => "BooleanLiteral",
            ExpressionNode::StringLiteral { .. } => "StringLiteral",
            ExpressionNode::ArrayDefinition { .. } => "ArrayDefinition",
        }
    }
}
