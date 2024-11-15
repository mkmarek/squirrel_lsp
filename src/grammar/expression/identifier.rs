use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::{Location, Token};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifierExpression {
    pub token: Token,
    pub from: Location,
    pub to: Location,
}
