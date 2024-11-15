use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: Option<Expression>,
    pub parameters: Vec<Expression>,
    pub statement: Statement,
    pub is_static: bool,
    pub from: Location,
    pub to: Location,
}
