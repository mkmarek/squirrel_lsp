use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    pub expression: Expression,
    pub member: String,
    pub from: Location,
    pub to: Location,
}
