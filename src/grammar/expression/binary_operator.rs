use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::{Location, Operator};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryOperatorExpression {
    pub left: Expression,
    pub operator: Operator,
    pub right: Expression,
    pub from: Location,
    pub to: Location,
}
