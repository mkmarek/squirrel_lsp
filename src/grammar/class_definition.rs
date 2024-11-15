use serde::{Deserialize, Serialize};

use crate::squirrel_lexer::Location;

use super::{
    expression::Expression, function_definition::FunctionDefinition, statements::Statement,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub name: Option<Expression>,
    pub extends: Option<Expression>,
    pub members: Vec<ClassMemberDefinition>,
    pub from: Location,
    pub to: Location,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClassMemberDefinition {
    Field(ClassFieldDefinition),
    Method(FunctionDefinition),
    Constructor(FunctionDefinition),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassFieldDefinition {
    pub name: Expression,
    pub expression: Expression,
    pub is_static: bool,
    pub from: Location,
    pub to: Location,
}

impl ClassMemberDefinition {
    pub fn field(
        name: Expression,
        expression: Expression,
        is_static: bool,
        from: Location,
        to: Location,
    ) -> ClassMemberDefinition {
        ClassMemberDefinition::Field(ClassFieldDefinition {
            name,
            expression,
            is_static,
            from,
            to,
        })
    }

    pub fn method(
        name: Expression,
        parameters: Vec<Expression>,
        statement: Statement,
        is_static: bool,
        from: Location,
        to: Location,
    ) -> ClassMemberDefinition {
        ClassMemberDefinition::Method(FunctionDefinition {
            name: Some(name),
            parameters,
            statement,
            is_static,
            from,
            to,
        })
    }

    pub fn constructor(
        parameters: Vec<Expression>,
        statement: Statement,
        is_static: bool,
        from: Location,
        to: Location,
    ) -> ClassMemberDefinition {
        ClassMemberDefinition::Constructor(FunctionDefinition {
            name: None,
            parameters,
            statement,
            is_static,
            from,
            to,
        })
    }
}
