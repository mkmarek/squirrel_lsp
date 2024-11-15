use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstStatement {
    pub name: Expression,
    pub expression: Expression,
    pub from: Location,
    pub to: Location,
}
