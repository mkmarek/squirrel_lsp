use std::collections::HashMap;

use tower_lsp::lsp_types::SemanticTokensResult;

use crate::{
    ast_visitor::{visit, AstVisitor, AstVisitorResult},
    grammar::{expressions::*, statements::*},
    squirrel_lexer::{
        Keyword, Lexer, LexerError, LexerErrorWithLocation, Location, Operator, Token,
        TokenWithLocation,
    },
    squirrel_parser::{Parser, ParserError, ParserErrorWithLocation},
};

struct FormatterIndentationVisitor {
    line_indentations: HashMap<usize, usize>,
    current_indentation: usize,
    current_line: usize,
}

impl FormatterIndentationVisitor {
    pub fn new() -> Self {
        Self {
            line_indentations: HashMap::new(),
            current_indentation: 0,
            current_line: 0,
        }
    }
}

impl FormatterIndentationVisitor {
    fn increment_indent(&mut self, location: &Location) {
        for l in self.current_line..=location.line {
            self.line_indentations
                .entry(l)
                .or_insert(self.current_indentation);
        }

        self.current_indentation += 1;

        self.current_line = location.line;
    }

    fn decrement_indent(&mut self, location: &Location, include_current_location: bool) {
        for l in self.current_line..=location.line {
            self.line_indentations
                .entry(l)
                .or_insert(self.current_indentation);
        }

        self.current_indentation -= 1;

        if include_current_location {
            let entry = self
                .line_indentations
                .entry(location.line)
                .or_insert(self.current_indentation);

            if *entry > self.current_indentation {
                *entry = self.current_indentation;
            }
        }

        self.current_line = location.line;
    }
}

