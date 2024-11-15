use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}
