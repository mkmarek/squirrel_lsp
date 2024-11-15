use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TryCatchStatement {
    pub try_statement: Statement,
    pub catch_variable: Expression,
    pub catch_statement: Statement,
    pub from: Location,
    pub to: Location,
}
