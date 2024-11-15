use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct YieldStatement {
    pub expression: Option<Expression>,
    pub from: Location,
    pub to: Location,
}
