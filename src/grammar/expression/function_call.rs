use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallExpression {
    pub function: Expression,
    pub arguments: Vec<Expression>,
    pub from: Location,
    pub to: Location,
}