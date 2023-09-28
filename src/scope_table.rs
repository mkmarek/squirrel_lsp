use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::{
    ast_visitor::{visit, AstVisitor, AstVisitorResult},
    squirrel_ast::{
        ClassDefinition, ClassMemberDeclaration, Expression, IdentifierExpression, Statement,
        Statements, TableEntry, TableExpression,
    },
    squirrel_lexer::{Location, Operator},
};

#[derive(Debug)]
pub struct ScopeTable {
    scopes: Vec<Scope>,
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub variables: Vec<VariableDeclaration>,
    pub variable_usage: Vec<VariableUsage>,
}

impl Scope {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            children: Vec::new(),
            variables: Vec::new(),
            variable_usage: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Local,
    Const,
    Function,
    ClassMember,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub sequence_number: usize,
    pub kind: VariableDeclarationKind,
    pub name: String,
    pub value: Option<Expression>,
    pub scope: usize,
    pub from: Location,
    pub to: Location,
}

#[derive(Debug)]
pub enum DeclarationType {
    Variable(usize, usize),
    This,
    Base,
}

#[derive(Debug)]
pub struct VariableUsage {
    pub name: String,
    pub declaration: Option<DeclarationType>,
    pub from: Location,
    pub to: Location,
}

struct ScopeTableVisitor<'a> {
    scope_table: &'a mut ScopeTable,
    scope_stack: Vec<usize>,
}

impl ScopeTableVisitor<'_> {
    fn spawn_child_scope(&mut self) {
        let scope = self.scope_table.scopes.len();
        self.scope_table
            .scopes
            .push(Scope::new(Some(*self.scope_stack.last().unwrap())));
        self.scope_table.scopes[*self.scope_stack.last().unwrap()]
            .children
            .push(scope);
        self.scope_stack.push(scope);
    }
}

