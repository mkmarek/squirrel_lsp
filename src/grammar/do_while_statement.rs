use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub condition: Expression,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}
