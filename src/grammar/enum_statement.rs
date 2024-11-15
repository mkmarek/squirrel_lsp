use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumStatement {
    pub name: Expression,
    pub enumerations: Vec<Enumeration>,
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
