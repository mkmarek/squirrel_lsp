use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatLiteralExpression {
    pub value: f64,
    pub from: Location,
    pub to: Location,
}
