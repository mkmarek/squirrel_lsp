use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::statements::Statements;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Statements,
    pub from: Location,
    pub to: Location,
}
