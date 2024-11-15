use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringLiteralExpression {
    pub value: String,
    pub from: Location,
    pub to: Location,
}
