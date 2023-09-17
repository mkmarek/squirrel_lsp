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
    Block(Statements, Location, Location),
    If(
        Expression,
        Box<Statement>,
        Option<Box<Statement>>,
        Location,
        Location,
    ),
    While(Expression, Box<Statement>, Location, Location),
    DoWhile(Expression, Box<Statement>, Location, Location),
    Switch(
        Expression,
        Vec<Case>,
        Option<Statements>,
        Location,
        Location,
    ),
    For(
        Option<Box<Statement>>,
        Option<Expression>,
        Option<Expression>,
        Box<Statement>,
        Location,
        Location,
    ),
    ForEach(
        Option<String>,
        String,
        Expression,
        Box<Statement>,
        Location,
        Location,
    ),
    TryCatch(Box<Statement>, String, Box<Statement>, Location, Location),
    Break(Location, Location),
    Continue(Location, Location),
    Return(Option<Expression>, Location, Location),
    Yield(Option<Expression>, Location, Location),
    Throw(Expression, Location, Location),
    Expression(Expression, Location, Location),
    Const(String, Expression, Location, Location),
    Local(Vec<Initialization>, Location, Location),
    FunctionDeclaration(Box<FunctionDeclaration>),
    Class(ClassDefinition),
    Enum(String, Vec<Enumeration>, Location, Location),
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
    pub name: String,
    pub value: Option<Expression>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: Option<Expression>,
    pub parameters: Vec<Expression>,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClassMemberDeclaration {
    FieldDeclaration(
        Option<String>,
        Option<Expression>,
        Expression,
        bool,
        Location,
        Location,
    ),
    MethodDeclaration(FunctionDeclaration, bool),
    ConstructorDeclaration(FunctionDeclaration, bool),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Initialization {
    pub name: String,
    pub expression: Option<Expression>,
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
    UnaryOperator(Operator, Box<Expression>, Location, Location),
    PostfixUnaryOperator(Operator, Box<Expression>, Location, Location),
    BinaryOperator(
        Box<Expression>,
        Operator,
        Box<Expression>,
        Location,
        Location,
    ),
    Spread(Location, Location),
    StringLiteral(String, Location, Location),
    MutliLineStringLiteral(String, Location, Location),
    FloatLiteral(f64, Location, Location),
    IntegerLiteral(i64, Location, Location),
    Table(Vec<TableEntry>, Location, Location),
    Clone(Box<Expression>, Location, Location),
    Resume(Box<Expression>, Location, Location),
    Delete(Box<Expression>, Location, Location),
    Array(Vec<Expression>, Location, Location),
    ArrayAccess(Box<Expression>, Box<Expression>, Location, Location),
    Grouping(Option<Box<Expression>>, Location, Location),
    Identifier(Token, Location, Location),
    NullLiteral(Location, Location),
    BooleanLiteral(bool, Location, Location),
    FunctionCall(Box<Expression>, Vec<Expression>, Location, Location),
    ScopeResolution(Option<Box<Expression>>, Box<Expression>, Location, Location),
    MemberAccess(Box<Expression>, String, Location, Location),
    Class(Box<ClassDefinition>),
    TernaryOperator(
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Location,
        Location,
    ),
    Function(Box<FunctionDeclaration>, Location, Location),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableEntry {
    pub id: Option<String>,
    pub id_exp: Option<Expression>,
    pub value: Option<Expression>,
    pub function: Option<FunctionDeclaration>,
    pub from: Location,
    pub to: Location,
}
