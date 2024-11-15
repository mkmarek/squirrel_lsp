use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeResolutionExpression {
    pub scope: Option<String>,
    pub accessor: String,
    pub from: Location,
    pub to: Location,
}