impl AstVisitor for ScopeTableVisitor<'_> {
    fn enter_table_expression(&mut self, table: &TableExpression) -> AstVisitorResult {
        self.spawn_child_scope();

        for entry in &table.entries {
            match entry {
                TableEntry::Field(f) => {
                    if let Expression::Identifier(ident) = &f.name {
                        let scope = self.scope_stack.last().unwrap();
                        let sequence_number = self.scope_table.scopes[*scope].variables.len();
                        self.scope_table.scopes[*scope]
                            .variables
                            .push(VariableDeclaration {
                                kind: VariableDeclarationKind::ClassMember,
                                sequence_number,
                                name: ident.token.to_string(),
                                value: Some(f.expression.clone()),
                                scope: *scope,
                                from: ident.from.clone(),
                                to: ident.to.clone(),
                            });
                    }
                }
                TableEntry::Function(f) => {
                    if let Some(Expression::Identifier(ident)) = &f.function.name {
                        let scope = self.scope_stack.last().unwrap();
                        let sequence_number = self.scope_table.scopes[*scope].variables.len();
                        self.scope_table.scopes[*scope]
                            .variables
                            .push(VariableDeclaration {
                                kind: VariableDeclarationKind::ClassMember,
                                sequence_number,
                                name: ident.token.to_string(),
                                value: None,
                                scope: *scope,
                                from: ident.from.clone(),
                                to: ident.to.clone(),
                            });
                    }
                }
                TableEntry::FieldWithExpressionKey(_) => {}
            }
        }

        AstVisitorResult::Continue
    }

    fn leave_table_expression(&mut self, _table: &TableExpression) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn enter_catch_clause(
        &mut self,
        error_variable: &Expression,
        _catch_clause: &Statement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();

        let scope = self.scope_stack.last().unwrap();
        let sequence_number = self.scope_table.scopes[*scope].variables.len();
        if let Expression::Identifier(ident) = error_variable {
            self.scope_table.scopes[*scope]
                .variables
                .push(VariableDeclaration {
                    kind: VariableDeclarationKind::Local,
                    sequence_number,
                    name: ident.token.to_string(),
                    value: None,
                    scope: *scope,
                    from: ident.from.clone(),
                    to: ident.to.clone(),
                });
        }

        AstVisitorResult::Continue
    }

    fn leave_catch_clause(
        &mut self,
        _catch_variable: &Expression,
        _catch_clause: &Statement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn enter_class_definition(
        &mut self,
        class: &ClassDefinition,
    ) -> crate::ast_visitor::AstVisitorResult {
        let scope = self.scope_stack.last().unwrap();

        if let Some(Expression::Identifier(ident)) = &class.name {
            let variable = VariableDeclaration {
                kind: VariableDeclarationKind::Local,
                sequence_number: self.scope_table.scopes[*scope].variables.len(),
                name: ident.token.to_string(),
                value: None,
                scope: *scope,
                from: class.from.clone(),
                to: class.to.clone(),
            };

            self.scope_table.scopes[*scope].variables.push(variable);
        }

        self.spawn_child_scope();

        let scope = self.scope_stack.last().unwrap();
        for member in &class.members {
            match member {
                ClassMemberDeclaration::FieldDeclaration(field) => {
                    if let Expression::Identifier(name) = &field.name {
                        let variable = VariableDeclaration {
                            kind: VariableDeclarationKind::ClassMember,
                            sequence_number: self.scope_table.scopes[*scope].variables.len(),
                            name: name.token.to_string(),
                            value: Some(field.expression.clone()),
                            scope: *scope,
                            from: name.from.clone(),
                            to: name.to.clone(),
                        };

                        self.scope_table.scopes[*scope].variables.push(variable);
                    }
                }
                _ => {}
            }
        }

        AstVisitorResult::Continue
    }

    fn leave_class_definition(&mut self, _statement: &ClassDefinition) -> AstVisitorResult {
        let scope = self.scope_stack.last().unwrap();
        for member in &mut self.scope_table.scopes[*scope].variables {
            member.kind = VariableDeclarationKind::ClassMember;
        }

        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn enter_function_declaration(
        &mut self,
        function: &crate::squirrel_ast::FunctionDeclaration,
    ) -> crate::ast_visitor::AstVisitorResult {
        let scope = self.scope_stack.last().unwrap();

        if let Some(Expression::Identifier(ident)) = &function.name {
            let variable = VariableDeclaration {
                kind: VariableDeclarationKind::Function,
                sequence_number: self.scope_table.scopes[*scope].variables.len(),
                name: ident.token.to_string(),
                value: None,
                scope: *scope,
                from: function.from.clone(),
                to: function.to.clone(),
            };

            self.scope_table.scopes[*scope].variables.push(variable);
        }

        self.spawn_child_scope();
        let scope = self.scope_stack.last().unwrap();

        for parameter in &function.parameters {
            if let Expression::Identifier(ident) = parameter {
                let variable = VariableDeclaration {
                    kind: VariableDeclarationKind::Local,
                    sequence_number: self.scope_table.scopes[*scope].variables.len(),
                    name: ident.token.to_string(),
                    value: None,
                    scope: *scope,
                    from: ident.from.clone(),
                    to: ident.to.clone(),
                };

                self.scope_table.scopes[*scope].variables.push(variable);
            } else if let Expression::BinaryOperator(op) = parameter {
                if op.operator == Operator::Assign {
                    if let Expression::Identifier(ident) = &op.left {
                        let variable = VariableDeclaration {
                            kind: VariableDeclarationKind::Local,
                            sequence_number: self.scope_table.scopes[*scope].variables.len(),
                            name: ident.token.to_string(),
                            value: Some(op.right.clone()),
                            scope: *scope,
                            from: ident.from.clone(),
                            to: ident.to.clone(),
                        };

                        self.scope_table.scopes[*scope].variables.push(variable);
                    }
                }
            }
        }

        AstVisitorResult::Continue
    }

    fn leave_function_declaration(
        &mut self,
        _function_definition: &crate::squirrel_ast::FunctionDeclaration,
    ) -> crate::ast_visitor::AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn enter_local_statement(
        &mut self,
        statement: &crate::squirrel_ast::LocalStatement,
    ) -> AstVisitorResult {
        let scope = self.scope_stack.last().unwrap();
        let mut variables = Vec::new();

        for initialization in &statement.initializations {
            let variable = VariableDeclaration {
                kind: VariableDeclarationKind::Local,
                sequence_number: self.scope_table.scopes[*scope].variables.len(),
                name: initialization.name.clone(),
                value: initialization.expression.clone(),
                scope: *scope,
                from: initialization.from.clone(),
                to: initialization.to.clone(),
            };

            variables.push(variable);
        }

        self.scope_table.scopes[*scope]
            .variables
            .append(&mut variables);

        AstVisitorResult::Continue
    }

    fn enter_block_statement(
        &mut self,
        _statement: &crate::squirrel_ast::BlockStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_if_statement(
        &mut self,
        _statement: &crate::squirrel_ast::IfStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_while_statement(
        &mut self,
        _statement: &crate::squirrel_ast::WhileStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_do_while_statement(
        &mut self,
        _statement: &crate::squirrel_ast::DoWhileStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_switch_statement(
        &mut self,
        _statement: &crate::squirrel_ast::SwitchStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_for_statement(
        &mut self,
        _statement: &crate::squirrel_ast::ForStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn enter_for_each_statement(
        &mut self,
        statement: &crate::squirrel_ast::ForEachStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();

        let scope = self.scope_stack.last().unwrap();

        if let Some(id) = &statement.key {
            if let Expression::Identifier(ident) = id {
                let variable = VariableDeclaration {
                    kind: VariableDeclarationKind::Local,
                    sequence_number: self.scope_table.scopes[*scope].variables.len(),
                    name: ident.token.to_string(),
                    value: None,
                    scope: *scope,
                    from: ident.from.clone(),
                    to: ident.to.clone(),
                };

                self.scope_table.scopes[*scope].variables.push(variable);
            }
        }

        if let Expression::Identifier(el) = &statement.value {
            let variable = VariableDeclaration {
                kind: VariableDeclarationKind::Local,
                sequence_number: self.scope_table.scopes[*scope].variables.len(),
                name: el.token.to_string(),
                value: None,
                scope: *scope,
                from: el.from.clone(),
                to: el.to.clone(),
            };

            self.scope_table.scopes[*scope].variables.push(variable);
        }

        AstVisitorResult::Continue
    }

    fn enter_const_statement(
        &mut self,
        statement: &crate::squirrel_ast::ConstStatement,
    ) -> AstVisitorResult {
        let scope = self.scope_stack.last().unwrap();

        if let Expression::Identifier(name) = &statement.name {
            let variable = VariableDeclaration {
                kind: VariableDeclarationKind::Const,
                sequence_number: self.scope_table.scopes[*scope].variables.len(),
                name: name.token.to_string(),
                value: Some(statement.expression.clone()),
                scope: *scope,
                from: name.from.clone(),
                to: name.to.clone(),
            };

            self.scope_table.scopes[*scope].variables.push(variable);
        }
        AstVisitorResult::Continue
    }

    fn enter_enum_statement(
        &mut self,
        _statement: &crate::squirrel_ast::EnumStatement,
    ) -> AstVisitorResult {
        self.spawn_child_scope();
        AstVisitorResult::Continue
    }

    fn leave_block_statement(
        &mut self,
        _statement: &crate::squirrel_ast::BlockStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_if_statement(
        &mut self,
        _statement: &crate::squirrel_ast::IfStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_while_statement(
        &mut self,
        _statement: &crate::squirrel_ast::WhileStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_do_while_statement(
        &mut self,
        _statement: &crate::squirrel_ast::DoWhileStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_switch_statement(
        &mut self,
        _statement: &crate::squirrel_ast::SwitchStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_for_statement(
        &mut self,
        _statement: &crate::squirrel_ast::ForStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_for_each_statement(
        &mut self,
        _statement: &crate::squirrel_ast::ForEachStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn leave_enum_statement(
        &mut self,
        _statement: &crate::squirrel_ast::EnumStatement,
    ) -> AstVisitorResult {
        self.scope_stack.pop();
        AstVisitorResult::Continue
    }

    fn enter_identifier_expression(
        &mut self,
        identifier: &IdentifierExpression,
    ) -> crate::ast_visitor::AstVisitorResult {
        let declaration_type = match &identifier.token {
            crate::squirrel_lexer::Token::Identifier(ident) => self
                .scope_table
                .find_variable_declaration(&ident, *self.scope_stack.last().unwrap()),
            crate::squirrel_lexer::Token::Keyword(keyword) => match keyword {
                crate::squirrel_lexer::Keyword::Base => Some(DeclarationType::Base),
                crate::squirrel_lexer::Keyword::This => Some(DeclarationType::This),
                _ => return crate::ast_visitor::AstVisitorResult::Continue,
            },
            _ => return crate::ast_visitor::AstVisitorResult::Continue,
        };

        let variable_usage = VariableUsage {
            name: identifier.token.to_string(),
            declaration: declaration_type,
            from: identifier.from.clone(),
            to: identifier.to.clone(),
        };

        let scope = self.scope_stack.last().unwrap();
        self.scope_table.scopes[*scope]
            .variable_usage
            .push(variable_usage);

        crate::ast_visitor::AstVisitorResult::Continue
    }
}

impl ScopeTable {
    pub fn new(statements: &Statements) -> Self {
        let mut table = Self {
            scopes: vec![Scope::new(None)],
        };

        {
            let mut visitor = ScopeTableVisitor {
                scope_table: &mut table,
                scope_stack: vec![0],
            };

            visit(statements, &mut visitor);
        }

        table
    }

    pub fn get_declaration(&self, decl: &DeclarationType) -> Option<&VariableDeclaration> {
        match decl {
            DeclarationType::Base => None,
            DeclarationType::This => None,
            DeclarationType::Variable(scope, sequence_number) => {
                Some(&self.scopes[*scope].variables[*sequence_number])
            }
        }
    }

    pub fn find_variable_usage_by_location(
        &self,
        line: usize,
        character: usize,
    ) -> Option<&VariableUsage> {
        for scope in &self.scopes {
            for variable_usage in &scope.variable_usage {
                if variable_usage.from.line == line
                    && variable_usage.from.linechar <= character
                    && variable_usage.to.line == line
                    && variable_usage.to.linechar >= character
                {
                    return Some(variable_usage);
                }
            }
        }

        None
    }

    pub fn find_variable_declaration_by_location(
        &self,
        line: usize,
        character: usize,
    ) -> Option<&VariableDeclaration> {
        for scope in &self.scopes {
            for variable_declaration in &scope.variables {
                if variable_declaration.from.line == line
                    && variable_declaration.from.linechar <= character
                    && variable_declaration.to.line == line
                    && variable_declaration.to.linechar >= character
                {
                    return Some(variable_declaration);
                }
            }
        }

        None
    }

    pub fn validate_variables(&self) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for scope in &self.scopes {
            for variable in &scope.variables {
                if variable.kind != VariableDeclarationKind::Const
                    && variable.kind != VariableDeclarationKind::Local
                {
                    continue;
                }

                if variable.name.starts_with("_") {
                    continue;
                }

                let usages = self.find_variable_usages(&variable.name, variable.scope);

                if usages.is_empty() {
                    diagnostics.push(Diagnostic::new(
                        Range::new(
                            Position::new(variable.from.line as u32, variable.from.linechar as u32),
                            Position::new(variable.to.line as u32, variable.to.linechar as u32),
                        ),
                        Some(DiagnosticSeverity::WARNING),
                        None,
                        None,
                        format!("Unused variable '{}'", variable.name),
                        None,
                        None,
                    ));
                }
            }

            for usage in &scope.variable_usage {
                if usage.declaration.is_none() {
                    diagnostics.push(Diagnostic::new_simple(
                        Range::new(
                            Position::new(usage.from.line as u32, usage.from.linechar as u32),
                            Position::new(usage.to.line as u32, usage.to.linechar as u32),
                        ),
                        format!("Variable '{}' is not declared", usage.name),
                    ));
                }
            }
        }

        diagnostics
    }

    pub fn find_variable_usages(&self, name: &str, scope: usize) -> Vec<&VariableUsage> {
        let mut usages = Vec::new();

        for usage in &self.scopes[scope].variable_usage {
            if usage.name == name {
                usages.push(usage);
            }
        }

        for child in &self.scopes[scope].children {
            usages.append(&mut self.find_variable_usages(name, *child));
        }

        usages
    }

    pub fn find_variable_declaration(
        &self,
        name: &str,
        mut current_scope: usize,
    ) -> Option<DeclarationType> {
        loop {
            let scope = &self.scopes[current_scope];

            for variable in &scope.variables {
                if variable.name == name {
                    return Some(DeclarationType::Variable(
                        variable.scope,
                        variable.sequence_number,
                    ));
                }
            }

            if let Some(parent) = scope.parent {
                current_scope = parent;
            } else {
                break;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{scope_table::ScopeTable, squirrel_parser::Parser};

    #[test]
    fn test_single_scope_declaration_and_usage() {
        let input = "
local a = 1;
local b = 2;

a + b;
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.scopes.len(), 1);
        assert_eq!(scope_table.scopes[0].variables.len(), 2);
        assert_eq!(scope_table.scopes[0].variable_usage.len(), 2);

        assert_eq!(scope_table.scopes[0].variables[0].name, "a");
        assert_eq!(scope_table.scopes[0].variables[1].name, "b");

        assert_eq!(scope_table.scopes[0].variable_usage[0].name, "a");
        assert_eq!(
            scope_table.scopes[0].variable_usage[0]
                .declaration
                .is_some(),
            true
        );
        assert_eq!(scope_table.scopes[0].variable_usage[1].name, "b");
        assert_eq!(
            scope_table.scopes[0].variable_usage[1]
                .declaration
                .is_some(),
            true
        );
    }

    #[test]
    fn test_nested_scope_in_block() {
        let input = "
local a = 1;
{
    local b = 2;
    a + b;
}
a + b;
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.scopes.len(), 2);
        assert_eq!(scope_table.scopes[0].variables.len(), 1);
        assert_eq!(scope_table.scopes[0].variable_usage.len(), 2);

        assert_eq!(scope_table.scopes[0].variables[0].name, "a");
        assert_eq!(scope_table.scopes[0].variable_usage[0].name, "a");
        assert_eq!(
            scope_table.scopes[0].variable_usage[0]
                .declaration
                .is_some(),
            true
        );
        assert_eq!(scope_table.scopes[0].variable_usage[1].name, "b");
        assert_eq!(
            scope_table.scopes[0].variable_usage[1]
                .declaration
                .is_some(),
            false
        );

        assert_eq!(scope_table.scopes[1].variables.len(), 1);
        assert_eq!(scope_table.scopes[1].variable_usage.len(), 2);

        assert_eq!(scope_table.scopes[1].variables[0].name, "b");
        assert_eq!(scope_table.scopes[1].variable_usage[0].name, "a");
        assert_eq!(
            scope_table.scopes[1].variable_usage[0]
                .declaration
                .is_some(),
            true
        );
        assert_eq!(scope_table.scopes[1].variable_usage[1].name, "b");
        assert_eq!(
            scope_table.scopes[1].variable_usage[1]
                .declaration
                .is_some(),
            true
        );
    }

    #[test]
    fn test_validate_variables_local_variables() {
        let input = "
local a = 1;
{
    local b = 2;
    a + b;
}
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_function() {
        let input = "
function foo(a, b, c) {
    return a + b + c;
}
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_class() {
        let input = "
class Foo {
    function stuff(a, b) {
        return a + b; 
    }
}

local foo = Foo();
foo.stuff(1, 2);
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_foreach() {
        let input = "
foreach (i, v in [1, 2, 3]) {
    ::print(i, v);
}
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_for() {
        let input = "
for (local i = 0; i < 10; i += 1) {
    ::print(i);
}
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        println!("{:#?}", scope_table.validate_variables());
        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_try_catch() {
        let input = "
try {
    ::print(\"try\");
} catch (e) {
    ::print(\"catch\", e);
}
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_implicit_this_in_class() {
        let input = "
class Foo {
    bar = 1;
    function stuff() {
        return bar;
    }
}

local foo = Foo();
foo.stuff();
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_implicit_this_in_class_function() {
        let input = "
class Foo {
    function bar() {
        return 123;
    }
    function stuff() {
        return bar();
    }
}

local foo = Foo();
foo.stuff();
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_function_in_table() {
        let input = "
local tbl = {
    foo = function(a, b, c) {
        return a + b + c;
    }
};

tbl.foo();
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }

    #[test]
    fn test_validate_variables_self_calling() {
        let input = "
local tbl = {
    foo = function(a, b, c) {
        return foo(a, b, c);
    }
};

tbl.foo();
";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        let scope_table = ScopeTable::new(&statements);

        assert_eq!(scope_table.validate_variables().len(), 0);
    }
}
