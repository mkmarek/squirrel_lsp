use crate::{
    squirrel_ast::{
        ArrayAccessExpression, ArrayExpression, BinaryOperatorExpression, BlockStatement,
        BooleanLiteralExpression, BreakStatement, ClassDefinition, ClassMemberDeclaration,
        CloneExpression, ConstStatement, ContinueStatement, DeleteExpression, DoWhileStatement,
        EnumStatement, Expression, ExpressionStatement, FloatLiteralExpression, ForEachStatement,
        ForStatement, FunctionCallExpression, FunctionDeclaration, GroupingExpression,
        IdentifierExpression, IfStatement, IntegerLiteralExpression, LocalStatement,
        MemberAccessExpression, MutliLineStringLiteralExpression, NullLiteralExpression,
        PostfixUnaryOperatorExpression, ResumeExpression, ReturnStatement,
        ScopeResolutionExpression, SpreadExpression, Statement, Statements,
        StringLiteralExpression, SwitchStatement, TableExpression, TernaryOperatorExpression,
        ThrowStatement, TryCatchStatement, UnaryOperatorExpression, WhileStatement, YieldStatement,
    },
    squirrel_lexer::{Keyword, Lexer, Operator, Token},
};

pub enum IndentationType {
    Spaces,
    Tabs,
}

pub struct PrinterSettings {
    pub indentation: IndentationType,
    pub indentation_size: usize,
}

pub struct Printer {
    buffer: String,
    settings: PrinterSettings,
    indentation_level: usize,
}

impl Printer {
    pub fn new(buffer_capacity: usize, settings: PrinterSettings) -> Self {
        Self {
            buffer: String::with_capacity(buffer_capacity),
            settings,
            indentation_level: 0,
        }
    }

    pub fn print(&mut self, ast: &Statements) -> String {
        self.print_statements(ast);

        self.buffer.drain(..).collect()
    }

    fn print_token(&mut self, token: Token) {
        self.buffer.push_str(&token.to_string());
    }

    fn print_space(&mut self) {
        self.buffer.push(' ');
    }

    fn print_tab(&mut self) {
        self.buffer.push('\t');
    }

    fn newline_and_indent(&mut self) {
        self.buffer.push('\n');
        for _ in 0..self.indentation_level {
            match self.settings.indentation {
                IndentationType::Spaces => {
                    for _ in 0..self.settings.indentation_size {
                        self.print_space();
                    }
                }
                IndentationType::Tabs => {
                    self.print_tab();
                }
            }
        }
    }

    fn print_statements(&mut self, ast: &Statements) {
        for (i, statement) in ast.statements.iter().enumerate() {
            self.print_statement(statement);
            if i < ast.statements.len() - 1 {
                let line_difference = (ast.statements[i + 1].get_from().line
                    - statement.get_from().line)
                    .max(1)
                    .min(2);

                for _ in 0..line_difference {
                    self.newline_and_indent();
                }
            }
        }
    }

