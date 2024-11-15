use crate::squirrel_lexer::{Location, Operator, Token};

use super::{class_definition::ClassDefinition, function_definition::FunctionDefinition};

mod array;
mod array_access;
mod binary_operator;
mod boolean_literal;
mod clone;
mod delete;
mod float_literal;
mod function_call;
mod grouping;
mod identifier;
mod integer_literal;
mod member_access;
mod multiline_string_literal;
mod null_literal;
mod postfix_unary_operator;
mod resume;
mod scope_resolution;
mod spread;
mod string_literal;
mod table;
mod ternary_operator;
mod unary_operator;

pub use array::*;
pub use array_access::*;
pub use binary_operator::*;
pub use boolean_literal::*;
pub use clone::*;
pub use delete::*;
pub use float_literal::*;
pub use function_call::*;
pub use grouping::*;
pub use identifier::*;
pub use integer_literal::*;
pub use member_access::*;
pub use multiline_string_literal::*;
pub use null_literal::*;
pub use postfix_unary_operator::*;
pub use resume::*;
pub use scope_resolution::*;
pub use spread::*;
pub use string_literal::*;
pub use table::*;
pub use ternary_operator::*;
pub use unary_operator::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    UnaryOperator(Box<UnaryOperatorExpression>),
    PostfixUnaryOperator(Box<PostfixUnaryOperatorExpression>),
    BinaryOperator(Box<BinaryOperatorExpression>),
    Spread(Box<SpreadExpression>),
    StringLiteral(Box<StringLiteralExpression>),
    MutliLineStringLiteral(Box<MutliLineStringLiteralExpression>),
    FloatLiteral(Box<FloatLiteralExpression>),
    IntegerLiteral(Box<IntegerLiteralExpression>),
    Table(Box<TableExpression>),
    Clone(Box<CloneExpression>),
    Resume(Box<ResumeExpression>),
    Delete(Box<DeleteExpression>),
    Array(Box<ArrayExpression>),
    ArrayAccess(Box<ArrayAccessExpression>),
    Grouping(Box<GroupingExpression>),
    Identifier(Box<IdentifierExpression>),
    NullLiteral(Box<NullLiteralExpression>),
    BooleanLiteral(Box<BooleanLiteralExpression>),
    FunctionCall(Box<FunctionCallExpression>),
    ScopeResolution(Box<ScopeResolutionExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    Class(Box<ClassDefinition>),
    TernaryOperator(Box<TernaryOperatorExpression>),
    Function(Box<FunctionDefinition>),
}

impl Expression {
    pub fn identifier(token: Token, from: Location, to: Location) -> Expression {
        Expression::Identifier(Box::new(IdentifierExpression { token, from, to }))
    }

    pub fn scope_resolution(
        scope: Option<String>,
        accessor: String,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::ScopeResolution(Box::new(ScopeResolutionExpression {
            scope,
            accessor,
            from,
            to,
        }))
    }

    pub fn spread(from: Location, to: Location) -> Expression {
        Expression::Spread(Box::new(SpreadExpression { from, to }))
    }

    pub fn ternary_operator(
        condition: Expression,
        if_true: Expression,
        if_false: Expression,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::TernaryOperator(Box::new(TernaryOperatorExpression {
            condition,
            if_true,
            if_false,
            from,
            to,
        }))
    }

    pub fn binary_operator(
        left: Expression,
        operator: Operator,
        right: Expression,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::BinaryOperator(Box::new(BinaryOperatorExpression {
            left,
            operator,
            right,
            from,
            to,
        }))
    }

    pub fn unary_operator(
        operator: Operator,
        expression: Expression,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::UnaryOperator(Box::new(UnaryOperatorExpression {
            operator,
            expression,
            from,
            to,
        }))
    }

    pub fn resume(expression: Expression, from: Location, to: Location) -> Expression {
        Expression::Resume(Box::new(ResumeExpression {
            expression,
            from,
            to,
        }))
    }

    pub fn delete(expression: Expression, from: Location, to: Location) -> Expression {
        Expression::Delete(Box::new(DeleteExpression {
            expression,
            from,
            to,
        }))
    }

    pub fn clone(expression: Expression, from: Location, to: Location) -> Expression {
        Expression::Clone(Box::new(CloneExpression {
            expression,
            from,
            to,
        }))
    }

    pub fn postfix_unary_operator(
        operator: Operator,
        expression: Expression,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::PostfixUnaryOperator(Box::new(PostfixUnaryOperatorExpression {
            expression,
            operator,
            from,
            to,
        }))
    }

    pub fn member_access(
        expression: Expression,
        member: String,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::MemberAccess(Box::new(MemberAccessExpression {
            expression,
            member,
            from,
            to,
        }))
    }

    pub fn function_call(
        function: Expression,
        arguments: Vec<Expression>,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::FunctionCall(Box::new(FunctionCallExpression {
            function,
            arguments,
            from,
            to,
        }))
    }

    pub fn array_access(
        array: Expression,
        index: Expression,
        from: Location,
        to: Location,
    ) -> Expression {
        Expression::ArrayAccess(Box::new(ArrayAccessExpression {
            array,
            index,
            from,
            to,
        }))
    }

    pub fn string_literal(value: String, from: Location, to: Location) -> Expression {
        Expression::StringLiteral(Box::new(StringLiteralExpression { value, from, to }))
    }

    pub fn multiline_string_literal(value: String, from: Location, to: Location) -> Expression {
        Expression::MutliLineStringLiteral(Box::new(MutliLineStringLiteralExpression {
            value,
            from,
            to,
        }))
    }

    pub fn integer_literal(value: i64, from: Location, to: Location) -> Expression {
        Expression::IntegerLiteral(Box::new(IntegerLiteralExpression { value, from, to }))
    }

    pub fn float_literal(value: f64, from: Location, to: Location) -> Expression {
        Expression::FloatLiteral(Box::new(FloatLiteralExpression { value, from, to }))
    }

    pub fn null_literal(from: Location, to: Location) -> Expression {
        Expression::NullLiteral(Box::new(NullLiteralExpression { from, to }))
    }

    pub fn boolean_literal(value: bool, from: Location, to: Location) -> Expression {
        Expression::BooleanLiteral(Box::new(BooleanLiteralExpression { value, from, to }))
    }

    pub fn array(elements: Vec<Expression>, from: Location, to: Location) -> Expression {
        Expression::Array(Box::new(ArrayExpression { elements, from, to }))
    }

    pub fn grouping(expression: Option<Expression>, from: Location, to: Location) -> Expression {
        Expression::Grouping(Box::new(GroupingExpression {
            expression,
            from,
            to,
        }))
    }

    pub fn table(entries: Vec<TableEntry>, from: Location, to: Location) -> Expression {
        Expression::Table(Box::new(TableExpression { entries, from, to }))
    }
}
