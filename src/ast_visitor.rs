use crate::squirrel_ast::{
    ArrayAccessExpression, ArrayExpression, BinaryOperatorExpression, BlockStatement,
    BooleanLiteralExpression, BreakStatement, ClassDefinition, ClassMemberDeclaration,
    CloneExpression, ConstStatement, ContinueStatement, DeleteExpression, DoWhileStatement,
    EnumStatement, Expression, ExpressionStatement, FloatLiteralExpression, ForEachStatement,
    ForStatement, FunctionCallExpression, FunctionDeclaration, GroupingExpression,
    IdentifierExpression, IfStatement, IntegerLiteralExpression, LocalStatement,
    MemberAccessExpression, MutliLineStringLiteralExpression, NullLiteralExpression,
    PostfixUnaryOperatorExpression, ResumeExpression, ReturnStatement, ScopeResolutionExpression,
    SpreadExpression, Statement, Statements, StringLiteralExpression, SwitchStatement, TableEntry,
    TableExpression, TernaryOperatorExpression, ThrowStatement, TryCatchStatement,
    UnaryOperatorExpression, WhileStatement, YieldStatement,
};

#[derive(Debug, PartialEq)]
pub enum AstVisitorResult {
    Continue,
    Break,
}

pub trait AstVisitor {
    fn enter_block_statement(&mut self, _statement: &BlockStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_block_statement(&mut self, _statement: &BlockStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_if_statement(&mut self, _statement: &IfStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_if_statement(&mut self, _statement: &IfStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_while_statement(&mut self, _statement: &WhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_while_statement(&mut self, _statement: &WhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_do_while_statement(&mut self, _statement: &DoWhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_do_while_statement(&mut self, _statement: &DoWhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_switch_statement(&mut self, _statement: &SwitchStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_switch_statement(&mut self, _statement: &SwitchStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_for_statement(&mut self, _statement: &ForStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_for_statement(&mut self, _statement: &ForStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_for_each_statement(&mut self, _statement: &ForEachStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_for_each_statement(&mut self, _statement: &ForEachStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_try_catch_statement(&mut self, _statement: &TryCatchStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_catch_clause(
        &mut self,
        _variable: &Expression,
        _statement: &Statement,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_catch_clause(
        &mut self,
        _variable: &Expression,
        _statement: &Statement,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_try_catch_statement(&mut self, _statement: &TryCatchStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_break_statement(&mut self, _statement: &BreakStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_break_statement(&mut self, _statement: &BreakStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_continue_statement(&mut self, _statement: &ContinueStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_continue_statement(&mut self, _statement: &ContinueStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_return_statement(&mut self, _statement: &ReturnStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_return_statement(&mut self, _statement: &ReturnStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_yield_statement(&mut self, _statement: &YieldStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_yield_statement(&mut self, _statement: &YieldStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_throw_statement(&mut self, _statement: &ThrowStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_throw_statement(&mut self, _statement: &ThrowStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_expression_statement(&mut self, _statement: &ExpressionStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_expression_statement(&mut self, _statement: &ExpressionStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_const_statement(&mut self, _statement: &ConstStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_const_statement(&mut self, _statement: &ConstStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_local_statement(&mut self, _statement: &LocalStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_local_statement(&mut self, _statement: &LocalStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_function_declaration(&mut self, _statement: &FunctionDeclaration) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_function_declaration(&mut self, _statement: &FunctionDeclaration) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_class_definition(&mut self, _statement: &ClassDefinition) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_class_definition(&mut self, _statement: &ClassDefinition) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_enum_statement(&mut self, _statement: &EnumStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_enum_statement(&mut self, _statement: &EnumStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_unary_operator_expression(
        &mut self,
        _expression: &UnaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_unary_operator_expression(
        &mut self,
        _expression: &UnaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_postfix_unary_operator_expression(
        &mut self,
        _expression: &PostfixUnaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_postfix_unary_operator_expression(
        &mut self,
        _expression: &PostfixUnaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_binary_operator_expression(
        &mut self,
        _expression: &BinaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_binary_operator_expression(
        &mut self,
        _expression: &BinaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_spread_expression(&mut self, _expression: &SpreadExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_spread_expression(&mut self, _expression: &SpreadExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_string_literal_expression(
        &mut self,
        _expression: &StringLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_string_literal_expression(
        &mut self,
        _expression: &StringLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_multiline_string_literal_expression(
        &mut self,
        _expression: &MutliLineStringLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_multiline_string_literal_expression(
        &mut self,
        _expression: &MutliLineStringLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_float_literal_expression(
        &mut self,
        _expression: &FloatLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_float_literal_expression(
        &mut self,
        _expression: &FloatLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_integer_literal_expression(
        &mut self,
        _expression: &IntegerLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_integer_literal_expression(
        &mut self,
        _expression: &IntegerLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_table_expression(&mut self, _expression: &TableExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_table_expression(&mut self, _expression: &TableExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_clone_expression(&mut self, _expression: &CloneExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_clone_expression(&mut self, _expression: &CloneExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_resume_expression(&mut self, _expression: &ResumeExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_resume_expression(&mut self, _expression: &ResumeExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_delete_expression(&mut self, _expression: &DeleteExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_delete_expression(&mut self, _expression: &DeleteExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_array_expression(&mut self, _expression: &ArrayExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_array_expression(&mut self, _expression: &ArrayExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_array_access_expression(
        &mut self,
        _expression: &ArrayAccessExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_array_access_expression(
        &mut self,
        _expression: &ArrayAccessExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_grouping_expression(&mut self, _expression: &GroupingExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_grouping_expression(&mut self, _expression: &GroupingExpression) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_identifier_expression(
        &mut self,
        _expression: &IdentifierExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_identifier_expression(
        &mut self,
        _expression: &IdentifierExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_null_literal_expression(
        &mut self,
        _expression: &NullLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_null_literal_expression(
        &mut self,
        _expression: &NullLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_boolean_literal_expression(
        &mut self,
        _expression: &BooleanLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_boolean_literal_expression(
        &mut self,
        _expression: &BooleanLiteralExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_function_call_expression(
        &mut self,
        _expression: &FunctionCallExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_function_call_expression(
        &mut self,
        _expression: &FunctionCallExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_scope_resolution_expression(
        &mut self,
        _expression: &ScopeResolutionExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_scope_resolution_expression(
        &mut self,
        _expression: &ScopeResolutionExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_member_access_expression(
        &mut self,
        _expression: &MemberAccessExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_member_access_expression(
        &mut self,
        _expression: &MemberAccessExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_ternary_operator_expression(
        &mut self,
        _expression: &TernaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_ternary_operator_expression(
        &mut self,
        _expression: &TernaryOperatorExpression,
    ) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_function_expression(&mut self, _expression: &FunctionDeclaration) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_function_expression(&mut self, _expression: &FunctionDeclaration) -> AstVisitorResult {
        AstVisitorResult::Continue
    }
}

pub fn visit<'a>(statements: &'a Statements, visitor: &mut impl AstVisitor) -> AstVisitorResult {
    visit_statements(statements, visitor)
}

fn visit_statements<'a>(
    statements: &'a Statements,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    for statement in &statements.statements {
        if visit_statement(statement, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    AstVisitorResult::Continue
}

fn visit_statement<'a>(
    statement: &'a Statement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    match statement {
        Statement::Block(stat) => visit_block_statement(stat, visitor),
        Statement::If(stat) => visit_if_statement(stat, visitor),
        Statement::While(stat) => visit_while_statement(stat, visitor),
        Statement::DoWhile(stat) => visit_do_while_statement(stat, visitor),
        Statement::Switch(stat) => visit_switch_statement(stat, visitor),
        Statement::For(stat) => visit_for_statement(stat, visitor),
        Statement::ForEach(stat) => visit_foreach_statement(stat, visitor),
        Statement::TryCatch(stat) => visit_try_catch_statement(stat, visitor),
        Statement::Break(stat) => visit_break_statement(stat, visitor),
        Statement::Continue(stat) => visit_continue_statement(stat, visitor),
        Statement::Return(stat) => visit_return_statement(stat, visitor),
        Statement::Yield(stat) => visit_yield_statement(stat, visitor),
        Statement::Throw(stat) => visit_throw_statement(stat, visitor),
        Statement::Expression(stat) => visit_expression_statement(stat, visitor),
        Statement::Const(stat) => visit_const_statement(stat, visitor),
        Statement::Local(stat) => visit_local_statement(stat, visitor),
        Statement::FunctionDeclaration(stat) => visit_function_declaration(stat, visitor),
        Statement::Class(stat) => visit_class_definition(stat, visitor),
        Statement::Enum(stat) => visit_enum_statement(stat, visitor),
        Statement::Comment(_) => return AstVisitorResult::Continue,
    }
}

fn visit_block_statement<'a>(
    statement: &'a BlockStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_block_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }
    if visit_statements(&statement.statements, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_block_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_if_statement<'a>(
    statement: &'a IfStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_if_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.condition, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.if_true, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(else_branch) = &statement.if_false {
        if visit_statement(else_branch, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_if_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_while_statement<'a>(
    statement: &'a WhileStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_while_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.condition, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_while_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_do_while_statement<'a>(
    statement: &'a DoWhileStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_do_while_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.condition, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_do_while_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_switch_statement<'a>(
    statement: &'a SwitchStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_switch_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for case in &statement.cases {
        if visit_expression(&case.expression, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }

        if visit_statements(&case.statements, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if let Some(def) = &statement.default {
        if visit_statements(def, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_switch_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_for_statement<'a>(
    statement: &'a ForStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_for_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(initializer) = &statement.initialization {
        if visit_statement(initializer, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if let Some(condition) = &statement.condition {
        if visit_expression(condition, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if let Some(increment) = &statement.increment {
        if visit_expression(increment, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visit_statement(&statement.statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_for_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_foreach_statement<'a>(
    statement: &'a ForEachStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_for_each_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(key) = &statement.key {
        if visit_expression(key, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visit_expression(&statement.value, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.iterable, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_for_each_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_try_catch_statement<'a>(
    statement: &'a TryCatchStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_try_catch_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.try_statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.enter_catch_clause(&statement.catch_variable, &statement.catch_statement)
        == AstVisitorResult::Break
    {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.catch_variable, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_statement(&statement.catch_statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_catch_clause(&statement.catch_variable, &statement.catch_statement)
        == AstVisitorResult::Break
    {
        return AstVisitorResult::Break;
    }

    if visitor.leave_try_catch_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_break_statement<'a>(
    statement: &'a BreakStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_break_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_break_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_continue_statement<'a>(
    statement: &'a ContinueStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_continue_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_continue_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_return_statement<'a>(
    statement: &'a ReturnStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_return_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(expression) = &statement.expression {
        if visit_expression(expression, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_return_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_yield_statement<'a>(
    statement: &'a YieldStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_yield_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(expression) = &statement.expression {
        if visit_expression(expression, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_yield_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_throw_statement<'a>(
    statement: &'a ThrowStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_throw_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_throw_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_expression_statement<'a>(
    statement: &'a ExpressionStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_expression_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_expression_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_const_statement<'a>(
    statement: &'a ConstStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_const_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.name, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_const_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_local_statement<'a>(
    statement: &'a LocalStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_local_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for initialization in statement.initializations.iter() {
        if let Some(expression) = &initialization.expression {
            if visit_expression(expression, visitor) == AstVisitorResult::Break {
                return AstVisitorResult::Break;
            }
        }
    }

    if visitor.leave_local_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_function_declaration<'a>(
    declaration: &'a FunctionDeclaration,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_function_declaration(declaration) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(name) = &declaration.name {
        if visit_expression(name, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    for parameter in &declaration.parameters {
        if visit_expression(parameter, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visit_statement(&declaration.statement, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_function_declaration(declaration) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_class_definition<'a>(
    definition: &'a ClassDefinition,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_class_definition(definition) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(name) = &definition.name {
        if visit_expression(name, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if let Some(extends) = &definition.extends {
        if visit_expression(extends, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    for member in &definition.members {
        match member {
            ClassMemberDeclaration::MethodDeclaration(declaration) => {
                if visit_function_declaration(declaration, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
            ClassMemberDeclaration::ConstructorDeclaration(declaration) => {
                if visit_function_declaration(declaration, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
            ClassMemberDeclaration::FieldDeclaration(field) => {
                if visit_expression(&field.name, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }

                if visit_expression(&field.expression, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
        }
    }

    if visitor.leave_class_definition(definition) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_enum_statement<'a>(
    statement: &'a EnumStatement,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_enum_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&statement.name, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for enumeration in &statement.enumerations {
        if visit_expression(&enumeration.name, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }

        if let Some(value) = &enumeration.value {
            if visit_expression(value, visitor) == AstVisitorResult::Break {
                return AstVisitorResult::Break;
            }
        }
    }

    if visitor.leave_enum_statement(statement) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_expression<'a>(
    expression: &'a Expression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    match expression {
        Expression::UnaryOperator(unary_operator) => {
            visit_unary_operator_expression(unary_operator, visitor)
        }
        Expression::PostfixUnaryOperator(postfix_unary_operator) => {
            visit_postfix_unary_operator_expression(postfix_unary_operator, visitor)
        }
        Expression::BinaryOperator(binary_operator) => {
            visit_binary_operator_expression(binary_operator, visitor)
        }
        Expression::Spread(spread) => visit_spread_expression(spread, visitor),
        Expression::StringLiteral(string_literal) => {
            visit_string_literal_expression(string_literal, visitor)
        }
        Expression::MutliLineStringLiteral(multi_line_string_literal) => {
            visit_multi_line_string_literal_expression(multi_line_string_literal, visitor)
        }
        Expression::FloatLiteral(float_literal) => {
            visit_float_literal_expression(float_literal, visitor)
        }
        Expression::IntegerLiteral(integer_literal) => {
            visit_integer_literal_expression(integer_literal, visitor)
        }
        Expression::Table(table) => visit_table_expression(table, visitor),
        Expression::Clone(clone) => visit_clone_expression(clone, visitor),
        Expression::Resume(resume) => visit_resume_expression(resume, visitor),
        Expression::Delete(delete) => visit_delete_expression(delete, visitor),
        Expression::Array(array) => visit_array_expression(array, visitor),
        Expression::ArrayAccess(array_access) => {
            visit_array_access_expression(array_access, visitor)
        }
        Expression::Grouping(grouping) => visit_grouping_expression(grouping, visitor),
        Expression::Identifier(identifier) => visit_identifier_expression(identifier, visitor),
        Expression::NullLiteral(null_literal) => {
            visit_null_literal_expression(null_literal, visitor)
        }
        Expression::BooleanLiteral(boolean_literal) => {
            visit_boolean_literal_expression(boolean_literal, visitor)
        }
        Expression::FunctionCall(function_call) => {
            visit_function_call_expression(function_call, visitor)
        }
        Expression::ScopeResolution(scope_resolution) => {
            visit_scope_resolution_expression(scope_resolution, visitor)
        }
        Expression::MemberAccess(member_access) => {
            visit_member_access_expression(member_access, visitor)
        }
        Expression::Class(class) => visit_class_definition(class, visitor),
        Expression::TernaryOperator(ternary_operator) => {
            visit_ternary_operator_expression(ternary_operator, visitor)
        }
        Expression::Function(function) => visit_function_declaration(function, visitor),
    }
}

fn visit_unary_operator_expression<'a>(
    expression: &'a UnaryOperatorExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_unary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_unary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_postfix_unary_operator_expression<'a>(
    expression: &'a PostfixUnaryOperatorExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_postfix_unary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_postfix_unary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_binary_operator_expression<'a>(
    expression: &'a BinaryOperatorExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_binary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.left, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.right, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_binary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_spread_expression<'a>(
    expression: &'a SpreadExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_spread_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_spread_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_string_literal_expression<'a>(
    expression: &'a StringLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_string_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_string_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_multi_line_string_literal_expression<'a>(
    expression: &'a MutliLineStringLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_multiline_string_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_multiline_string_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_float_literal_expression<'a>(
    expression: &'a FloatLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_float_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_float_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_integer_literal_expression<'a>(
    expression: &'a IntegerLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_integer_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_integer_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_table_expression<'a>(
    expression: &'a TableExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_table_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for entry in &expression.entries {
        match entry {
            TableEntry::Field(f) => {
                if visit_expression(&f.name, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }

                if visit_expression(&f.expression, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
            TableEntry::Function(f) => {
                if visit_function_declaration(&f.function, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
            TableEntry::FieldWithExpressionKey(f) => {
                if visit_expression(&f.key, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }

                if visit_expression(&f.expression, visitor) == AstVisitorResult::Break {
                    return AstVisitorResult::Break;
                }
            }
        }
    }

    if visitor.leave_table_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_clone_expression<'a>(
    expression: &'a CloneExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_clone_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_clone_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_array_expression<'a>(
    expression: &'a ArrayExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_array_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for element in &expression.elements {
        if visit_expression(element, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_array_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_resume_expression<'a>(
    expression: &'a ResumeExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_resume_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_resume_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_delete_expression<'a>(
    expression: &'a DeleteExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_delete_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_delete_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_identifier_expression<'a>(
    expression: &'a IdentifierExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_identifier_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_identifier_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_array_access_expression<'a>(
    expression: &'a ArrayAccessExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_array_access_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.array, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.index, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_array_access_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_grouping_expression<'a>(
    expression: &'a GroupingExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_grouping_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if let Some(expr) = &expression.expression {
        if visit_expression(expr, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_grouping_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_null_literal_expression<'a>(
    expression: &'a NullLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_null_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_null_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_boolean_literal_expression<'a>(
    expression: &'a BooleanLiteralExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_boolean_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_boolean_literal_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_function_call_expression<'a>(
    expression: &'a FunctionCallExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_function_call_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.function, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    for arg in &expression.arguments {
        if visit_expression(arg, visitor) == AstVisitorResult::Break {
            return AstVisitorResult::Break;
        }
    }

    if visitor.leave_function_call_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_scope_resolution_expression<'a>(
    expression: &'a ScopeResolutionExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_scope_resolution_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_scope_resolution_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_member_access_expression<'a>(
    expression: &'a MemberAccessExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_member_access_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.expression, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_member_access_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

fn visit_ternary_operator_expression<'a>(
    expression: &'a TernaryOperatorExpression,
    visitor: &mut impl AstVisitor,
) -> AstVisitorResult {
    if visitor.enter_ternary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.condition, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.if_true, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visit_expression(&expression.if_false, visitor) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    if visitor.leave_ternary_operator_expression(expression) == AstVisitorResult::Break {
        return AstVisitorResult::Break;
    }

    AstVisitorResult::Continue
}

#[cfg(test)]
mod test {
    use crate::{squirrel_lexer::Token, squirrel_parser::Parser};

    use super::{visit, AstVisitor, AstVisitorResult};

    struct IdentifierVisitor {
        identifiers: Vec<Token>,
    }

    impl AstVisitor for IdentifierVisitor {
        fn enter_identifier_expression(
            &mut self,
            expression: &crate::squirrel_ast::IdentifierExpression,
        ) -> AstVisitorResult {
            self.identifiers.push(expression.token.clone());

            return AstVisitorResult::Continue;
        }
    }

    #[test]
    fn test_identifiers_extraction() {
        let input = "local a = b.c.d + c + 10;";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let mut visitor = IdentifierVisitor {
            identifiers: Vec::new(),
        };

        visit(&statements, &mut visitor);

        println!("{:?}", visitor.identifiers);
        assert_eq!(visitor.identifiers.len(), 2);
    }
}