    fn print_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Block(stat) => self.print_block(stat),
            Statement::If(stat) => self.print_if(stat),
            Statement::While(stat) => self.print_while(stat),
            Statement::DoWhile(stat) => self.print_do_while(stat),
            Statement::Switch(stat) => self.print_switch(stat),
            Statement::For(stat) => self.print_for(stat),
            Statement::ForEach(stat) => self.print_for_each(stat),
            Statement::TryCatch(stat) => self.print_try_catch(stat),
            Statement::Break(stat) => self.print_break(stat),
            Statement::Continue(stat) => self.print_continue(stat),
            Statement::Return(stat) => self.print_return(stat),
            Statement::Yield(stat) => self.print_yield(stat),
            Statement::Throw(stat) => self.print_throw(stat),
            Statement::Expression(stat) => self.print_expression_statement(stat),
            Statement::Const(stat) => self.print_const(stat),
            Statement::Local(stat) => self.print_local(stat),
            Statement::FunctionDeclaration(stat) => self.print_function_declaration(stat),
            Statement::Class(stat) => self.print_class_definition(stat),
            Statement::Enum(stat) => self.print_enum(stat),
            Statement::Comment(stat) => {
                for (i, comment) in stat.iter().enumerate() {
                    self.print_comment(comment);
                    if i < stat.len() - 1 {
                        self.newline_and_indent();
                    }
                }
            }
        }
    }

    fn print_block(&mut self, stat: &BlockStatement) {
        self.print_token(Token::LeftBrace);
        if stat.statements.statements.len() > 0 {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statements(&stat.statements);
            self.indentation_level -= 1;
        }
        self.newline_and_indent();
        self.print_token(Token::RightBrace);
    }

    fn print_if(&mut self, stat: &IfStatement) {
        self.print_token(Token::Keyword(Keyword::If));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        self.print_expression(&stat.condition);
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &stat.if_true {
            self.print_block(block);
            if stat.if_false.is_some() {
                self.print_space();
            }
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.if_true);
            self.indentation_level -= 1;
            if stat.if_false.is_some() {
                self.newline_and_indent();
            }
        }
        if let Some(else_block) = &stat.if_false {
            self.print_token(Token::Keyword(Keyword::Else));
            self.print_space();

            if let Statement::Block(block) = &else_block {
                self.print_block(block);
            } else {
                self.indentation_level += 1;
                self.newline_and_indent();
                self.print_statement(else_block);
                self.indentation_level -= 1;
            }
        }
    }

    fn print_while(&mut self, stat: &WhileStatement) {
        self.print_token(Token::Keyword(Keyword::While));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        self.print_expression(&stat.condition);
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &stat.statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.statement);
            self.indentation_level -= 1;
        }
    }

    fn print_do_while(&mut self, stat: &DoWhileStatement) {
        self.print_token(Token::Keyword(Keyword::Do));
        self.print_space();
        if let Statement::Block(block) = &stat.statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.statement);
            self.indentation_level -= 1;
            self.newline_and_indent();
        }
        self.print_space();
        self.print_token(Token::Keyword(Keyword::While));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        self.print_expression(&stat.condition);
        self.print_token(Token::RightParenthesis);
    }

    fn print_switch(&mut self, stat: &SwitchStatement) {
        self.print_token(Token::Keyword(Keyword::Switch));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        self.print_expression(&stat.expression);
        self.print_token(Token::RightParenthesis);
        self.print_space();
        self.print_token(Token::LeftBrace);
        self.indentation_level += 1;
        self.newline_and_indent();
        for (i, case) in stat.cases.iter().enumerate() {
            self.print_token(Token::Keyword(Keyword::Case));
            self.print_space();
            self.print_expression(&case.expression);
            self.print_token(Token::Colon);
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statements(&case.statements);
            self.indentation_level -= 1;
            if i < stat.cases.len() - 1 || stat.default.is_some() {
                self.newline_and_indent();
            }
        }
        if let Some(default) = &stat.default {
            self.print_token(Token::Keyword(Keyword::Default));
            self.print_token(Token::Colon);
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statements(&default);
            self.indentation_level -= 1;
        }
        self.indentation_level -= 1;
        self.newline_and_indent();
        self.print_token(Token::RightBrace);
    }

    fn print_for(&mut self, stat: &ForStatement) {
        self.print_token(Token::Keyword(Keyword::For));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        if let Some(initializer) = &stat.initialization {
            self.print_statement(initializer);
        }
        self.print_token(Token::Semicolon);
        if let Some(condition) = &stat.condition {
            self.print_space();
            self.print_expression(condition);
        }
        self.print_token(Token::Semicolon);
        if let Some(increment) = &stat.increment {
            self.print_space();
            self.print_expression(increment);
        }
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &stat.statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.statement);
            self.indentation_level -= 1;
        }
    }

    fn print_for_each(&mut self, stat: &ForEachStatement) {
        self.print_token(Token::Keyword(Keyword::Foreach));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        if let Some(key) = &stat.key {
            self.print_expression(key);
            self.print_token(Token::Operator(Operator::Comma));
            self.print_space();
        }
        self.print_expression(&stat.value);
        self.print_space();
        self.print_token(Token::Operator(Operator::In));
        self.print_space();
        self.print_expression(&stat.iterable);
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &stat.statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.statement);
            self.indentation_level -= 1;
        }
    }

    fn print_enum(&mut self, stat: &EnumStatement) {
        self.print_token(Token::Keyword(Keyword::Enum));
        self.print_space();
        self.print_expression(&stat.name);
        self.print_space();
        self.print_token(Token::LeftBrace);
        self.indentation_level += 1;
        self.newline_and_indent();
        for (i, enumeration) in stat.enumerations.iter().enumerate() {
            self.print_expression(&enumeration.name);
            if let Some(value) = &enumeration.value {
                self.print_space();
                self.print_token(Token::Operator(Operator::Assign));
                self.print_space();
                self.print_expression(value);
            }

            if i < stat.enumerations.len() - 1 {
                self.print_token(Token::Operator(Operator::Comma));
                self.newline_and_indent();
            }
        }
        self.indentation_level -= 1;
        self.newline_and_indent();
        self.print_token(Token::RightBrace);
    }

    fn print_try_catch(&mut self, stat: &TryCatchStatement) {
        self.print_token(Token::Keyword(Keyword::Try));
        self.print_space();
        if let Statement::Block(block) = &stat.try_statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.try_statement);
            self.indentation_level -= 1;
            self.newline_and_indent();
        }
        self.print_space();
        self.print_token(Token::Keyword(Keyword::Catch));
        self.print_space();
        self.print_token(Token::LeftParenthesis);
        self.print_expression(&stat.catch_variable);
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &stat.catch_statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&stat.catch_statement);
            self.indentation_level -= 1;
        }
    }

    fn print_break(&mut self, stat: &BreakStatement) {
        self.print_token(Token::Keyword(Keyword::Break));
    }

    fn print_continue(&mut self, stat: &ContinueStatement) {
        self.print_token(Token::Keyword(Keyword::Continue));
    }

    fn print_return(&mut self, stat: &ReturnStatement) {
        self.print_token(Token::Keyword(Keyword::Return));
        self.print_space();
        if let Some(expression) = &stat.expression {
            self.print_expression(expression);
        }
    }

    fn print_yield(&mut self, stat: &YieldStatement) {
        self.print_token(Token::Keyword(Keyword::Yield));
        self.print_space();
        if let Some(expression) = &stat.expression {
            self.print_expression(expression);
        }
    }

    fn print_throw(&mut self, stat: &ThrowStatement) {
        self.print_token(Token::Keyword(Keyword::Throw));
        self.print_space();
        self.print_expression(&stat.expression);
    }

    fn print_expression_statement(&mut self, stat: &ExpressionStatement) {
        self.print_expression(&stat.expression);
    }

    fn print_const(&mut self, stat: &ConstStatement) {
        self.print_token(Token::Keyword(Keyword::Const));
        self.print_space();
        self.print_expression(&stat.name);
        self.print_space();
        self.print_token(Token::Operator(Operator::Assign));
        self.print_space();
        self.print_expression(&stat.expression);
    }

    fn print_local(&mut self, stat: &LocalStatement) {
        self.print_token(Token::Keyword(Keyword::Local));
        self.print_space();
        for (i, init) in stat.initializations.iter().enumerate() {
            if i > 0 {
                self.print_token(Token::Operator(Operator::Comma));
                self.print_space();
            }
            self.print_token(Token::Identifier(init.name.clone()));
            if let Some(expr) = &init.expression {
                self.print_space();
                self.print_token(Token::Operator(Operator::Assign));
                self.print_space();
                self.print_expression(expr);
            }
        }
    }

    fn print_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::UnaryOperator(expr) => self.print_unary_operator(expr.as_ref()),
            Expression::PostfixUnaryOperator(expr) => {
                self.print_postfix_unary_operator(expr.as_ref())
            }
            Expression::BinaryOperator(expr) => self.print_binary_operator(expr.as_ref()),
            Expression::Spread(expr) => self.print_spread(expr.as_ref()),
            Expression::StringLiteral(expr) => self.print_string_literal(expr.as_ref()),
            Expression::MutliLineStringLiteral(expr) => {
                self.print_multi_line_string_literal(expr.as_ref())
            }
            Expression::FloatLiteral(expr) => self.print_float_literal(expr.as_ref()),
            Expression::IntegerLiteral(expr) => self.print_integer_literal(expr.as_ref()),
            Expression::Table(expr) => self.print_table(expr.as_ref()),
            Expression::Clone(expr) => self.print_clone(expr.as_ref()),
            Expression::Resume(expr) => self.print_resume(expr.as_ref()),
            Expression::Delete(expr) => self.print_delete(expr.as_ref()),
            Expression::Array(expr) => self.print_array(expr.as_ref()),
            Expression::ArrayAccess(expr) => self.print_array_access(expr.as_ref()),
            Expression::Grouping(expr) => self.print_grouping(expr.as_ref()),
            Expression::Identifier(expr) => self.print_identifier(expr.as_ref()),
            Expression::NullLiteral(expr) => self.print_null_literal(expr.as_ref()),
            Expression::BooleanLiteral(expr) => self.print_boolean_literal(expr.as_ref()),
            Expression::FunctionCall(expr) => self.print_function_call(expr.as_ref()),
            Expression::ScopeResolution(expr) => self.print_scope_resolution(expr.as_ref()),
            Expression::MemberAccess(expr) => self.print_member_access(expr.as_ref()),
            Expression::Class(expr) => self.print_class_definition(expr.as_ref()),
            Expression::TernaryOperator(expr) => self.print_ternary_operator(expr.as_ref()),
            Expression::Function(expr) => self.print_function_declaration(expr.as_ref()),
        }
    }

    fn print_function_declaration(&mut self, expr: &FunctionDeclaration) {
        if expr.is_static {
            self.print_token(Token::Keyword(Keyword::Static));
            self.print_space();
        }
        self.print_token(Token::Keyword(Keyword::Function));
        self.print_space();
        if let Some(name) = &expr.name {
            self.print_expression(name);
        }
        self.print_token(Token::LeftParenthesis);
        for (i, param) in expr.parameters.iter().enumerate() {
            if i > 0 {
                self.print_token(Token::Operator(Operator::Comma));
                self.print_space();
            }
            self.print_expression(param);
        }
        self.print_token(Token::RightParenthesis);
        self.print_space();
        if let Statement::Block(block) = &expr.statement {
            self.print_block(block);
        } else {
            self.indentation_level += 1;
            self.newline_and_indent();
            self.print_statement(&expr.statement);
            self.indentation_level -= 1;
        }
    }

    fn print_ternary_operator(&mut self, expr: &TernaryOperatorExpression) {
        self.print_expression(&expr.condition);
        self.print_space();
        self.print_token(Token::QuestionMark);
        self.print_space();
        self.print_expression(&expr.if_true);
        self.print_space();
        self.print_token(Token::Colon);
        self.print_space();
        self.print_expression(&expr.if_false);
    }

    fn print_class_definition(&mut self, expr: &ClassDefinition) {
        self.print_token(Token::Keyword(Keyword::Class));
        self.print_space();
        if let Some(name) = &expr.name {
            self.print_expression(name);
        }
        self.print_space();
        if let Some(parent) = &expr.extends {
            self.print_token(Token::Keyword(Keyword::Extends));
            self.print_space();
            self.print_expression(parent);
            self.print_space();
        }
        self.print_token(Token::LeftBrace);
        self.indentation_level += 1;
        self.newline_and_indent();
        let mut newline_after_last = false;
        for (i, member) in expr.members.iter().enumerate() {
            if i > 0 {
                self.newline_and_indent();
            }
            match member {
                ClassMemberDeclaration::FieldDeclaration(field) => {
                    if newline_after_last {
                        self.newline_and_indent();
                    }
                    newline_after_last = false;

                    if field.is_static {
                        self.print_token(Token::Keyword(Keyword::Static));
                        self.print_space();
                    }

                    self.print_expression(&field.name);
                    self.print_space();
                    self.print_token(Token::Operator(Operator::Assign));
                    self.print_space();
                    self.print_expression(&field.expression);
                }
                ClassMemberDeclaration::MethodDeclaration(method) => {
                    if i > 0 {
                        self.newline_and_indent();
                    }
                    newline_after_last = true;

                    self.print_function_declaration(method);
                }
                ClassMemberDeclaration::ConstructorDeclaration(constructor) => {
                    if i > 0 {
                        self.newline_and_indent();
                    }
                    newline_after_last = true;

                    self.print_token(Token::Keyword(Keyword::Constructor));
                    self.print_token(Token::LeftParenthesis);
                    for (i, param) in constructor.parameters.iter().enumerate() {
                        if i > 0 {
                            self.print_token(Token::Operator(Operator::Comma));
                            self.print_space();
                        }
                        self.print_expression(param);
                    }
                    self.print_token(Token::RightParenthesis);
                    self.print_space();
                    if let Statement::Block(block) = &constructor.statement {
                        self.print_block(block);
                    } else {
                        self.indentation_level += 1;
                        self.newline_and_indent();
                        self.print_statement(&constructor.statement);
                        self.indentation_level -= 1;
                    }
                }
            }
        }
        self.indentation_level -= 1;
        self.newline_and_indent();
        self.print_token(Token::RightBrace);
    }

    fn print_member_access(&mut self, expr: &MemberAccessExpression) {
        self.print_expression(&expr.expression);
        self.print_token(Token::Dot);
        self.print_token(Token::Identifier(expr.member.to_string()));
    }

    fn print_scope_resolution(&mut self, expr: &ScopeResolutionExpression) {
        if let Some(scope) = &expr.scope {
            self.print_token(Token::Identifier(scope.to_string()));
        }
        self.print_token(Token::DoubleColon);
        self.print_token(Token::Identifier(expr.accessor.to_string()));
    }

    fn print_function_call(&mut self, expr: &FunctionCallExpression) {
        self.print_expression(&expr.function);
        self.print_token(Token::LeftParenthesis);
        for (i, arg) in expr.arguments.iter().enumerate() {
            if i > 0 {
                self.print_token(Token::Operator(Operator::Comma));
                self.print_space();
            }
            self.print_expression(arg);
        }
        self.print_token(Token::RightParenthesis);
    }

    fn print_boolean_literal(&mut self, expr: &BooleanLiteralExpression) {
        if expr.value {
            self.print_token(Token::Keyword(Keyword::True));
        } else {
            self.print_token(Token::Keyword(Keyword::False));
        }
    }

    fn print_null_literal(&mut self, expr: &NullLiteralExpression) {
        self.print_token(Token::Keyword(Keyword::Null));
    }

    fn print_identifier(&mut self, expr: &IdentifierExpression) {
        self.print_token(expr.token.clone());
    }

    fn print_grouping(&mut self, expr: &GroupingExpression) {
        self.print_token(Token::LeftParenthesis);
        if let Some(expr) = &expr.expression {
            self.print_expression(expr);
        }
        self.print_token(Token::RightParenthesis);
    }

    fn print_array_access(&mut self, expr: &ArrayAccessExpression) {
        self.print_expression(&expr.array);
        self.print_token(Token::LeftBracket);
        self.print_expression(&expr.index);
        self.print_token(Token::RightBracket);
    }

    fn print_array(&mut self, expr: &ArrayExpression) {
        self.print_token(Token::LeftBracket);
        self.indentation_level += 1;
        for (i, element) in expr.elements.iter().enumerate() {
            if i > 0 {
                self.print_token(Token::Operator(Operator::Comma));
            }

            self.newline_and_indent();
            self.print_expression(element);
        }
        self.indentation_level -= 1;
        if expr.elements.len() > 0 {
            self.newline_and_indent();
        }
        self.print_token(Token::RightBracket);
    }

    fn print_delete(&mut self, expr: &DeleteExpression) {
        self.print_token(Token::Keyword(Keyword::Delete));
        self.print_space();
        self.print_expression(&expr.expression);
    }

    fn print_resume(&mut self, expr: &ResumeExpression) {
        self.print_token(Token::Keyword(Keyword::Resume));
        self.print_space();
        self.print_expression(&expr.expression);
    }

    fn print_clone(&mut self, expr: &CloneExpression) {
        self.print_token(Token::Keyword(Keyword::Clone));
        self.print_space();
        self.print_expression(&expr.expression);
    }

    fn print_table(&mut self, expr: &TableExpression) {
        self.print_token(Token::LeftBrace);
        self.indentation_level += 1;

        let mut newline_after_last = false;
        for (i, elm) in expr.entries.iter().enumerate() {
            if i > 0 {
                self.print_token(Token::Operator(Operator::Comma));
            }
            self.newline_and_indent();

            match elm {
                crate::squirrel_ast::TableEntry::Field(f) => {
                    if newline_after_last {
                        self.newline_and_indent();
                    }
                    newline_after_last = false;

                    self.print_expression(&f.name);
                    self.print_space();
                    self.print_token(Token::Operator(Operator::Assign));
                    self.print_space();
                    self.print_expression(&f.expression);
                }
                crate::squirrel_ast::TableEntry::Function(f) => {
                    if i > 0 {
                        self.newline_and_indent();
                    }
                    newline_after_last = true;

                    self.print_function_declaration(&f.function);
                }
                crate::squirrel_ast::TableEntry::FieldWithExpressionKey(f) => {
                    if i > 0 {
                        self.newline_and_indent();
                    }
                    newline_after_last = true;

                    self.print_token(Token::LeftBracket);
                    self.print_expression(&f.key);
                    self.print_token(Token::RightBracket);
                    self.print_space();
                    self.print_token(Token::Operator(Operator::Assign));
                    self.print_space();
                    self.print_expression(&f.expression);
                }
            }
        }
        self.indentation_level -= 1;
        if !expr.entries.is_empty() {
            self.newline_and_indent();
        }
        self.print_token(Token::RightBrace);
    }

    fn print_integer_literal(&mut self, expr: &IntegerLiteralExpression) {
        self.print_token(Token::Integer(expr.value));
    }

    fn print_float_literal(&mut self, expr: &FloatLiteralExpression) {
        let num_str = if expr.value.fract() == 0.0 {
            format!("{}.0", expr.value)
        } else {
            format!("{}", expr.value)
        };

        self.buffer.push_str(num_str.as_str());
    }

    fn print_multi_line_string_literal(&mut self, expr: &MutliLineStringLiteralExpression) {
        self.buffer.push_str(&format!("@\"{}\"", expr.value));
    }

    fn print_string_literal(&mut self, expr: &StringLiteralExpression) {
        self.buffer.push_str(&format!("\"{}\"", expr.value));
    }

    fn print_spread(&mut self, expr: &SpreadExpression) {
        self.print_token(Token::Dot);
        self.print_token(Token::Dot);
        self.print_token(Token::Dot);
    }

    fn print_binary_operator(&mut self, expr: &BinaryOperatorExpression) {
        self.print_expression(&expr.left);
        self.print_space();
        self.print_token(Token::Operator(expr.operator.clone()));
        self.print_space();
        self.print_expression(&expr.right);
    }

    fn print_postfix_unary_operator(&mut self, expr: &PostfixUnaryOperatorExpression) {
        self.print_expression(&expr.expression);
        self.print_token(Token::Operator(expr.operator.clone()));
    }

    fn print_unary_operator(&mut self, expr: &UnaryOperatorExpression) {
        self.print_token(Token::Operator(expr.operator.clone()));
        self.print_expression(&expr.expression);
    }

    fn print_comment(&mut self, stat: &crate::squirrel_ast::CommentStatement) {
        if stat.is_multi_line {
            self.buffer.push_str(&format!("/*{}*/", stat.comment));
        } else {
            self.buffer.push_str(&format!("//{}", stat.comment));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::squirrel_parser::Parser;

    use super::*;

    #[test]
    fn test_print_block_statements() {
        let source = "
{
{
}
}";
        let mut printer = Printer::new(
            1024,
            PrinterSettings {
                indentation: IndentationType::Spaces,
                indentation_size: 4,
            },
        );

        let mut parser = Parser::new(source);

        let ast = parser.parse().unwrap();

        let result = printer.print(&ast);

        assert_eq!(result, "{\n    {\n    }\n}");
    }

    #[test]
    fn test_print_if_statement() {
        let source = "
{
if (foo)
local a = 1;
else
local b = 2;
}";
        let mut printer = Printer::new(
            1024,
            PrinterSettings {
                indentation: IndentationType::Spaces,
                indentation_size: 4,
            },
        );

        let mut parser = Parser::new(source);

        let ast = parser.parse().unwrap();

        let result = printer.print(&ast);

        println!("{}", result);

        assert_eq!(
            result,
            "{\n    if (foo) \n        local a = 1\n    else \n        local b = 2\n}"
        );
    }

    #[test]
    fn test_print_if_statement_with_block() {
        let source = "
{
if (foo) {
local a = 1;
} else {
local b = 2;
}
}";
        let mut printer = Printer::new(
            1024,
            PrinterSettings {
                indentation: IndentationType::Spaces,
                indentation_size: 4,
            },
        );

        let mut parser = Parser::new(source);

        let ast = parser.parse().unwrap();

        let result = printer.print(&ast);

        println!("{}", result);

        assert_eq!(
            result,
            "{\n    if (foo) {\n        local a = 1\n    } else {\n        local b = 2\n    }\n}"
        );
    }
    #[test]
    fn print_test_cases() {
        let paths = fs::read_dir("./test_cases").unwrap();

        for path in paths {
            let p = path.as_ref().unwrap().path();
            let file_name = p.file_name().unwrap().to_str().unwrap();
            let extension = p.extension().unwrap().to_str().unwrap();

            if extension == "nut" {
                println!("Name: {}", file_name);

                let contents = fs::read_to_string(path.unwrap().path()).unwrap();
                let mut parser = Parser::new(&contents);
                let program = parser.parse();

                let mut printer = Printer::new(
                    contents.len(),
                    PrinterSettings {
                        indentation: IndentationType::Spaces,
                        indentation_size: 2,
                    },
                );

                let printed = printer.print(&program.unwrap());
                let printed_path = "./test_cases/".to_string() + file_name + ".nut.formatted";

                fs::write(Path::new(&printed_path), printed).unwrap();
            }
        }
    }
}
