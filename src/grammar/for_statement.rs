use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{expression::Expression, statements::Statement};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub initialization: Option<Statement>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub statement: Statement,
    pub from: Location,
    pub to: Location,
}
