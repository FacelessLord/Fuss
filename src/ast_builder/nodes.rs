use crate::lexer::common::lexer::Position;

pub struct CodeNode {
    pub statement_list: Vec<StatementNode>,
    pub span: (Position, Position),
}

pub enum StatementNode {
    LetStatement {
        var_name: String,
        value: ExpressionNode,
        span: (Position, Position),
    },
    FunctionDefStatement {
        func_name: String,
        arguments: Vec<String>,
        body: Vec<StatementNode>,
        span: (Position, Position),
    },
    IfStatement {
        condition: ExpressionNode,
        then_branch: Vec<StatementNode>,
        else_branch: Option<Vec<StatementNode>>,
        span: (Position, Position),
    },
    WhileStatement {
        condition: ExpressionNode,
        body: Vec<StatementNode>,
        span: (Position, Position),
    },
    ForStatement {
        indexer_def: Box<StatementNode>,
        condition: ExpressionNode,
        update_expr: ExpressionNode,
        body: Vec<StatementNode>,
        span: (Position, Position),
    },
    ScopeStatement {
        body: Vec<StatementNode>,
        span: (Position, Position),
    },
    ReturnStatement {
        value: ExpressionNode,
        span: (Position, Position),
    },
    FunctionCallStatement {
        source: ExpressionNode,
        arguments: Vec<ExpressionNode>,
        span: (Position, Position),
    },
    SymbolAccessStatement {
        source: ExpressionNode,
        symbol: String,
        span: (Position, Position),
    },
    ArrayAccessStatement {
        source: ExpressionNode,
        index: Vec<ExpressionNode>,
        span: (Position, Position),
    },
    IncrementStatement {
        source: ExpressionNode,
        span: (Position, Position),
    },
    DecrementStatement {
        source: ExpressionNode,
        span: (Position, Position),
    },
    AssignmentStatement {
        left_side: ExpressionNode,
        right_side: ExpressionNode,
        span: (Position, Position),
    },
}

pub enum ExpressionNode {
    AndExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    WideAndExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    OrExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    WideOrExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    XorExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    IsEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    NotEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    GreaterComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    GreaterOrEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    LessComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    LessOrEqualComparison {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    AddExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    SubtractExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    MultiplyExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    DivideExpression {
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
        span: (Position, Position),
    },
    NotExpression {
        source: Box<ExpressionNode>,
        span: (Position, Position),
    },
    InverseExpression {
        source: Box<ExpressionNode>,
        span: (Position, Position),
    },
    NegateExpression {
        source: Box<ExpressionNode>,
        span: (Position, Position),
    },
    SymbolAccessExpression {
        source: Box<ExpressionNode>,
        symbol: String,
        span: (Position, Position),
    },
    FunctionCallExpression {
        source: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>,
        span: (Position, Position),
    },
    IndexAccessExpression {
        source: Box<ExpressionNode>,
        index: Vec<ExpressionNode>,
        span: (Position, Position),
    },
    IncrementExpression {
        source: Box<ExpressionNode>,
        span: (Position, Position),
    },
    DecrementExpression {
        source: Box<ExpressionNode>,
        span: (Position, Position),
    },
    Variable {
        name: String,
        span: (Position, Position),
    },
    // TODO float
    NumberLiteral {
        value: i32,
        span: (Position, Position),
    },
    BooleanLiteral {
        value: bool,
        span: (Position, Position),
    },
    StringLiteral {
        value: String,
        span: (Position, Position),
    },
    ArrayDefinition {
        elements: Vec<ExpressionNode>,
        span: (Position, Position),
    },
}
