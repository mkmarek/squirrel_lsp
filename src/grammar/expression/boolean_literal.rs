use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanLiteralExpression {
    pub value: bool,
    pub from: Location,
    pub to: Location,
}