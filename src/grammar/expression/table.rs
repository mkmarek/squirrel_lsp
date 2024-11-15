use serde::{Deserialize, Serialize};

use crate::{grammar::function_definition::FunctionDefinition, squirrel_lexer::Location};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableExpression {
    pub entries: Vec<TableEntry>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TableEntry {
    Field(TableEntryField),
    Function(TableEntryFunction),
    FieldWithExpressionKey(TableEntryFieldWithExpressionKey),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableEntryField {
    pub name: Expression,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableEntryFunction {
    pub function: FunctionDefinition,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableEntryFieldWithExpressionKey {
    pub key: Expression,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}
