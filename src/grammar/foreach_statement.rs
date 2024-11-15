use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForEachStatement {
    pub key: Option<Expression>,
    pub value: Expression,
    pub iterable: Expression,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}
