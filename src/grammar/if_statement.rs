use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_true: Statement,
    pub if_false: Option<Statement>,
    pub from: Location,
    pub to: Location,
}
