use std::fmt::Display;

use crate::{
    squirrel_ast::{
        array_access_expression, array_expression, binary_operator_expression, block_statement,
        boolean_literal_expression, break_statement, class_constructor_declaration,
        class_field_declaration, clone_expression, const_statement, continue_statement,
        delete_expression, do_while_statement, enum_statement, expression_statement,
        float_literal_expression, for_statement, foreach_statement, function_call_expression,
        grouping_expression, identifier_expression, if_statement, integer_literal_expression,
        local_statement, member_access_expression, mutliline_string_literal_expression,
        null_literal_expression, postfix_unary_operator_expression, resume_expression,
        return_statement, scope_resolution_expression, spread_expression,
        string_literal_expression, switch_statement, table_expression, ternary_operator_expression,
        throw_statement, try_catch_statement, unary_operator_expression, while_statement,
        yield_statement, Case, ClassDefinition, ClassMemberDeclaration, Enumeration, Expression,
        FunctionDeclaration, Initialization, Statement, Statements, TableEntry, TableEntryField,
        TableEntryFieldWithExpressionKey, TableEntryFunction,
    },
    squirrel_lexer::{
        Keyword, Lexer, LexerError, LexerErrorWithLocation, Location, Operator, Token,
        TokenWithLocation,
    },
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

#[derive(Debug)]
pub enum ParserError {
    ExpectedStatement,
    ExpectedExpression,
    ExpectedIdentifier,
    InvalidKeyword,
    UnterminatedString,
    UnexpectedToken(Token),
    ExpectedTokenGot(Token, Token),
    ExpectedOneOfGot(Vec<Token>, Token),
}

#[derive(Debug)]
pub struct ParserErrorWithLocation {
    pub error: ParserError,
    pub details: String,
    pub from: Location,
    pub to: Location,
}

impl Display for ParserErrorWithLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match &self.error {
            ParserError::ExpectedStatement => "Expected statement",
            ParserError::ExpectedExpression => "Expected expression",
            ParserError::ExpectedIdentifier => "Expected identifier",
            ParserError::InvalidKeyword => "Invalid keyword",
            ParserError::UnterminatedString => "Unterminated string",
            ParserError::UnexpectedToken(token) => {
                return write!(f, "Unexpected token: {:?}", token);
            }
            ParserError::ExpectedTokenGot(expected, got) => {
                return write!(f, "Expected {:?} but got {:?}", expected, got);
            }
            ParserError::ExpectedOneOfGot(expected, got) => {
                return write!(f, "Expected one of {:?} but got {:?}", expected, got);
            }
        };

        write!(f, "{}", error)
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input, true),
        }
    }

    pub fn parse(&mut self) -> Result<Statements, ParserErrorWithLocation> {
        self.skip_newlines()?;

        let result = self.parse_statements()?;

        self.skip_newlines()?;
        self.expect_token(Token::EOF)?;

        Ok(result)
    }

    fn skip_newlines(&mut self) -> Result<(), ParserErrorWithLocation> {
        loop {
            let next_token = self.peek_token()?;
            if next_token.token == Token::Newline {
                self.next_token()?;
            } else if let Token::Comment(_) = next_token.token {
                self.next_token()?;
            } else if let Token::MultiLineComment(_) = next_token.token {
                self.next_token()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn parse_statements(&mut self) -> Result<Statements, ParserErrorWithLocation> {
        let mut statements = Vec::new();

        let from = self.lexer.current_location();

        loop {
            let require_separator = if statements.len() > 0 {
                let next_token = self.peek_token()?;
                if next_token.token == Token::Semicolon || next_token.token == Token::Newline {
                    self.next_token()?;
                    Ok(())
                } else {
                    Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedStatement,
                        details: "Expected ; or newline after a statement".to_string(),
                        from: next_token.from,
                        to: next_token.to,
                    })
                }
            } else {
                Ok(())
            };

            let statement = self.parse_statement()?;

            match statement {
                Some(statement) => {
                    require_separator?;
                    statements.push(statement);
                }
                None => break,
            }
        }

        let next_token = self.peek_token()?;
        if next_token.token == Token::Semicolon || next_token.token == Token::Newline {
            self.next_token()?;
        }

        Ok(Statements {
            statements,
            from,
            to: self.lexer.current_location(),
        })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, ParserErrorWithLocation> {
        let mut scoped = self.clone();

        scoped.skip_newlines()?;

        let from = scoped.lexer.current_location();
        let next_token = scoped.peek_token()?;
        let statement = match next_token.token {
            Token::LeftBrace => {
                let block = scoped.parse_block_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(block))
            }
            Token::Keyword(Keyword::Do) => {
                let do_while_statement = scoped.parse_do_while_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(do_while_statement))
            }
            Token::Keyword(Keyword::While) => {
                let while_statement = scoped.parse_while_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(while_statement))
            }
            Token::Keyword(Keyword::If) => {
                let if_statement = scoped.parse_if_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(if_statement))
            }
            Token::Keyword(Keyword::Local) => {
                let local_statement = scoped.parse_local_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(local_statement))
            }
            Token::Keyword(Keyword::Foreach) => {
                let foreach_statement = scoped.parse_foreach_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(foreach_statement))
            }
            Token::Keyword(Keyword::Switch) => {
                let switch_statement = scoped.parse_switch_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(switch_statement))
            }
            Token::Keyword(Keyword::For) => {
                let for_statement = scoped.parse_for_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(for_statement))
            }
            Token::Keyword(Keyword::Break) => {
                scoped.next_token()?;
                let to = scoped.lexer.current_location();

                self.lexer = scoped.lexer;

                Ok(Some(break_statement(from, to)))
            }
            Token::Keyword(Keyword::Continue) => {
                scoped.next_token()?;
                let to = scoped.lexer.current_location();

                self.lexer = scoped.lexer;

                Ok(Some(continue_statement(from, to)))
            }
            Token::Keyword(Keyword::Return) => {
                scoped.next_token()?;

                let mut expression = None;

                let next_token = scoped.peek_token()?;
                if next_token.token != Token::Semicolon && next_token.token != Token::Newline {
                    expression = scoped.parse_expression(false)?;
                }

                let to = scoped.lexer.current_location();

                self.lexer = scoped.lexer;

                Ok(Some(return_statement(expression, from, to)))
            }
            Token::Keyword(Keyword::Yield) => {
                scoped.next_token()?;

                let mut expression = None;

                let next_token = scoped.peek_token()?;
                if next_token.token != Token::Semicolon && next_token.token != Token::Newline {
                    expression = scoped.parse_expression(false)?;
                }

                let to = scoped.lexer.current_location();

                self.lexer = scoped.lexer;

                Ok(Some(yield_statement(expression, from, to)))
            }
            Token::Keyword(Keyword::Function) => {
                let function_statement = scoped.parse_function_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(function_statement))
            }
            Token::Keyword(Keyword::Class) => {
                let class_statement = scoped.parse_class_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(class_statement))
            }
            Token::Keyword(Keyword::Try) => {
                let try_statement = scoped.parse_try_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(try_statement))
            }
            Token::Keyword(Keyword::Throw) => {
                let throw_statement = scoped.parse_throw_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(throw_statement))
            }
            Token::Keyword(Keyword::Const) => {
                let const_statement = scoped.parse_const_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(const_statement))
            }
            Token::Keyword(Keyword::Enum) => {
                let enum_statement = scoped.parse_enum_statement()?;
                self.lexer = scoped.lexer;

                Ok(Some(enum_statement))
            }
            _ => {
                let expression = scoped.parse_expression(false)?;

                if expression.is_none() {
                    Ok(None)
                } else {
                    self.lexer = scoped.lexer;

                    Ok(Some(expression_statement(
                        expression.unwrap(),
                        from,
                        self.lexer.current_location(),
                    )))
                }
            }
        }?;

        Ok(statement)
    }

    fn parse_switch_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Switch))?;

        self.expect_token(Token::LeftParenthesis)?;
        let expression = self.parse_expression(false)?;

        if expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after '(' in switch statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let expression = expression.unwrap();

        self.expect_token(Token::RightParenthesis)?;

        self.skip_newlines()?;
        self.expect_token(Token::LeftBrace)?;

        let mut cases = Vec::new();
        let mut default_case = None;

        loop {
            self.skip_newlines()?;
            let from = self.lexer.current_location();
            let next_token = self.next_token()?;

            if next_token.token == Token::Keyword(Keyword::Case) {
                let case_expr = self.parse_expression(false)?;

                if case_expr.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after ':' in case statement"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                let case_expr = case_expr.unwrap();

                self.expect_token(Token::Colon)?;
                let statements = self.parse_statements()?;

                cases.push(Case {
                    expression: case_expr,
                    statements,
                    from,
                    to: self.lexer.current_location(),
                });
            } else if next_token.token == Token::Keyword(Keyword::Default) {
                self.expect_token(Token::Colon)?;
                let statements = self.parse_statements()?;
                default_case = Some(statements);
            } else if next_token.token == Token::RightBrace {
                break;
            } else {
                return Err(ParserErrorWithLocation {
                    error: ParserError::UnexpectedToken(next_token.token),
                    details: format!("Unexpected token in switch statement"),
                    from,
                    to: self.lexer.current_location(),
                });
            }
        }

        Ok(switch_statement(
            expression,
            cases,
            default_case,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_try_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Try))?;

        let try_statement = self.parse_statement()?;

        if try_statement.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after 'try'"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let try_statement = try_statement.unwrap();

        self.expect_token(Token::Keyword(Keyword::Catch))?;
        self.expect_token(Token::LeftParenthesis)?;
        let exception_name = self.expect_identifier()?;
        self.expect_token(Token::RightParenthesis)?;

        let catch_statement = self.parse_statement()?;

        if catch_statement.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after 'catch'"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let catch_statement = catch_statement.unwrap();

        Ok(try_catch_statement(
            try_statement,
            identifier_expression(exception_name.token, exception_name.from, exception_name.to),
            catch_statement,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_throw_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Throw))?;

        let expression = self.parse_expression(false)?;

        if expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after 'throw'"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let expression = expression.unwrap();

        Ok(throw_statement(
            expression,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_const_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Const))?;

        let identifier = self.expect_identifier()?;

        self.expect_token(Token::Operator(Operator::Assign))?;

        let expression = self.parse_expression(true)?;

        if expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after '=' in const statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let expression = expression.unwrap();

        Ok(const_statement(
            identifier_expression(identifier.token, identifier.from, identifier.to),
            expression,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_enum_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let _from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Enum))?;

        let enum_name = self.expect_identifier()?;
        let from = self.lexer.current_location();

        self.skip_newlines()?;
        self.expect_token(Token::LeftBrace)?;

        let mut enumerations = Vec::new();

        loop {
            self.skip_newlines()?;
            let from = self.lexer.current_location();

            let next_token = self.peek_token()?;

            if next_token.token == Token::RightBrace {
                break;
            }

            let enumeration_name = self.expect_identifier()?;

            self.skip_newlines()?;
            let next_token = self.peek_token()?;

            if next_token.token == Token::Operator(Operator::Comma)
                || next_token.token == Token::Newline
                || next_token.token == Token::RightBrace
            {
                let enumeration = Enumeration {
                    name: identifier_expression(
                        enumeration_name.token,
                        enumeration_name.from,
                        enumeration_name.to,
                    ),
                    value: None,
                    from,
                    to: self.lexer.current_location(),
                };

                enumerations.push(enumeration);

                if next_token.token == Token::RightBrace {
                    break;
                } else {
                    self.next_token()?;
                }
            } else {
                self.expect_token(Token::Operator(Operator::Assign))?;

                let enumeration_value = self.parse_literal_expressions()?;

                if enumeration_value.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after '=' in enumeration statement"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                let enumeration_value = enumeration_value.unwrap();

                let enumeration = Enumeration {
                    name: identifier_expression(
                        enumeration_name.token,
                        enumeration_name.from,
                        enumeration_name.to,
                    ),
                    value: Some(enumeration_value),
                    from,
                    to: self.lexer.current_location(),
                };

                enumerations.push(enumeration);
            }
        }

        self.expect_token(Token::RightBrace)?;

        Ok(enum_statement(
            identifier_expression(enum_name.token, enum_name.from, enum_name.to),
            enumerations,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_class_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let defintion = self.parse_class_definition(false)?;

        Ok(Statement::Class(Box::new(defintion)))
    }

    fn parse_class_definition(
        &mut self,
        skip_name: bool,
    ) -> Result<ClassDefinition, ParserErrorWithLocation> {
        self.skip_newlines()?;

        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Class))?;

        let (class_name, extends) = if skip_name {
            (None, None)
        } else {
            let class_name = self.parse_expression(true)?;

            if class_name.is_none() {
                return Err(ParserErrorWithLocation {
                    error: ParserError::ExpectedExpression,
                    details: format!("Expected expression after 'class'"),
                    from,
                    to: self.lexer.current_location(),
                });
            }

            let class_name = class_name.unwrap();

            let mut extends = None;

            self.skip_newlines()?;
            let next_token = self.peek_token()?;

            if next_token.token == Token::Keyword(Keyword::Extends) {
                self.next_token()?;
                let extends_expr = self.parse_expression(true)?;

                if extends_expr.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after 'extends'"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                extends = extends_expr;
            }

            (Some(class_name), extends)
        };

        self.skip_newlines()?;
        self.expect_token(Token::LeftBrace)?;

        let declarations = self.parse_class_member_declarations()?;

        self.skip_newlines()?;
        self.expect_token(Token::RightBrace)?;

        Ok(ClassDefinition {
            name: class_name,
            extends,
            members: declarations,
            from,
            to: self.lexer.current_location(),
        })
    }

    fn parse_class_member_declarations(
        &mut self,
    ) -> Result<Vec<ClassMemberDeclaration>, ParserErrorWithLocation> {
        let mut declarations = Vec::new();

        loop {
            let require_separator = if declarations.len() > 0 {
                let next_token = self.peek_token()?;
                if next_token.token == Token::Semicolon || next_token.token == Token::Newline {
                    self.next_token()?;
                    Ok(())
                } else {
                    Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedStatement,
                        details: "Expected ; or newline after a class member declaration"
                            .to_string(),
                        from: next_token.from,
                        to: next_token.to,
                    })
                }
            } else {
                Ok(())
            };

            self.skip_newlines()?;
            let next_token = self.peek_token()?;

            let is_static = {
                if next_token.token == Token::Keyword(Keyword::Static) {
                    self.next_token()?;
                    true
                } else {
                    false
                }
            };

            let next_token = self.peek_token()?;

            match &next_token.token {
                Token::Identifier(_ident) => {
                    let from = self.lexer.current_location();
                    self.next_token()?;

                    self.expect_token(Token::Operator(Operator::Assign))?;

                    let value = self.parse_expression(false)?;

                    if value.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: format!(
                                "Expected expression after '=' in class field declaration"
                            ),
                            from,
                            to: self.lexer.current_location(),
                        });
                    }

                    let value = value.unwrap();

                    require_separator?;

                    declarations.push(class_field_declaration(
                        identifier_expression(next_token.token, next_token.from, next_token.to),
                        value,
                        is_static,
                        from,
                        self.lexer.current_location(),
                    ));
                }
                Token::Keyword(Keyword::Function) => {
                    let mut declaration = self.parse_function_declaration(false)?;
                    declaration.is_static = is_static;

                    require_separator?;

                    declarations.push(ClassMemberDeclaration::MethodDeclaration(declaration));
                }
                Token::Keyword(Keyword::Constructor) => {
                    let from = self.lexer.current_location();
                    self.next_token()?;

                    self.expect_token(Token::LeftParenthesis)?;

                    let mut parameters = Vec::new();
                    loop {
                        self.skip_newlines()?;
                        let next_token = self.peek_token()?;

                        if next_token.token == Token::RightParenthesis {
                            break;
                        }

                        let parameter = self.parse_expression(true)?;

                        if parameter.is_none() {
                            return Err(ParserErrorWithLocation {
                                error: ParserError::ExpectedExpression,
                                details: format!("Expected expression as constructor parameter"),
                                from,
                                to: self.lexer.current_location(),
                            });
                        }

                        parameters.push(parameter.unwrap());

                        self.skip_newlines()?;
                        let next_token = self.peek_token()?;

                        if next_token.token == Token::RightParenthesis {
                            break;
                        }

                        self.expect_token(Token::Operator(Operator::Comma))?;
                    }

                    self.skip_newlines()?;
                    self.expect_token(Token::RightParenthesis)?;

                    self.skip_newlines()?;
                    let stat = self.parse_statement()?;

                    if stat.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedStatement,
                            details: format!("Expected statement after constructor declaration"),
                            from,
                            to: self.lexer.current_location(),
                        });
                    }

                    let stat = stat.unwrap();

                    require_separator?;

                    declarations.push(class_constructor_declaration(
                        parameters,
                        stat,
                        is_static,
                        from,
                        self.lexer.current_location(),
                    ));
                }
                Token::LeftBracket => {
                    let from = self.lexer.current_location();
                    self.next_token()?;

                    let expr = self.parse_expression(false)?;

                    if expr.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: format!("Expected expression after '['"),
                            from,
                            to: self.lexer.current_location(),
                        });
                    }

                    let expr = expr.unwrap();

                    self.expect_token(Token::RightBracket)?;
                    self.expect_token(Token::Operator(Operator::Assign))?;

                    let value = self.parse_expression(false)?;

                    if value.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: format!("Expected expression after '='"),
                            from,
                            to: self.lexer.current_location(),
                        });
                    }

                    let value = value.unwrap();

                    require_separator?;

                    declarations.push(class_field_declaration(
                        expr,
                        value,
                        is_static,
                        from,
                        self.lexer.current_location(),
                    ));
                }
                _ => break,
            }
        }
        {
            let possible_semicolon = self.peek_token()?;
            if possible_semicolon.token == Token::Semicolon {
                self.next_token()?;
            }
        }

        Ok(declarations)
    }

    fn parse_function_name(&mut self) -> Result<Expression, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        let name = self.expect_identifier()?;

        let current_token = self.peek_token()?;

        if current_token.token == Token::DoubleColon {
            self.next_token()?;

            let after_scope = self.expect_identifier()?;

            return Ok(scope_resolution_expression(
                Some(name.token.to_string()),
                after_scope.token.to_string(),
                from,
                self.lexer.current_location(),
            ));
        }

        Ok(identifier_expression(
            name.token,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_function_declaration(
        &mut self,
        skip_name: bool,
    ) -> Result<FunctionDeclaration, ParserErrorWithLocation> {
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Function))?;

        let name = if skip_name {
            None
        } else {
            let name = self.parse_function_name()?;
            Some(name)
        };

        self.skip_newlines()?;
        self.expect_token(Token::LeftParenthesis)?;

        let mut parameters = Vec::new();
        loop {
            self.skip_newlines()?;
            let next_token = self.peek_token()?;

            if next_token.token == Token::RightParenthesis {
                break;
            }

            let parameter = self.parse_expression(true)?;

            if parameter.is_none() {
                return Err(ParserErrorWithLocation {
                    error: ParserError::ExpectedExpression,
                    details: format!("Expected expression as function parameter"),
                    from: self.lexer.current_location(),
                    to: self.lexer.current_location(),
                });
            }

            let parameter = parameter.unwrap();

            parameters.push(parameter);

            self.skip_newlines()?;
            let next_token = self.peek_token()?;

            if next_token.token == Token::RightParenthesis {
                break;
            }

            self.expect_token(Token::Operator(Operator::Comma))?;
        }

        self.expect_token(Token::RightParenthesis)?;

        self.skip_newlines()?;

        let statement = self.parse_statement()?;

        if statement.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after function declaration"),
                from: self.lexer.current_location(),
                to: self.lexer.current_location(),
            });
        }

        let statement = statement.unwrap();

        Ok(FunctionDeclaration {
            name,
            parameters,
            statement,
            is_static: false,
            from,
            to: self.lexer.current_location(),
        })
    }

    fn parse_function_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let declaration = self.parse_function_declaration(false)?;

        Ok(Statement::FunctionDeclaration(Box::new(declaration)))
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        let from = self.lexer.current_location();

        self.expect_token(Token::Keyword(Keyword::For))?;
        self.expect_token(Token::LeftParenthesis)?;

        let init = self.parse_statement()?;

        self.expect_token(Token::Semicolon)?;

        let condition = self.parse_expression(false)?;

        self.expect_token(Token::Semicolon)?;
        let increment = self.parse_expression(false)?;

        self.expect_token(Token::RightParenthesis)?;

        let statement = self.parse_statement()?;

        if statement.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after for statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let statement = statement.unwrap();

        Ok(for_statement(
            init,
            condition,
            increment,
            statement,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_foreach_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Foreach))?;

        self.expect_token(Token::LeftParenthesis)?;

        let (index_id, value_id) = {
            let first_ident = self.expect_identifier()?;

            let next_token = self.peek_token()?;

            if next_token.token == Token::Operator(Operator::In) {
                (None, first_ident)
            } else {
                self.expect_token(Token::Operator(Operator::Comma))?;

                let second_ident = self.expect_identifier()?;

                (Some(first_ident), second_ident)
            }
        };

        self.expect_token(Token::Operator(Operator::In))?;

        let expression = self.parse_expression(true)?;

        if expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after 'in' in foreach statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let expression = expression.unwrap();

        self.expect_token(Token::RightParenthesis)?;

        self.skip_newlines()?;

        let statement = self.parse_statement()?;

        if statement.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after foreach expression"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let statement = statement.unwrap();

        Ok(foreach_statement(
            index_id
                .map(|index_id| identifier_expression(index_id.token, index_id.from, index_id.to)),
            identifier_expression(value_id.token, value_id.from, value_id.to),
            expression,
            statement,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_local_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Local))?;

        let mut initializations = Vec::new();

        loop {
            let from = self.lexer.current_location();
            let name = self.expect_identifier()?;

            let mut expression = None;

            let next_token = self.peek_token()?;
            if next_token.token == Token::Operator(Operator::Assign) {
                self.next_token()?;
                expression = self.parse_expression(true)?;

                if expression.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after '=' in local statement"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }
            }

            initializations.push(Initialization {
                name: name.token.to_string(),
                expression,
                from,
                to: self.lexer.current_location(),
            });

            if self.peek_token()?.token != Token::Operator(Operator::Comma) {
                break;
            }

            self.next_token()?;
        }

        if initializations.len() == 0 {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedIdentifier,
                details: format!("Expected identifier after 'local' keyword"),
                from,
                to: self.lexer.current_location(),
            });
        }

        Ok(local_statement(
            initializations,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_do_while_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        self.expect_token(Token::Keyword(Keyword::Do))?;

        let stat = self.parse_statement()?;

        if stat.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after 'do' in do-while statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let stat = stat.unwrap();

        self.skip_newlines()?;
        self.expect_token(Token::Keyword(Keyword::While))?;

        self.expect_token(Token::LeftParenthesis)?;
        let condition = self.parse_expression(false)?;

        if condition.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after 'while' in do-while statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let condition = condition.unwrap();

        self.expect_token(Token::RightParenthesis)?;

        Ok(do_while_statement(
            condition,
            stat,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();

        self.expect_token(Token::Keyword(Keyword::While))?;
        self.expect_token(Token::LeftParenthesis)?;
        let condition = self.parse_expression(false)?;

        if condition.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after 'while' in while statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let condition = condition.unwrap();

        self.expect_token(Token::RightParenthesis)?;

        let stat = self.parse_statement()?;

        if stat.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after 'while' in while statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let stat = stat.unwrap();

        Ok(while_statement(
            condition,
            stat,
            from,
            self.lexer.current_location(),
        ))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();

        self.expect_token(Token::Keyword(Keyword::If))?;
        self.expect_token(Token::LeftParenthesis)?;
        let condition = self.parse_expression(false)?;

        if condition.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after 'if' in if statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let condition = condition.unwrap();

        self.expect_token(Token::RightParenthesis)?;

        self.skip_newlines()?;
        let body = self.parse_statement()?;

        if body.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedStatement,
                details: format!("Expected statement after 'if' in if statement"),
                from,
                to: self.lexer.current_location(),
            });
        }

        let body = body.unwrap();

        let else_body = self.try_parse(|parser| {
            if parser.peek_token()?.token == Token::Semicolon {
                parser.next_token()?;
            }
            let from = parser.lexer.current_location();
            if parser.peek_token_skip_whitespaces()?.token == Token::Keyword(Keyword::Else) {
                parser.skip_newlines()?;
                parser.next_token()?;

                let else_body = parser.parse_statement()?;

                if else_body.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedStatement,
                        details: format!("Expected statement after 'else' in if statement"),
                        from,
                        to: parser.lexer.current_location(),
                    });
                }

                Ok(Some(else_body.unwrap()))
            } else {
                Ok(None)
            }
        })?;

        Ok(if_statement(
            condition,
            body,
            else_body,
            from,
            self.lexer.current_location(),
        ))
    }

    fn try_parse<T>(
        &mut self,
        foo: impl FnOnce(&mut Self) -> Result<Option<T>, ParserErrorWithLocation>,
    ) -> Result<Option<T>, ParserErrorWithLocation> {
        let mut parser = self.clone();
        let result = foo(&mut parser)?;

        if result.is_some() {
            *self = parser;
            return Ok(result);
        }

        Ok(None)
    }

    fn parse_expression(
        &mut self,
        no_comma: bool,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let spread = self.parse_spread()?;

        if spread.is_some() {
            return Ok(spread);
        }

        self.parse_ternary_operator(no_comma)
    }

    fn parse_spread(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();

        if self
            .lexer
            .matches_tokens(vec![Token::Dot, Token::Dot, Token::Dot])
            .map_err(Self::map_lexer_error)?
        {
            self.next_token()?;
            self.next_token()?;
            self.next_token()?;

            Ok(Some(spread_expression(from, self.lexer.current_location())))
        } else {
            Ok(None)
        }
    }

    fn parse_ternary_operator(
        &mut self,
        no_comma: bool,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let condition = self.parse_binary_expressions(if no_comma { 1 } else { 0 })?;

        if condition.is_none() {
            return Ok(None);
        }

        let condition = condition.unwrap();

        if self.peek_token()?.token != Token::QuestionMark {
            return Ok(Some(condition));
        }

        self.next_token()?;

        let true_expression = self.parse_ternary_operator(no_comma)?;

        if true_expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after '?' in ternary operator"),
                from: self.lexer.current_location(),
                to: self.lexer.current_location(),
            });
        }

        let true_expression = true_expression.unwrap();

        self.expect_token(Token::Colon)?;

        let false_expression = self.parse_ternary_operator(no_comma)?;

        if false_expression.is_none() {
            return Err(ParserErrorWithLocation {
                error: ParserError::ExpectedExpression,
                details: format!("Expected expression after ':' in ternary operator"),
                from: self.lexer.current_location(),
                to: self.lexer.current_location(),
            });
        }

        let false_expression = false_expression.unwrap();

        Ok(Some(ternary_operator_expression(
            condition,
            true_expression,
            false_expression,
            from,
            self.lexer.current_location(),
        )))
    }

    fn operator_precedence(index: usize) -> Option<Vec<Operator>> {
        // -, ~, !, typeof , ++, --	highest
        // /, *, %	…
        // +, -
        // <<, >>, >>>
        // <, <=, >, >=, instanceof
        // ==, !=, <=>
        // &
        // ^
        // &&, in
        // +=, =, -=, /=, *=, %=
        // ,
        match index {
            0 => Some(vec![Operator::Comma]),
            1 => Some(vec![
                Operator::PlusEqual,
                Operator::Assign,
                Operator::LeftArrow,
                Operator::MinusEqual,
                Operator::DivideEqual,
                Operator::MultiplyEqual,
                Operator::ModuloEqual,
            ]),
            2 => Some(vec![Operator::And, Operator::Or, Operator::In]),
            3 => Some(vec![Operator::BitwiseXor]),
            4 => Some(vec![Operator::BitwiseAnd]),
            5 => Some(vec![Operator::BitwiseOr]),
            6 => Some(vec![
                Operator::Equal,
                Operator::NotEqual,
                Operator::ThreeWayComparison,
            ]),
            7 => Some(vec![
                Operator::Less,
                Operator::LessEqual,
                Operator::Greater,
                Operator::GreaterEqual,
                Operator::InstanceOf,
            ]),
            8 => Some(vec![
                Operator::RightShift,
                Operator::UnsignedRightShift,
                Operator::LeftShift,
            ]),
            9 => Some(vec![Operator::Minus, Operator::Plus]),
            10 => Some(vec![Operator::Modulo, Operator::Multiply, Operator::Divide]),
            11 => Some(vec![
                Operator::Not,
                Operator::BitwiseNot,
                Operator::Minus,
                Operator::Typeof,
                Operator::Increment,
                Operator::Decrement,
            ]),
            _ => None,
        }
    }

    fn parse_binary_expressions(
        &mut self,
        operator_index: usize,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let left = if operator_index < 10 {
            self.parse_binary_expressions(operator_index + 1)?
        } else {
            self.parse_unary_expressions(operator_index + 1)?
        };

        if left.is_none() {
            return Ok(None);
        }

        let mut left = left.unwrap();

        loop {
            let next_token = self.peek_token()?;
            if let Token::Operator(operator) = next_token.token {
                if !Self::operator_precedence(operator_index)
                    .unwrap()
                    .contains(&operator)
                {
                    break;
                }

                for op in Self::operator_precedence(operator_index).unwrap() {
                    if operator != op {
                        continue;
                    }

                    self.next_token()?;
                    let right = if operator_index <= 10 {
                        self.parse_binary_expressions(operator_index)?
                    } else {
                        self.parse_unary_expressions(operator_index)?
                    };

                    if right.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: format!("Expected expression after operator '{:?}'", operator),
                            from: from.clone(),
                            to: self.lexer.current_location(),
                        });
                    }

                    left = binary_operator_expression(
                        left,
                        operator.clone(),
                        right.unwrap(),
                        from.clone(),
                        self.lexer.current_location(),
                    );

                    break;
                }
            } else {
                break;
            }
        }

        Ok(Some(left))
    }

    fn parse_unary_expressions(
        &mut self,
        operator_index: usize,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let next_token = self.peek_token()?;

        for op in Self::operator_precedence(operator_index).unwrap() {
            match &next_token.token {
                Token::Operator(operator) => {
                    if operator != &op {
                        continue;
                    }

                    self.next_token()?;
                    let right = self.parse_unary_expressions(operator_index)?;

                    if right.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: format!("Expected expression after operator '{:?}'", operator),
                            from,
                            to: self.lexer.current_location(),
                        });
                    }

                    return Ok(Some(unary_operator_expression(
                        operator.clone(),
                        right.unwrap(),
                        from,
                        self.lexer.current_location(),
                    )));
                }
                _ => {}
            }
        }

        self.parse_clone()
    }

    fn parse_clone(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let next_token = self.peek_token()?;

        match next_token.token {
            Token::Keyword(Keyword::Resume) => {
                self.next_token()?;
                let expression = self.parse_expression(true)?;

                if expression.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after 'clone'"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                Ok(Some(resume_expression(
                    expression.unwrap(),
                    from,
                    self.lexer.current_location(),
                )))
            }
            Token::Keyword(Keyword::Delete) => {
                self.next_token()?;
                let expression = self.parse_expression(true)?;

                if expression.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after 'clone'"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                Ok(Some(delete_expression(
                    expression.unwrap(),
                    from,
                    self.lexer.current_location(),
                )))
            }
            Token::Keyword(Keyword::Clone) => {
                self.next_token()?;
                let expression = self.parse_expression(true)?;

                if expression.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: format!("Expected expression after 'clone'"),
                        from,
                        to: self.lexer.current_location(),
                    });
                }

                Ok(Some(clone_expression(
                    expression.unwrap(),
                    from,
                    self.lexer.current_location(),
                )))
            }
            _ => self.parse_postfix_unary_expressions(),
        }
    }

    fn parse_postfix_unary_expressions(
        &mut self,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        let left = self.parse_member_access()?;

        if left.is_none() {
            return Ok(None);
        }

        let mut left = left.unwrap();

        loop {
            let from = self.lexer.current_location();
            let next_token = self.peek_token()?;
            left = match next_token.token {
                Token::Operator(Operator::Increment) => {
                    self.next_token()?;
                    postfix_unary_operator_expression(
                        Operator::Increment,
                        left,
                        from,
                        self.lexer.current_location(),
                    )
                }
                Token::Operator(Operator::Decrement) => {
                    self.next_token()?;
                    postfix_unary_operator_expression(
                        Operator::Decrement,
                        left,
                        from,
                        self.lexer.current_location(),
                    )
                }
                _ => break,
            };
        }

        Ok(Some(left))
    }

    fn parse_member_access(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let left = self.parse_literal_expressions()?;

        if left.is_none() {
            return Ok(None);
        }

        let mut left = left.unwrap();

        loop {
            if let Some(res) = self.try_parse(|parser| {
                parser.skip_newlines()?;
                let next_token = parser.peek_token()?;
                match next_token.token {
                    // Property access
                    Token::Dot => {
                        parser.next_token()?;
                        let next_token = parser.next_token()?;

                        match next_token.token {
                            Token::Identifier(identifier) => Ok(Some(member_access_expression(
                                left.clone(),
                                identifier,
                                from.clone(),
                                parser.lexer.current_location(),
                            ))),
                            Token::Keyword(Keyword::Constructor) => {
                                Ok(Some(member_access_expression(
                                    left.clone(),
                                    "constructor".to_string(),
                                    from.clone(),
                                    parser.lexer.current_location(),
                                )))
                            }
                            _ => Err(ParserErrorWithLocation {
                                error: ParserError::ExpectedIdentifier,
                                details: format!("Expected identifier after '.'"),
                                from: from.clone(),
                                to: parser.lexer.current_location(),
                            }),
                        }
                    }
                    // function invocation
                    Token::LeftParenthesis => {
                        parser.next_token()?;

                        let mut arguments = Vec::new();

                        loop {
                            parser.skip_newlines()?;
                            let next_token = parser.peek_token()?;

                            if next_token.token == Token::RightParenthesis {
                                break;
                            }

                            let expression = parser.parse_expression(true)?;

                            if expression.is_none() {
                                return Err(ParserErrorWithLocation {
                                    error: ParserError::ExpectedExpression,
                                    details: format!("Expected expression after '('"),
                                    from: from.clone(),
                                    to: parser.lexer.current_location(),
                                });
                            }

                            arguments.push(expression.unwrap());

                            parser.skip_newlines()?;
                            let next_token = parser.peek_token()?;

                            if next_token.token == Token::RightParenthesis {
                                break;
                            }

                            parser.expect_token(Token::Operator(Operator::Comma))?;
                        }

                        parser.expect_token(Token::RightParenthesis)?;

                        Ok(Some(function_call_expression(
                            left.clone(),
                            arguments,
                            from.clone(),
                            parser.lexer.current_location(),
                        )))
                    }
                    // Index access
                    Token::LeftBracket => {
                        parser.next_token()?;
                        let index = parser.parse_expression(true)?;

                        if index.is_none() {
                            return Err(ParserErrorWithLocation {
                                error: ParserError::ExpectedExpression,
                                details: format!("Expected expression after '['"),
                                from: from.clone(),
                                to: parser.lexer.current_location(),
                            });
                        }

                        parser.skip_newlines()?;

                        parser.expect_token(Token::RightBracket)?;

                        Ok(Some(array_access_expression(
                            left.clone(),
                            index.unwrap(),
                            from.clone(),
                            parser.lexer.current_location(),
                        )))
                    }

                    _ => Ok(None),
                }
            })? {
                left = res;
            } else {
                break;
            }
        }

        Ok(Some(left))
    }

    fn parse_scope_resolution(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let mut left = self.parse_identifier_expression()?;
        let next_token = self.peek_token()?;

        if next_token.token == Token::DoubleColon {
            self.next_token()?;
            if let Some(Expression::Identifier(ident)) = left {
                let next_token = self.next_token()?;
                left = match &next_token.token {
                    Token::Identifier(val) => Ok(Some(scope_resolution_expression(
                        Some(ident.token.to_string()),
                        val.clone(),
                        from,
                        next_token.to.clone(),
                    ))),
                    _ => Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedIdentifier,
                        details: format!("Expected identifier after '::'"),
                        from: next_token.from.clone(),
                        to: next_token.to.clone(),
                    }),
                }?
            } else {
                let next_token = self.next_token()?;
                left = match &next_token.token {
                    Token::Identifier(val) => Ok(Some(scope_resolution_expression(
                        None,
                        val.clone(),
                        from,
                        next_token.to.clone(),
                    ))),
                    _ => Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedIdentifier,
                        details: format!("Expected identifier after '::'"),
                        from: next_token.from.clone(),
                        to: next_token.to.clone(),
                    }),
                }?
            }
        }

        Ok(left)
    }

    fn parse_literal_expressions(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        let next_token = self.peek_token()?;

        let literal = match next_token.token {
            Token::String(value) => {
                self.next_token()?;
                Ok(Some(string_literal_expression(
                    value,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::MultiLineString(value) => {
                self.next_token()?;
                Ok(Some(mutliline_string_literal_expression(
                    value,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::Integer(value) => {
                self.next_token()?;
                Ok(Some(integer_literal_expression(
                    value,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::Float(value) => {
                self.next_token()?;
                Ok(Some(float_literal_expression(
                    value,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::Keyword(Keyword::Null) => {
                self.next_token()?;
                Ok(Some(null_literal_expression(from.clone(), next_token.to)))
            }
            Token::Keyword(Keyword::True) => {
                self.next_token()?;
                Ok(Some(boolean_literal_expression(
                    true,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::Keyword(Keyword::False) => {
                self.next_token()?;
                Ok(Some(boolean_literal_expression(
                    false,
                    from.clone(),
                    next_token.to,
                )))
            }
            Token::LeftBracket => {
                self.next_token()?;
                let mut elements = Vec::new();

                loop {
                    self.skip_newlines()?;
                    let next_token = self.peek_token()?;

                    if next_token.token == Token::RightBracket {
                        break;
                    }

                    let element = self.parse_expression(true)?;

                    if element.is_none() {
                        return Err(ParserErrorWithLocation {
                            error: ParserError::ExpectedExpression,
                            details: "Expected expression in array".to_string(),
                            from: from.clone(),
                            to: self.lexer.current_location(),
                        });
                    }

                    elements.push(element.unwrap());

                    let next_token = self.peek_token()?;

                    if next_token.token == Token::Operator(Operator::Comma) {
                        self.next_token()?;
                    } else if next_token.token == Token::RightBracket {
                        break;
                    }
                }

                self.expect_token(Token::RightBracket)?;

                Ok(Some(array_expression(
                    elements,
                    from.clone(),
                    self.lexer.current_location(),
                )))
            }
            Token::LeftParenthesis => {
                let from = self.lexer.current_location();
                self.next_token()?;

                let expression = self.parse_expression(false)?;

                self.expect_token(Token::RightParenthesis)?;

                Ok(Some(grouping_expression(
                    expression,
                    from.clone(),
                    self.lexer.current_location(),
                )))
            }
            Token::Keyword(Keyword::Class) => self.parse_class_expression(),
            Token::Keyword(Keyword::Function) => self.parse_function_expression(),
            Token::LeftBrace => self.parse_table_exression(),
            Token::Keyword(Keyword::Base) => self.parse_identifier_expression(),
            Token::Keyword(Keyword::This) => self.parse_identifier_expression(),
            Token::Identifier(_) => self.parse_scope_resolution(),
            Token::DoubleColon => self.parse_scope_resolution(),
            _ => Ok(None),
        }?;

        Ok(literal)
    }

    fn parse_class_expression(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        let decl = self.parse_class_definition(true)?;

        Ok(Some(Expression::Class(Box::new(decl))))
    }

    fn parse_function_expression(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let decl = self.parse_function_declaration(true)?;

        Ok(Some(Expression::Function(Box::new(decl))))
    }

    fn parse_identifier_expression(
        &mut self,
    ) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let next_token = self.peek_token()?;
        let from = next_token.from.clone();

        let name = match next_token.token {
            Token::Keyword(Keyword::Base) => {
                self.next_token()?;
                Some(next_token)
            }
            Token::Keyword(Keyword::Constructor) => {
                self.next_token()?;
                Some(next_token)
            }
            Token::Keyword(Keyword::This) => {
                self.next_token()?;
                Some(next_token)
            }
            Token::Identifier(_) => {
                self.next_token()?;
                Some(next_token)
            }
            _ => None,
        };

        if name.is_none() {
            return Ok(None);
        }

        Ok(Some(identifier_expression(
            name.unwrap().token,
            from,
            self.lexer.current_location(),
        )))
    }

    fn parse_table_exression(&mut self) -> Result<Option<Expression>, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        self.expect_token(Token::LeftBrace)?;
        let mut properties = Vec::new();

        loop {
            self.skip_newlines()?;
            let from = self.lexer.current_location();
            let next_token = self.peek_token()?;

            if next_token.token == Token::Keyword(Keyword::Function) {
                let function_decl = self.parse_function_declaration(false)?;

                properties.push(TableEntry::Function(TableEntryFunction {
                    function: function_decl,
                    from,
                    to: self.lexer.current_location(),
                }));

                if self.peek_token()?.token == Token::Operator(Operator::Comma) {
                    self.next_token()?;
                }

                continue;
            }

            self.next_token()?;

            if next_token.token == Token::RightBrace {
                break;
            }

            let (id, id_expr) = if let Token::Identifier(_) = &next_token.token {
                (
                    Some(identifier_expression(
                        next_token.token.clone(),
                        next_token.from.clone(),
                        next_token.to.clone(),
                    )),
                    None,
                )
            } else if next_token.token == Token::LeftBracket {
                let from = self.lexer.current_location();
                let id_exp = self.parse_expression(false)?;

                if id_exp.is_none() {
                    return Err(ParserErrorWithLocation {
                        error: ParserError::ExpectedExpression,
                        details: "Expected expression as a key of a table".to_string(),
                        from: from.clone(),
                        to: self.lexer.current_location(),
                    });
                }

                self.expect_token(Token::RightBracket)?;

                (None, Some(id_exp.unwrap()))
            } else {
                return Err(ParserErrorWithLocation {
                    error: ParserError::ExpectedOneOfGot(
                        vec![Token::Identifier(String::new()), Token::LeftBracket],
                        next_token.token,
                    ),
                    details: "Expected identifier or expression as a key of a table".to_string(),
                    from: from.clone(),
                    to: self.lexer.current_location(),
                });
            };

            self.expect_token(Token::Operator(Operator::Assign))?;

            let value = self.parse_expression(true)?;
            if value.is_none() {
                return Err(ParserErrorWithLocation {
                    error: ParserError::ExpectedExpression,
                    details: "Expected expression as a value of a table".to_string(),
                    from,
                    to: self.lexer.current_location(),
                });
            }
            let value = value.unwrap();

            if id_expr.is_some() {
                properties.push(TableEntry::FieldWithExpressionKey(
                    TableEntryFieldWithExpressionKey {
                        key: id_expr.unwrap(),
                        expression: value,
                        from,
                        to: self.lexer.current_location(),
                    },
                ));
            } else {
                properties.push(TableEntry::Field(TableEntryField {
                    name: id.unwrap(),
                    expression: value,
                    from,
                    to: self.lexer.current_location(),
                }))
            }

            let next_token = self.peek_token()?;

            match next_token.token {
                Token::RightBrace => {
                    self.next_token()?;
                    break;
                }
                Token::Operator(Operator::Comma) => {
                    self.next_token()?;
                }
                Token::Newline => {
                    self.next_token()?;
                }
                _ => {}
            }
        }

        Ok(Some(table_expression(
            properties,
            from,
            self.lexer.current_location(),
        )))
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ParserErrorWithLocation> {
        self.skip_newlines()?;
        let from = self.lexer.current_location();
        self.expect_token(Token::LeftBrace)?;
        let statements = self.parse_statements()?;

        self.skip_newlines()?;
        self.expect_token(Token::RightBrace)?;

        Ok(block_statement(
            statements,
            from,
            self.lexer.current_location(),
        ))
    }

    fn peek_token_skip_whitespaces(
        &mut self,
    ) -> Result<TokenWithLocation, ParserErrorWithLocation> {
        self.lexer
            .peek_no_whitespace()
            .map_err(Self::map_lexer_error)
    }

    fn peek_token(&mut self) -> Result<TokenWithLocation, ParserErrorWithLocation> {
        self.lexer.peek().map_err(Self::map_lexer_error)
    }

    fn expect_identifier(&mut self) -> Result<TokenWithLocation, ParserErrorWithLocation> {
        let next_token = self.next_token()?;

        match &next_token.token {
            Token::Identifier(_) => Ok(next_token),
            _ => Err(ParserErrorWithLocation {
                error: ParserError::ExpectedIdentifier,
                details: format!("Expected identifier got {:?}", next_token.token),
                from: next_token.from,
                to: next_token.to,
            }),
        }
    }

    fn expect_token(&mut self, token: Token) -> Result<TokenWithLocation, ParserErrorWithLocation> {
        let next_token = self.next_token()?;

        if next_token.token == token {
            Ok(next_token)
        } else {
            Err(ParserErrorWithLocation {
                error: ParserError::ExpectedTokenGot(token.clone(), next_token.token.clone()),
                details: format!("Expected {} got {}", token, next_token.token),
                from: next_token.from,
                to: next_token.to,
            })
        }
    }

    fn next_token(&mut self) -> Result<TokenWithLocation, ParserErrorWithLocation> {
        self.lexer.next().map_err(Self::map_lexer_error)
    }

    fn map_lexer_error(err: LexerErrorWithLocation) -> ParserErrorWithLocation {
        match err.error {
            LexerError::UnexpectedEOF => ParserErrorWithLocation {
                error: ParserError::UnexpectedToken(Token::EOF),
                details: "Unexpected end of file".to_string(),
                from: err.from,
                to: err.to,
            },
            LexerError::UnterminatedString => ParserErrorWithLocation {
                error: ParserError::UnterminatedString,
                details: "Unterminated string".to_string(),
                from: err.from,
                to: err.to,
            },
            LexerError::InvalidKeyword(_) => ParserErrorWithLocation {
                error: ParserError::InvalidKeyword,
                details: "Invalid keyword".to_string(),
                from: err.from,
                to: err.to,
            },
            LexerError::InvalidToken(c) => ParserErrorWithLocation {
                error: ParserError::UnexpectedToken(Token::Identifier(format!("{:?}", c))),
                details: format!("Invalid token {:?}", c),
                from: err.from,
                to: err.to,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_parse_empty_block_statement() {
        let input = "{}";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        assert_eq!(1, statements.statements.len());

        let first_inner_block = match &statements.statements[0] {
            Statement::Block(block) => &block.statements,
            _ => panic!("Expected first statement to be a block"),
        };

        assert_eq!(0, first_inner_block.statements.len());
    }

    #[test]
    fn test_parse_nested_block_statements() {
        let input = "{{};{{}};{}}";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        assert_eq!(1, statements.statements.len());

        let first_inner_block = match &statements.statements[0] {
            Statement::Block(block) => &block.statements,
            _ => panic!("Expected first statement to be a block"),
        };

        assert_eq!(3, first_inner_block.statements.len());

        let second_inner_block = match &first_inner_block.statements[1] {
            Statement::Block(block) => &block.statements,
            _ => panic!("Expected second statement to be a block"),
        };

        assert_eq!(1, second_inner_block.statements.len());
    }

    #[test]
    fn test_parse_simple_binary_expression() {
        let input = "1 + 2";
        let mut parser = Parser::new(input);

        let statements = parser.parse().unwrap();

        assert_eq!(statements.statements.len(), 1);

        match &statements.statements[0] {
            Statement::Expression(expr) => {
                let binary_operator = match &expr.expression {
                    Expression::BinaryOperator(binary_operator) => binary_operator,
                    _ => panic!("Expected expression to be a binary operator"),
                };

                assert_eq!(binary_operator.operator, Operator::Plus);

                match &binary_operator.left {
                    Expression::IntegerLiteral(literal) => assert_eq!(literal.value, 1),
                    _ => panic!("Expected left to be an integer literal"),
                }

                match &binary_operator.right {
                    Expression::IntegerLiteral(literal) => assert_eq!(literal.value, 2),
                    _ => panic!("Expected right to be an integer literal"),
                }
            }
            _ => panic!("Expected first statement to be an expression"),
        };
    }

    #[test]
    fn test_local_variable_declaration() {
        let input = "local a = 10;";
        let mut parser = Parser::new(input);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.statements.len(), 1);

        match &statements.statements[0] {
            Statement::Local(local) => {
                assert_eq!(local.initializations.len(), 1);

                let first_init = &local.initializations[0];

                assert_eq!(first_init.name, "a");

                match first_init.expression.as_ref() {
                    Some(Expression::IntegerLiteral(literal)) => assert_eq!(literal.value, 10),
                    _ => panic!("Expected initializer to be an integer literal"),
                }
            }
            _ => panic!("Expected first statement to be a local variable declaration"),
        };
    }

    #[test]
    fn run_test_cases() {
        let paths = fs::read_dir("./test_cases").unwrap();

        for path in paths {
            let p = path.as_ref().unwrap().path();
            let file_name = p.file_name().unwrap().to_str().unwrap();
            let extension = p.extension().unwrap().to_str().unwrap();

            if extension == "nut" {
                println!("Name: {}", file_name);

                let contents = fs::read_to_string(path.unwrap().path()).unwrap();

                let mut lexer = Lexer::new(&contents, true);
                let mut tokens = Vec::new();

                loop {
                    let token = lexer.next().unwrap();

                    if token.token == crate::squirrel_lexer::Token::EOF {
                        break;
                    }

                    tokens.push(token);
                }

                let json_path = "./test_cases/".to_string() + file_name + ".tokens.json";

                let tokens = serde_json::to_string_pretty(&tokens).unwrap();

                fs::write(Path::new(&json_path), tokens).unwrap();

                let mut parser = Parser::new(&contents);
                let program = parser.parse();

                let ast = serde_json::to_string_pretty(&program.unwrap()).unwrap();

                let json_path = "./test_cases/".to_string() + file_name + ".ast.json";

                fs::write(Path::new(&json_path), ast).unwrap();
            }
        }
    }
}