impl AstVisitor for FormatterIndentationVisitor {
    fn enter_block_statement(&mut self, statement: &BlockStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_block_statement(&mut self, statement: &BlockStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_if_statement(&mut self, statement: &IfStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_if_statement(&mut self, statement: &IfStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_while_statement(&mut self, statement: &WhileStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_while_statement(&mut self, statement: &WhileStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_do_while_statement(&mut self, statement: &DoWhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn leave_do_while_statement(&mut self, statement: &DoWhileStatement) -> AstVisitorResult {
        AstVisitorResult::Continue
    }

    fn enter_switch_statement(&mut self, statement: &SwitchStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_switch_statement(&mut self, statement: &SwitchStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_for_statement(&mut self, statement: &ForStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_for_statement(&mut self, statement: &ForStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_for_each_statement(&mut self, statement: &ForEachStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_for_each_statement(&mut self, statement: &ForEachStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_try_catch_statement(&mut self, statement: &TryCatchStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn enter_catch_clause(
        &mut self,
        _variable: &Expression,
        statement: &Statement,
    ) -> AstVisitorResult {
        self.increment_indent(&statement.get_from());
        AstVisitorResult::Continue
    }

    fn leave_catch_clause(
        &mut self,
        _variable: &Expression,
        statement: &Statement,
    ) -> AstVisitorResult {
        self.decrement_indent(&statement.get_from(), true);
        AstVisitorResult::Continue
    }

    fn leave_try_catch_statement(&mut self, statement: &TryCatchStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_break_statement(&mut self, statement: &BreakStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_break_statement(&mut self, statement: &BreakStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_continue_statement(&mut self, statement: &ContinueStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_continue_statement(&mut self, statement: &ContinueStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_return_statement(&mut self, statement: &ReturnStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_return_statement(&mut self, statement: &ReturnStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_yield_statement(&mut self, statement: &YieldStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_yield_statement(&mut self, statement: &YieldStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_throw_statement(&mut self, statement: &ThrowStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_throw_statement(&mut self, statement: &ThrowStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_expression_statement(&mut self, statement: &ExpressionStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_expression_statement(&mut self, statement: &ExpressionStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_const_statement(&mut self, statement: &ConstStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_const_statement(&mut self, statement: &ConstStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_local_statement(&mut self, statement: &LocalStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_local_statement(&mut self, statement: &LocalStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, false);
        AstVisitorResult::Continue
    }

    fn enter_function_declaration(&mut self, statement: &FunctionDefinition) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_function_declaration(&mut self, statement: &FunctionDefinition) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_class_definition(&mut self, statement: &ClassDefinition) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_class_definition(&mut self, statement: &ClassDefinition) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_enum_statement(&mut self, statement: &EnumStatement) -> AstVisitorResult {
        self.increment_indent(&statement.from);
        AstVisitorResult::Continue
    }

    fn leave_enum_statement(&mut self, statement: &EnumStatement) -> AstVisitorResult {
        self.decrement_indent(&statement.to, true);
        AstVisitorResult::Continue
    }

    fn enter_unary_operator_expression(
        &mut self,
        expression: &UnaryOperatorExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_unary_operator_expression(
        &mut self,
        expression: &UnaryOperatorExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_postfix_unary_operator_expression(
        &mut self,
        expression: &PostfixUnaryOperatorExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_postfix_unary_operator_expression(
        &mut self,
        expression: &PostfixUnaryOperatorExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_binary_operator_expression(
        &mut self,
        expression: &BinaryOperatorExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_binary_operator_expression(
        &mut self,
        expression: &BinaryOperatorExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_spread_expression(&mut self, expression: &SpreadExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_spread_expression(&mut self, expression: &SpreadExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_string_literal_expression(
        &mut self,
        expression: &StringLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_string_literal_expression(
        &mut self,
        expression: &StringLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_multiline_string_literal_expression(
        &mut self,
        expression: &MutliLineStringLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_multiline_string_literal_expression(
        &mut self,
        expression: &MutliLineStringLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_float_literal_expression(
        &mut self,
        expression: &FloatLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_float_literal_expression(
        &mut self,
        expression: &FloatLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_integer_literal_expression(
        &mut self,
        expression: &IntegerLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_integer_literal_expression(
        &mut self,
        expression: &IntegerLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_table_expression(&mut self, expression: &TableExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_table_expression(&mut self, expression: &TableExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_clone_expression(&mut self, expression: &CloneExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_clone_expression(&mut self, expression: &CloneExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_resume_expression(&mut self, expression: &ResumeExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_resume_expression(&mut self, expression: &ResumeExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_delete_expression(&mut self, expression: &DeleteExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_delete_expression(&mut self, expression: &DeleteExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_array_expression(&mut self, expression: &ArrayExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_array_expression(&mut self, expression: &ArrayExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_array_access_expression(
        &mut self,
        expression: &ArrayAccessExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_array_access_expression(
        &mut self,
        expression: &ArrayAccessExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_grouping_expression(&mut self, expression: &GroupingExpression) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_grouping_expression(&mut self, expression: &GroupingExpression) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_identifier_expression(
        &mut self,
        expression: &IdentifierExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_identifier_expression(
        &mut self,
        expression: &IdentifierExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_null_literal_expression(
        &mut self,
        expression: &NullLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_null_literal_expression(
        &mut self,
        expression: &NullLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_boolean_literal_expression(
        &mut self,
        expression: &BooleanLiteralExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_boolean_literal_expression(
        &mut self,
        expression: &BooleanLiteralExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_function_call_expression(
        &mut self,
        expression: &FunctionCallExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_function_call_expression(
        &mut self,
        expression: &FunctionCallExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_scope_resolution_expression(
        &mut self,
        expression: &ScopeResolutionExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_scope_resolution_expression(
        &mut self,
        expression: &ScopeResolutionExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_member_access_expression(
        &mut self,
        expression: &MemberAccessExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_member_access_expression(
        &mut self,
        expression: &MemberAccessExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, false);
        AstVisitorResult::Continue
    }

    fn enter_ternary_operator_expression(
        &mut self,
        expression: &TernaryOperatorExpression,
    ) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_ternary_operator_expression(
        &mut self,
        expression: &TernaryOperatorExpression,
    ) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }

    fn enter_function_expression(&mut self, expression: &FunctionDefinition) -> AstVisitorResult {
        self.increment_indent(&expression.from);
        AstVisitorResult::Continue
    }

    fn leave_function_expression(&mut self, expression: &FunctionDefinition) -> AstVisitorResult {
        self.decrement_indent(&expression.to, true);
        AstVisitorResult::Continue
    }
}

#[derive(Debug, Clone)]
pub struct Formatter<'a> {
    pub lexer: Lexer<'a>,
    pub buffer: String,
    new_line_counter: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input, false),
            buffer: String::with_capacity(input.len()),
            new_line_counter: 0,
        }
    }

    pub fn format(&mut self) -> Result<String, ParserErrorWithLocation> {
        self.format_without_indentation()?;

        self.format_indent()?;

        Ok(self.buffer.clone())
    }

    fn format_indent(&mut self) -> Result<(), ParserErrorWithLocation> {
        let mut parser = Parser::new(&self.buffer);
        let ast = parser.parse()?;

        let mut visitor = FormatterIndentationVisitor::new();
        visit(&ast, &mut visitor);

        let lines = self.buffer.lines().collect::<Vec<&str>>();
        let mut new_buffer = String::with_capacity(self.buffer.len());

        let mut last_indentation = 0;
        for (index, line) in lines.iter().enumerate() {
            let line_indentation = visitor
                .line_indentations
                .get(&index)
                .unwrap_or(&last_indentation);

            for _ in 0..*line_indentation {
                new_buffer.push_str("  ");
            }

            new_buffer.push_str(line);
            new_buffer.push('\n');
            last_indentation = *line_indentation;
        }

        self.buffer = new_buffer;

        Ok(())
    }

    fn format_without_indentation(&mut self) -> Result<(), ParserErrorWithLocation> {
        self.buffer.clear();

        let mut previous_token = Token::EOF;
        while let Ok(token_with_location) = self.lexer.next() {
            let token = token_with_location.token.clone();
            let next_token = self.lexer.peek().map(|t| t.token.clone());

            if next_token.is_err() {
                break;
            }

            if token == Token::Newline {
                self.new_line_counter += 1;
            } else {
                self.new_line_counter = 0;
            }

            let next_token = next_token.unwrap();

            match &token {
                Token::Identifier(identifier) => {
                    self.format_identifier(identifier, &next_token);
                }
                Token::Newline => {
                    if previous_token != Token::LeftBrace && self.new_line_counter <= 2 {
                        self.format_newline(&next_token);
                    }
                }
                Token::Comment(comment) => {
                    self.format_comment(comment, &next_token);
                }
                Token::MultiLineComment(comment) => {
                    self.format_multi_line_comment(comment, &next_token);
                }
                Token::String(str) => {
                    self.format_string(str, &next_token);
                }
                Token::MultiLineString(str) => {
                    self.format_multiline_string(str, &next_token);
                }
                Token::Integer(int) => {
                    self.format_integer(*int, &next_token);
                }
                Token::Float(float) => {
                    self.format_float(*float, &next_token);
                }
                Token::Operator(operator) => {
                    self.format_operator(operator, &next_token);
                }
                Token::Keyword(keyword) => {
                    self.format_keyword(keyword, &next_token);
                }
                Token::LeftBrace => {
                    self.format_left_brace(&next_token);
                }
                Token::RightBrace => {
                    self.format_right_brace(&next_token);
                }
                Token::LeftParenthesis => {
                    self.format_left_parenthesis(&next_token);
                }
                Token::RightParenthesis => {
                    self.format_right_parenthesis(&next_token);
                }
                Token::LeftBracket => {
                    self.format_left_bracket(&next_token);
                }
                Token::RightBracket => {
                    self.format_right_bracket(&next_token);
                }
                Token::Dot => {
                    self.format_dot(&next_token);
                }
                Token::Colon => {
                    self.format_colon(&next_token);
                }
                Token::QuestionMark => {
                    self.format_question_mark(&next_token);
                }
                Token::Semicolon => {
                    self.format_semicolon(&next_token);
                }
                Token::DoubleColon => {
                    self.format_double_colon(&next_token);
                }
                Token::EOF => {
                    break;
                }
            }
            previous_token = token.clone();
        }

        Ok(())
    }

    fn newline(&mut self) {
        self.buffer.push('\n');
    }

    fn format_identifier(&mut self, identifier: &str, next_token: &Token) {
        self.buffer.push_str(identifier);

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(op) => {
                if op != &Operator::Comma {
                    self.buffer.push(' ');
                }
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {}
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_newline(&mut self, next_token: &Token) {
        match next_token {
            Token::RightBrace => {
                self.buffer.push('\n');
            }
            _ => {
                self.newline();
            }
        }
    }

    fn format_comment(&mut self, comment: &str, next_token: &Token) {
        match next_token {
            Token::Newline => {
                self.buffer.push_str("// ");
                self.buffer.push_str(comment.trim());
            }
            Token::RightBrace => {
                self.buffer.push_str("// ");
                self.buffer.push_str(comment.trim());
                self.buffer.push('\n');
            }
            _ => {
                self.buffer.push_str("// ");
                self.buffer.push_str(comment.trim());
                self.newline()
            }
        }
    }

    fn format_multi_line_comment(&mut self, comment: &str, next_token: &Token) {
        match next_token {
            Token::Newline => {
                self.buffer.push_str("/* ");
                self.buffer.push_str(comment.trim());
                self.buffer.push_str(" */");
            }
            Token::RightBrace => {
                self.buffer.push_str("/* ");
                self.buffer.push_str(comment.trim());
                self.buffer.push_str(" */");
                self.buffer.push('\n');
            }
            _ => {
                self.buffer.push_str("/* ");
                self.buffer.push_str(comment.trim());
                self.buffer.push_str(" */");
                self.buffer.push(' ');
            }
        }
    }

    fn format_string(&mut self, str: &str, next_token: &Token) {
        self.buffer.push('"');
        self.buffer.push_str(str);
        self.buffer.push('"');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(op) => {
                if op != &Operator::Comma {
                    self.buffer.push(' ');
                }
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_float(&mut self, float: f64, next_token: &Token) {
        self.buffer.push_str(&float.to_string());

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(op) => {
                if op != &Operator::Comma {
                    self.buffer.push(' ');
                }
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_operator(&mut self, operator: &Operator, next_token: &Token) {
        self.buffer.push_str(operator.into());

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {
                self.buffer.push(' ');
            }
            Token::RightBracket => {
                self.buffer.push(' ');
            }
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_keyword(&mut self, keyword: &Keyword, next_token: &Token) {
        self.buffer.push_str(keyword.into());

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_left_brace(&mut self, next_token: &Token) {
        self.buffer.push('{');
        self.newline();
    }

    fn format_right_brace(&mut self, next_token: &Token) {
        if !self.buffer.ends_with('\n') {
            self.newline();
        }
        self.buffer.push('}');

        match next_token {
            Token::Newline => {
                self.newline();
                self.new_line_counter += 1;
            }
            Token::RightBrace => {}
            Token::RightBracket => {}
            Token::RightParenthesis => {}
            _ => {
                self.buffer.push(' ');
            }
        }
    }

    fn format_left_parenthesis(&mut self, next_token: &Token) {
        self.buffer.push('(');

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {}
            Token::MultiLineString(_) => {}
            Token::Integer(_) => {}
            Token::Float(_) => {}
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {}
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {}
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_right_parenthesis(&mut self, next_token: &Token) {
        self.buffer.push(')');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {}
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_left_bracket(&mut self, next_token: &Token) {
        self.buffer.push('[');

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {}
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {}
            Token::Float(_) => {}
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {}
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {}
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_right_bracket(&mut self, next_token: &Token) {
        self.buffer.push(']');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {}
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_dot(&mut self, next_token: &Token) {
        self.buffer.push('.');

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {}
            Token::MultiLineComment(_) => {}
            Token::String(_) => {}
            Token::MultiLineString(_) => {}
            Token::Integer(_) => {}
            Token::Float(_) => {}
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {}
            Token::LeftBrace => {}
            Token::RightBrace => {}
            Token::LeftParenthesis => {}
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {}
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_colon(&mut self, next_token: &Token) {
        self.buffer.push(':');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_question_mark(&mut self, next_token: &Token) {
        self.buffer.push('?');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_semicolon(&mut self, next_token: &Token) {
        self.buffer.push(';');

        match next_token {
            Token::Identifier(_) => {
                self.buffer.push(' ');
            }
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_double_colon(&mut self, next_token: &Token) {
        self.buffer.push_str("::");

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(_) => {
                self.buffer.push(' ');
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {
                self.buffer.push(' ');
            }
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_integer(&mut self, int: i64, next_token: &Token) {
        self.buffer.push_str(&int.to_string());

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(op) => {
                if op != &Operator::Comma {
                    self.buffer.push(' ');
                }
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }

    fn format_multiline_string(&mut self, str: &str, next_token: &Token) {
        self.buffer.push_str("@\"");
        self.buffer.push_str(str);
        self.buffer.push('"');

        match next_token {
            Token::Identifier(_) => {}
            Token::Newline => {}
            Token::Comment(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineComment(_) => {
                self.buffer.push(' ');
            }
            Token::String(_) => {
                self.buffer.push(' ');
            }
            Token::MultiLineString(_) => {
                self.buffer.push(' ');
            }
            Token::Integer(_) => {
                self.buffer.push(' ');
            }
            Token::Float(_) => {
                self.buffer.push(' ');
            }
            Token::Operator(op) => {
                if op != &Operator::Comma {
                    self.buffer.push(' ');
                }
            }
            Token::Keyword(_) => {
                self.buffer.push(' ');
            }
            Token::LeftBrace => {
                self.buffer.push(' ');
            }
            Token::RightBrace => {
                self.buffer.push(' ');
            }
            Token::LeftParenthesis => {
                self.buffer.push(' ');
            }
            Token::RightParenthesis => {}
            Token::LeftBracket => {}
            Token::RightBracket => {}
            Token::Dot => {}
            Token::Colon => {}
            Token::QuestionMark => {
                self.buffer.push(' ');
            }
            Token::Semicolon => {}
            Token::DoubleColon => {}
            Token::EOF => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

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
                let mut parser = Formatter::new(&contents);
                let formatted = parser.format();
                let printed_path = "./test_cases/".to_string() + file_name + ".nut.formatted";

                fs::write(Path::new(&printed_path), formatted.unwrap()).unwrap();
            }
        }
    }
}
