use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalStatement {
    pub initializations: Vec<Initialization>,
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
