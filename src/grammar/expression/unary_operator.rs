use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::{Location, Operator};

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnaryOperatorExpression {
    pub operator: Operator,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}
