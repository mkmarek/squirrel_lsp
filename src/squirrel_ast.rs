use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::{Location, Operator, Token};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Statements {
    pub statements: Vec<Statement>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Block(Box<BlockStatement>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    DoWhile(Box<DoWhileStatement>),
    Switch(Box<SwitchStatement>),
    For(Box<ForStatement>),
    ForEach(Box<ForEachStatement>),
    TryCatch(Box<TryCatchStatement>),
    Break(Box<BreakStatement>),
    Continue(Box<ContinueStatement>),
    Return(Box<ReturnStatement>),
    Yield(Box<YieldStatement>),
    Throw(Box<ThrowStatement>),
    Expression(Box<ExpressionStatement>),
    Const(Box<ConstStatement>),
    Local(Box<LocalStatement>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    Class(Box<ClassDefinition>),
    Enum(Box<EnumStatement>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Statements,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_true: Statement,
    pub if_false: Option<Statement>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub condition: Expression,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub expression: Expression,
    pub cases: Vec<Case>,
    pub default: Option<Statements>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub initialization: Option<Statement>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForEachStatement {
    pub key: Option<Expression>,
    pub value: Expression,
    pub iterable: Expression,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TryCatchStatement {
    pub try_statement: Statement,
    pub catch_variable: Expression,
    pub catch_statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BreakStatement {
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YieldStatement {
    pub expression: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstStatement {
    pub name: Expression,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalStatement {
    pub initializations: Vec<Initialization>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumStatement {
    pub name: Expression,
    pub enumerations: Vec<Enumeration>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub name: Option<Expression>,
    pub extends: Option<Expression>,
    pub members: Vec<ClassMemberDeclaration>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    pub name: Expression,
    pub value: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: Option<Expression>,
    pub parameters: Vec<Expression>,
    pub statement: Statement,
    pub is_static: bool,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClassMemberDeclaration {
    FieldDeclaration(ClassFieldDeclaration),
    MethodDeclaration(FunctionDeclaration),
    ConstructorDeclaration(FunctionDeclaration),
}

pub fn class_field_declaration(
    name: Expression,
    expression: Expression,
    is_static: bool,
    from: Location,
    to: Location,
) -> ClassMemberDeclaration {
    ClassMemberDeclaration::FieldDeclaration(ClassFieldDeclaration {
        name,
        expression,
        is_static,
        from,
        to,
    })
}

pub fn class_method_declaration(
    name: Option<Expression>,
    parameters: Vec<Expression>,
    statement: Statement,
    is_static: bool,
    from: Location,
    to: Location,
) -> ClassMemberDeclaration {
    ClassMemberDeclaration::MethodDeclaration(FunctionDeclaration {
        name,
        parameters,
        statement,
        is_static,
        from,
        to,
    })
}

pub fn class_constructor_declaration(
    parameters: Vec<Expression>,
    statement: Statement,
    is_static: bool,
    from: Location,
    to: Location,
) -> ClassMemberDeclaration {
    ClassMemberDeclaration::ConstructorDeclaration(FunctionDeclaration {
        name: None,
        parameters,
        statement,
        is_static,
        from,
        to,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassFieldDeclaration {
    pub name: Expression,
    pub expression: Expression,
    pub is_static: bool,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Initialization {
    pub name: String,
    pub expression: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Case {
    pub expression: Expression,
    pub statements: Statements,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    UnaryOperator(Box<UnaryOperatorExpression>),
    PostfixUnaryOperator(Box<PostfixUnaryOperatorExpression>),
    BinaryOperator(Box<BinaryOperatorExpression>),
    Spread(Box<SpreadExpression>),
    StringLiteral(Box<StringLiteralExpression>),
    MutliLineStringLiteral(Box<MutliLineStringLiteralExpression>),
    FloatLiteral(Box<FloatLiteralExpression>),
    IntegerLiteral(Box<IntegerLiteralExpression>),
    Table(Box<TableExpression>),
    Clone(Box<CloneExpression>),
    Resume(Box<ResumeExpression>),
    Delete(Box<DeleteExpression>),
    Array(Box<ArrayExpression>),
    ArrayAccess(Box<ArrayAccessExpression>),
    Grouping(Box<GroupingExpression>),
    Identifier(Box<IdentifierExpression>),
    NullLiteral(Box<NullLiteralExpression>),
    BooleanLiteral(Box<BooleanLiteralExpression>),
    FunctionCall(Box<FunctionCallExpression>),
    ScopeResolution(Box<ScopeResolutionExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    Class(Box<ClassDefinition>),
    TernaryOperator(Box<TernaryOperatorExpression>),
    Function(Box<FunctionDeclaration>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnaryOperatorExpression {
    pub operator: Operator,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostfixUnaryOperatorExpression {
    pub operator: Operator,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryOperatorExpression {
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpreadExpression {
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringLiteralExpression {
    pub value: String,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutliLineStringLiteralExpression {
    pub value: String,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatLiteralExpression {
    pub value: f64,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerLiteralExpression {
    pub value: i64,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableExpression {
    pub entries: Vec<TableEntry>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneExpression {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResumeExpression {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteExpression {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrayAccessExpression {
    pub array: Expression,
    pub index: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupingExpression {
    pub expression: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifierExpression {
    pub token: Token,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NullLiteralExpression {
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanLiteralExpression {
    pub value: bool,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallExpression {
    pub function: Expression,
    pub arguments: Vec<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeResolutionExpression {
    pub scope: Option<String>,
    pub accessor: String,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    pub expression: Expression,
    pub member: String,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TernaryOperatorExpression {
    pub condition: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableEntry {
    pub id: Option<Expression>,
    pub value: Option<Expression>,
    pub function: Option<FunctionDeclaration>,
    pub from: Location,
    pub to: Location,
}

pub fn unary_operator_expression(
    operator: Operator,
    expression: Expression,
    from: Location,
    to: Location,
) -> Expression {
    Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
        operator,
        expression,
        from,
        to,
    }))
}

pub fn postfix_unary_operator_expression(
    operator: Operator,
    expression: Expression,
    from: Location,
    to: Location,
) -> Expression {
    Expression::PostfixUnaryOperator(Box::new(PostfixUnaryOperatorExpression {
        operator,
        expression,
        from,
        to,
    }))
}

pub fn binary_operator_expression(
    left: Expression,
    operator: Operator,
    right: Expression,
    from: Location,
    to: Location,
) -> Expression {
    Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
        left,
        operator,
        right,
        from,
        to,
    }))
}

pub fn spread_expression(from: Location, to: Location) -> Expression {
    Expression::Spread(Box::new(SpreadExpression { from, to }))
}

pub fn string_literal_expression(value: String, from: Location, to: Location) -> Expression {
    Expression::StringLiteral(Box::new(StringLiteralExpression { value, from, to }))
}

pub fn mutliline_string_literal_expression(
    value: String,
    from: Location,
    to: Location,
) -> Expression {
    Expression::MutliLineStringLiteral(Box::new(MutliLineStringLiteralExpression {
        value,
        from,
        to,
    }))
}

pub fn mutli_line_string_literal_expression(
    value: String,
    from: Location,
    to: Location,
) -> Expression {
    Expression::MutliLineStringLiteral(Box::new(MutliLineStringLiteralExpression {
        value,
        from,
        to,
    }))
}

pub fn float_literal_expression(value: f64, from: Location, to: Location) -> Expression {
    Expression::FloatLiteral(Box::new(FloatLiteralExpression { value, from, to }))
}

pub fn integer_literal_expression(value: i64, from: Location, to: Location) -> Expression {
    Expression::IntegerLiteral(Box::new(IntegerLiteralExpression { value, from, to }))
}

pub fn table_expression(entries: Vec<TableEntry>, from: Location, to: Location) -> Expression {
    Expression::Table(Box::new(TableExpression { entries, from, to }))
}

pub fn clone_expression(expression: Expression, from: Location, to: Location) -> Expression {
    Expression::Clone(Box::new(CloneExpression {
        expression,
        from,
        to,
    }))
}

pub fn resume_expression(expression: Expression, from: Location, to: Location) -> Expression {
    Expression::Resume(Box::new(ResumeExpression {
        expression,
        from,
        to,
    }))
}

pub fn delete_expression(expression: Expression, from: Location, to: Location) -> Expression {
    Expression::Delete(Box::new(DeleteExpression {
        expression,
        from,
        to,
    }))
}

pub fn array_expression(elements: Vec<Expression>, from: Location, to: Location) -> Expression {
    Expression::Array(Box::new(ArrayExpression { elements, from, to }))
}

pub fn array_access_expression(
    array: Expression,
    index: Expression,
    from: Location,
    to: Location,
) -> Expression {
    Expression::ArrayAccess(Box::new(ArrayAccessExpression {
        array,
        index,
        from,
        to,
    }))
}

pub fn grouping_expression(
    expression: Option<Expression>,
    from: Location,
    to: Location,
) -> Expression {
    Expression::Grouping(Box::new(GroupingExpression {
        expression,
        from,
        to,
    }))
}

pub fn identifier_expression(token: Token, from: Location, to: Location) -> Expression {
    Expression::Identifier(Box::new(IdentifierExpression { token, from, to }))
}

pub fn null_literal_expression(from: Location, to: Location) -> Expression {
    Expression::NullLiteral(Box::new(NullLiteralExpression { from, to }))
}

pub fn boolean_literal_expression(value: bool, from: Location, to: Location) -> Expression {
    Expression::BooleanLiteral(Box::new(BooleanLiteralExpression { value, from, to }))
}

pub fn function_call_expression(
    function: Expression,
    arguments: Vec<Expression>,
    from: Location,
    to: Location,
) -> Expression {
    Expression::FunctionCall(Box::new(FunctionCallExpression {
        function,
        arguments,
        from,
        to,
    }))
}

pub fn scope_resolution_expression(
    scope: Option<String>,
    accessor: String,
    from: Location,
    to: Location,
) -> Expression {
    Expression::ScopeResolution(Box::new(ScopeResolutionExpression {
        scope,
        accessor,
        from,
        to,
    }))
}

pub fn member_access_expression(
    expression: Expression,
    member: String,
    from: Location,
    to: Location,
) -> Expression {
    Expression::MemberAccess(Box::new(MemberAccessExpression {
        expression,
        member,
        from,
        to,
    }))
}

pub fn ternary_operator_expression(
    condition: Expression,
    if_true: Expression,
    if_false: Expression,
    from: Location,
    to: Location,
) -> Expression {
    Expression::TernaryOperator(Box::new(TernaryOperatorExpression {
        condition,
        if_true,
        if_false,
        from,
        to,
    }))
}

pub fn class_expression(
    name: Option<Expression>,
    extends: Option<Expression>,
    members: Vec<ClassMemberDeclaration>,
    from: Location,
    to: Location,
) -> Expression {
    Expression::Class(Box::new(ClassDefinition {
        name,
        extends,
        members,
        from,
        to,
    }))
}

pub fn function_expression(
    name: Option<Expression>,
    parameters: Vec<Expression>,
    body: Statement,
    is_static: bool,
    from: Location,
    to: Location,
) -> Expression {
    Expression::Function(Box::new(FunctionDeclaration {
        name,
        parameters,
        statement: body,
        is_static,
        from,
        to,
    }))
}

pub fn block_statement(statements: Statements, from: Location, to: Location) -> Statement {
    Statement::Block(Box::new(BlockStatement {
        statements,
        from,
        to,
    }))
}

pub fn if_statement(
    condition: Expression,
    if_true: Statement,
    if_false: Option<Statement>,
    from: Location,
    to: Location,
) -> Statement {
    Statement::If(Box::new(IfStatement {
        condition,
        if_true,
        if_false,
        from,
        to,
    }))
}

pub fn while_statement(
    condition: Expression,
    body: Statement,
    from: Location,
    to: Location,
) -> Statement {
    Statement::While(Box::new(WhileStatement {
        condition,
        statement: body,
        from,
        to,
    }))
}

pub fn do_while_statement(
    condition: Expression,
    body: Statement,
    from: Location,
    to: Location,
) -> Statement {
    Statement::DoWhile(Box::new(DoWhileStatement {
        condition,
        statement: body,
        from,
        to,
    }))
}

pub fn switch_statement(
    expression: Expression,
    cases: Vec<Case>,
    default: Option<Statements>,
    from: Location,
    to: Location,
) -> Statement {
    Statement::Switch(Box::new(SwitchStatement {
        expression,
        cases,
        default,
        from,
        to,
    }))
}

pub fn switch_case(expression: Expression, body: Statements, from: Location, to: Location) -> Case {
    Case {
        expression,
        statements: body,
        from,
        to,
    }
}

pub fn for_statement(
    initialization: Option<Statement>,
    condition: Option<Expression>,
    increment: Option<Expression>,
    body: Statement,
    from: Location,
    to: Location,
) -> Statement {
    Statement::For(Box::new(ForStatement {
        initialization,
        condition,
        increment,
        statement: body,
        from,
        to,
    }))
}

pub fn foreach_statement(
    key: Option<Expression>,
    value: Expression,
    iterable: Expression,
    body: Statement,
    from: Location,
    to: Location,
) -> Statement {
    Statement::ForEach(Box::new(ForEachStatement {
        key,
        value,
        iterable,
        statement: body,
        from,
        to,
    }))
}

pub fn try_catch_statement(
    try_statement: Statement,
    catch_variable: Expression,
    catch_statement: Statement,
    from: Location,
    to: Location,
) -> Statement {
    Statement::TryCatch(Box::new(TryCatchStatement {
        try_statement,
        catch_variable,
        catch_statement,
        from,
        to,
    }))
}

pub fn break_statement(from: Location, to: Location) -> Statement {
    Statement::Break(Box::new(BreakStatement { from, to }))
}

pub fn continue_statement(from: Location, to: Location) -> Statement {
    Statement::Continue(Box::new(ContinueStatement { from, to }))
}

pub fn return_statement(expression: Option<Expression>, from: Location, to: Location) -> Statement {
    Statement::Return(Box::new(ReturnStatement {
        expression,
        from,
        to,
    }))
}

pub fn yield_statement(expression: Option<Expression>, from: Location, to: Location) -> Statement {
    Statement::Yield(Box::new(YieldStatement {
        expression,
        from,
        to,
    }))
}

pub fn throw_statement(expression: Expression, from: Location, to: Location) -> Statement {
    Statement::Throw(Box::new(ThrowStatement {
        expression,
        from,
        to,
    }))
}

pub fn expression_statement(expression: Expression, from: Location, to: Location) -> Statement {
    Statement::Expression(Box::new(ExpressionStatement {
        expression,
        from,
        to,
    }))
}

pub fn const_statement(
    name: Expression,
    expression: Expression,
    from: Location,
    to: Location,
) -> Statement {
    Statement::Const(Box::new(ConstStatement {
        name,
        expression,
        from,
        to,
    }))
}

pub fn local_statement(
    initializations: Vec<Initialization>,
    from: Location,
    to: Location,
) -> Statement {
    Statement::Local(Box::new(LocalStatement {
        initializations,
        from,
        to,
    }))
}

pub fn initialization(
    name: String,
    expression: Option<Expression>,
    from: Location,
    to: Location,
) -> Initialization {
    Initialization {
        name,
        expression,
        from,
        to,
    }
}

pub fn function_declaration_statement(
    name: Expression,
    parameters: Vec<Expression>,
    body: Statement,
    is_static: bool,
    from: Location,
    to: Location,
) -> Statement {
    Statement::FunctionDeclaration(Box::new(FunctionDeclaration {
        name: Some(name),
        parameters,
        statement: body,
        is_static,
        from,
        to,
    }))
}

pub fn class_statement(
    name: Expression,
    extends: Option<Expression>,
    members: Vec<ClassMemberDeclaration>,
    from: Location,
    to: Location,
) -> Statement {
    Statement::Class(Box::new(ClassDefinition {
        name: Some(name),
        extends,
        members,
        from,
        to,
    }))
}

pub fn enum_statement(
    name: Expression,
    enumerations: Vec<Enumeration>,
    from: Location,
    to: Location,
) -> Statement {
    Statement::Enum(Box::new(EnumStatement {
        name,
        enumerations,
        from,
        to,
    }))
}
