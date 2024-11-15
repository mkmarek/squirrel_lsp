use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TernaryOperatorExpression {
    pub condition: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
    pub from: Location,
    pub to: Location,
}
