use crate::{
    squirrel_lexer::{Lexer, Operator, Token, TokenWithLocation},
    squirrel_parser::{Parser, ParserErrorWithLocation},
    squirrel_printer::{PrintInstruction, Printer},
};

#[derive(Debug, Clone)]
pub struct Formatter {
    input: String,
    buffer: String,
}

impl Formatter {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            buffer: String::with_capacity(input.len()),
        }
    }

    pub fn format(&mut self) -> Result<String, ParserErrorWithLocation> {
        let original_comments = {
            let mut lexer_with_comments = Lexer::new(&self.input, false);
            let mut tokens = Vec::new();

            loop {
                let token = lexer_with_comments.next();
                match token {
                    Ok(TokenWithLocation {
                        token: Token::EOF, ..
                    }) => break,
                    Ok(token) => tokens.push(token.token),
                    Err(_) => break,
                }
            }

            tokens
        };

        let formatted_tokens = {
            let mut parser = Parser::new(&self.input);

            let ast = parser.parse()?;
            let mut printer = Printer::new(1000);

            printer.print(&ast)
        };

        let mut result_tokens = Vec::new();
        let mut formatted_token_iterator = formatted_tokens.into_iter().peekable();
        let mut original_token_iterator = original_comments.into_iter().peekable();

        let mut current_indentation = 0;
        let mut newline_counter = 0;

        loop {
            let formatted = formatted_token_iterator.peek();
            let original = original_token_iterator.peek();

            let formatted = {
                if let Some(PrintInstruction::SetIndentation(indentation)) = formatted {
                    current_indentation = *indentation;
                    formatted_token_iterator.next();
                    continue;
                }

                formatted.map(|token| match token {
                    PrintInstruction::EmitToken(token) => token,
                    PrintInstruction::SetIndentation(_) => {
                        unreachable!("SetIndentation should be handled above")
                    }
                })
            };

            println!("Formatted {:?} Original {:?}", formatted, original);

            match (formatted, original) {
                (Some(formatted), Some(original)) => {
                    if *formatted == *original {
                        if *formatted == Token::Newline {
                            if newline_counter < 2 {
                                result_tokens.push(Token::Newline);
                                result_tokens.push(Token::Indent(current_indentation));
                            }
                            newline_counter += 1;
                        } else {
                            result_tokens.push(formatted.clone());
                            newline_counter = 0;
                        }

                        println!("Formatted next");
                        formatted_token_iterator.next();

                        println!("Original next");
                        original_token_iterator.next();
                    } else {
                        match (formatted, original) {
                            (Token::Newline, Token::Comment(comment)) => {
                                result_tokens.push(Token::Space);
                                result_tokens.push(Token::Comment(comment.clone()));

                                newline_counter = 0;

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (Token::Newline, Token::MultiLineComment(comment)) => {
                                result_tokens.push(Token::Space);
                                result_tokens.push(Token::MultiLineComment(comment.clone()));
                                result_tokens.push(Token::Space);
                                newline_counter = 0;

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (_, Token::Comment(comment)) => {
                                result_tokens.push(Token::Space);
                                result_tokens.push(Token::Comment(comment.clone()));
                                newline_counter = 0;

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (_, Token::MultiLineComment(comment)) => {
                                result_tokens.push(Token::Space);
                                result_tokens.push(Token::MultiLineComment(comment.clone()));
                                result_tokens.push(Token::Space);
                                newline_counter = 0;

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (Token::Space, _) => {
                                result_tokens.push(Token::Space);
                                newline_counter = 0;

                                println!("Formatted next");
                                formatted_token_iterator.next();
                            }
                            // Formatter may decide to remove commas in favour of newlines
                            (Token::Newline, Token::Operator(Operator::Comma)) => {
                                if newline_counter < 2 {
                                    result_tokens.push(Token::Newline);
                                    result_tokens.push(Token::Indent(current_indentation));
                                }
                                newline_counter += 1;

                                println!("Formatted next");
                                formatted_token_iterator.next();

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            // Formatter may decide to remove newlines in favour of commas
                            (Token::Operator(Operator::Comma), Token::Newline) => {
                                result_tokens.push(Token::Operator(Operator::Comma));
                                newline_counter = 0;

                                println!("Formatted next");
                                formatted_token_iterator.next();

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            // Formatter may decide to remove semicolons
                            (_, Token::Semicolon) => {
                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (Token::Newline, Token::Newline) => {
                                if newline_counter < 2 {
                                    result_tokens.push(Token::Newline);
                                    result_tokens.push(Token::Indent(current_indentation));
                                }
                                newline_counter += 1;

                                println!("Formatted next");
                                formatted_token_iterator.next();

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (Token::Newline, _) => {
                                if newline_counter < 2 {
                                    result_tokens.push(Token::Newline);
                                    result_tokens.push(Token::Indent(current_indentation));
                                }
                                newline_counter += 1;

                                println!("Formatted next");
                                formatted_token_iterator.next();
                            }
                            (Token::Dummy, Token::Newline) => {
                                if newline_counter < 2 {
                                    result_tokens.push(Token::Newline);
                                    result_tokens.push(Token::Indent(current_indentation));
                                }
                                newline_counter += 1;

                                println!("Formatted next");
                                formatted_token_iterator.next();

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            (Token::Dummy, _) => {
                                println!("Formatted next");
                                formatted_token_iterator.next();
                            }
                            (_, Token::Newline) => {
                                if newline_counter < 2 {
                                    result_tokens.push(Token::Newline);
                                    result_tokens.push(Token::Indent(current_indentation));
                                }
                                newline_counter += 1;

                                println!("Original next");
                                original_token_iterator.next();
                            }
                            _ => panic!(
                                "Formatted token {:?} does not match original token {:?}",
                                formatted, original
                            ),
                        }
                    }
                }
                (None, Some(token)) => {
                    if *token == Token::EOF {
                        break;
                    }
                    if token == &Token::Newline {
                        if newline_counter < 2 {
                            result_tokens.push(Token::Newline);
                            result_tokens.push(Token::Indent(current_indentation));
                        }
                        newline_counter += 1;
                    } else {
                        result_tokens.push(token.clone());
                        newline_counter = 0;
                    }
                    original_token_iterator.next();
                }
                (Some(token), None) => {
                    if *token == Token::EOF {
                        break;
                    }
                    if token == &Token::Newline {
                        if newline_counter < 2 {
                            result_tokens.push(Token::Newline);
                            result_tokens.push(Token::Indent(current_indentation));
                        }
                        newline_counter += 1;
                    } else {
                        result_tokens.push(token.clone());
                        newline_counter = 0;
                    }
                    formatted_token_iterator.next();
                }
                (None, None) => break,
            }
        }

        // Remove double spaces

        let mut i = 0;
        while i < result_tokens.len() {
            if i == 0 && result_tokens[i] == Token::Space {
                result_tokens.remove(i);
            } else if let Token::Space = result_tokens[i] {
                if i + 1 < result_tokens.len() && matches!(result_tokens[i + 1], Token::Space) {
                    result_tokens.remove(i + 1);
                } else {
                    i += 1;
                }
            } else if let Token::Indent(_) = result_tokens[i] {
                if i + 1 < result_tokens.len() && matches!(result_tokens[i + 1], Token::Space) {
                    result_tokens.remove(i + 1);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        let mut result = String::with_capacity(self.input.len());

        for token in result_tokens {
            result.push_str(&token.to_source_string());
        }

        Ok(result)
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
