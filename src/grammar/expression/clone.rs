use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneExpression {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}
