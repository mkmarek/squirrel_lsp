mod block_statement;
mod break_statement;
mod class_definition;
mod const_statement;
mod continue_statement;
mod do_while_statement;
mod enum_statement;
mod expression;
mod expression_statement;
mod for_statement;
mod foreach_statement;
mod function_definition;
mod if_statement;
mod local_statement;
mod return_statement;
mod switch_statement;
mod throw_statement;
mod try_catch_statement;
mod while_statement;
mod yield_statement;

pub mod expressions {
    pub use super::expression::*;
}

pub mod statements {
    pub use super::block_statement::*;
    pub use super::break_statement::*;
    pub use super::class_definition::*;
    pub use super::const_statement::*;
    pub use super::continue_statement::*;
    pub use super::do_while_statement::*;
    pub use super::enum_statement::*;
    use super::expression::Expression;
    pub use super::expression_statement::*;
    use super::expressions;
    pub use super::for_statement::*;
    pub use super::foreach_statement::*;
    pub use super::function_definition::*;
    pub use super::if_statement::*;
    pub use super::local_statement::*;
    pub use super::return_statement::*;
    pub use super::switch_statement::*;
    pub use super::throw_statement::*;
    pub use super::try_catch_statement::*;
    pub use super::while_statement::*;
    pub use super::yield_statement::*;

    use crate::squirrel_lexer::Location;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statement {
        Block(Box<BlockStatement>),
        If(Box<IfStatement>),
        While(Box<WhileStatement>),
        DoWhile(Box<DoWhileStatement>),
        Switch(Box<SwitchStatement>),
        For(Box<ForStatement>),
        ForEach(Box<ForEachStatement>),
        TryCatch(Box<TryCatchStatement>),
        Break(Box<BreakStatement>),
        Continue(Box<ContinueStatement>),
        Return(Box<ReturnStatement>),
        Yield(Box<YieldStatement>),
        Throw(Box<ThrowStatement>),
        Expression(Box<ExpressionStatement>),
        Const(Box<ConstStatement>),
        Local(Box<LocalStatement>),
        FunctionDefinition(Box<FunctionDefinition>),
        Class(Box<ClassDefinition>),
        Enum(Box<EnumStatement>),
    }

    impl Statement {
        pub fn new_break(from: Location, to: Location) -> Statement {
            Statement::Break(Box::new(BreakStatement { from, to }))
        }

        pub fn new_continue(from: Location, to: Location) -> Statement {
            Statement::Continue(Box::new(ContinueStatement { from, to }))
        }

        pub fn new_return(
            expression: Option<expressions::Expression>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Return(Box::new(ReturnStatement {
                expression,
                from,
                to,
            }))
        }

        pub fn new_yield(
            expression: Option<expressions::Expression>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Yield(Box::new(YieldStatement {
                expression,
                from,
                to,
            }))
        }

        pub fn new_expression(
            expression: expressions::Expression,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Expression(Box::new(ExpressionStatement {
                expression,
                from,
                to,
            }))
        }

        pub fn new_switch(
            expression: expressions::Expression,
            cases: Vec<Case>,
            default: Option<Statements>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Switch(Box::new(SwitchStatement {
                expression,
                cases,
                default,
                from,
                to,
            }))
        }

        pub fn new_try_catch(
            try_statement: Statement,
            catch_variable: Expression,
            catch_statement: Statement,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::TryCatch(Box::new(TryCatchStatement {
                try_statement,
                catch_variable,
                catch_statement,
                from,
                to,
            }))
        }

        pub fn new_throw(
            expression: expressions::Expression,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Throw(Box::new(ThrowStatement {
                expression,
                from,
                to,
            }))
        }

        pub fn new_const(
            name: Expression,
            expression: Expression,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Const(Box::new(ConstStatement {
                name,
                expression,
                from,
                to,
            }))
        }

        pub fn new_enum(
            name: Expression,
            enumerations: Vec<Enumeration>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Enum(Box::new(EnumStatement {
                name,
                enumerations,
                from,
                to,
            }))
        }

        pub fn new_for(
            initialization: Option<Statement>,
            condition: Option<expressions::Expression>,
            increment: Option<expressions::Expression>,
            statement: Statement,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::For(Box::new(ForStatement {
                initialization,
                condition,
                increment,
                statement,
                from,
                to,
            }))
        }

