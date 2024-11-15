use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statements};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub expression: Expression,
    pub cases: Vec<Case>,
    pub default: Option<Statements>,
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