        pub fn new_foreach(
            key: Option<Expression>,
            value: Expression,
            iterable: Expression,
            statement: Statement,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::ForEach(Box::new(ForEachStatement {
                key,
                value,
                iterable,
                statement,
                from,
                to,
            }))
        }

        pub fn new_local(
            initializations: Vec<Initialization>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::Local(Box::new(LocalStatement {
                initializations,
                from,
                to,
            }))
        }

        pub fn new_do_while(
            condition: expressions::Expression,
            statement: Statement,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::DoWhile(Box::new(DoWhileStatement {
                statement,
                condition,
                from,
                to,
            }))
        }

        pub fn new_while(
            condition: expressions::Expression,
            statement: Statement,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::While(Box::new(WhileStatement {
                condition,
                statement,
                from,
                to,
            }))
        }

        pub fn new_if(
            condition: expressions::Expression,
            if_true: Statement,
            if_false: Option<Statement>,
            from: Location,
            to: Location,
        ) -> Statement {
            Statement::If(Box::new(IfStatement {
                condition,
                if_true,
                if_false,
                from,
                to,
            }))
        }

        pub fn new_block(statements: Statements, from: Location, to: Location) -> Statement {
            Statement::Block(Box::new(BlockStatement {
                statements,
                from,
                to,
            }))
        }

        pub fn get_from(&self) -> Location {
            match self {
                Statement::Block(block) => block.from.clone(),
                Statement::If(if_statement) => if_statement.from.clone(),
                Statement::While(while_statement) => while_statement.from.clone(),
                Statement::DoWhile(do_while_statement) => do_while_statement.from.clone(),
                Statement::Switch(switch_statement) => switch_statement.from.clone(),
                Statement::For(for_statement) => for_statement.from.clone(),
                Statement::ForEach(for_each_statement) => for_each_statement.from.clone(),
                Statement::TryCatch(try_catch_statement) => try_catch_statement.from.clone(),
                Statement::Break(break_statement) => break_statement.from.clone(),
                Statement::Continue(continue_statement) => continue_statement.from.clone(),
                Statement::Return(return_statement) => return_statement.from.clone(),
                Statement::Yield(yield_statement) => yield_statement.from.clone(),
                Statement::Throw(throw_statement) => throw_statement.from.clone(),
                Statement::Expression(expression_statement) => expression_statement.from.clone(),
                Statement::Const(const_statement) => const_statement.from.clone(),
                Statement::Local(local_statement) => local_statement.from.clone(),
                Statement::FunctionDefinition(function_definition) => {
                    function_definition.from.clone()
                }
                Statement::Class(class_definition) => class_definition.from.clone(),
                Statement::Enum(enum_statement) => enum_statement.from.clone(),
            }
        }

        fn get_to(&self) -> Location {
            match self {
                Statement::Block(block) => block.to.clone(),
                Statement::If(if_statement) => if_statement.to.clone(),
                Statement::While(while_statement) => while_statement.to.clone(),
                Statement::DoWhile(do_while_statement) => do_while_statement.to.clone(),
                Statement::Switch(switch_statement) => switch_statement.to.clone(),
                Statement::For(for_statement) => for_statement.to.clone(),
                Statement::ForEach(for_each_statement) => for_each_statement.to.clone(),
                Statement::TryCatch(try_catch_statement) => try_catch_statement.to.clone(),
                Statement::Break(break_statement) => break_statement.to.clone(),
                Statement::Continue(continue_statement) => continue_statement.to.clone(),
                Statement::Return(return_statement) => return_statement.to.clone(),
                Statement::Yield(yield_statement) => yield_statement.to.clone(),
                Statement::Throw(throw_statement) => throw_statement.to.clone(),
                Statement::Expression(expression_statement) => expression_statement.to.clone(),
                Statement::Const(const_statement) => const_statement.to.clone(),
                Statement::Local(local_statement) => local_statement.to.clone(),
                Statement::FunctionDefinition(function_definition) => {
                    function_definition.to.clone()
                }
                Statement::Class(class_definition) => class_definition.to.clone(),
                Statement::Enum(enum_statement) => enum_statement.to.clone(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Statements {
        pub statements: Vec<Statement>,
        pub from: Location,
        pub to: Location,
    }
}
