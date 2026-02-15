use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub input: &'a [u8],
    pub position: usize,
    pub location: Location,
    pub token_counter: usize,
    pub skip_comments: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Location {
    pub line: usize,
    pub linechar: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LexerError {
    InvalidToken(Option<char>),
    InvalidKeyword(String),
    UnterminatedString,
    UnexpectedEOF,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Keyword {
    Base,
    Break,
    Case,
    Catch,
    Class,
    Clone,
    Continue,
    Const,
    Default,
    Delete,
    Else,
    Enum,
    Extends,
    For,
    Foreach,
    Function,
    If,
    Do,
    Local,
    Null,
    Resume,
    Return,
    Switch,
    This,
    Throw,
    Try,
    While,
    Yield,
    Constructor,
    True,
    False,
    Static,
    Line,
    File,
    Rawcall,
}

impl TryFrom<&str> for Keyword {
    type Error = LexerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "base" => Ok(Keyword::Base),
            "break" => Ok(Keyword::Break),
            "case" => Ok(Keyword::Case),
            "catch" => Ok(Keyword::Catch),
            "class" => Ok(Keyword::Class),
            "clone" => Ok(Keyword::Clone),
            "continue" => Ok(Keyword::Continue),
            "const" => Ok(Keyword::Const),
            "default" => Ok(Keyword::Default),
            "delete" => Ok(Keyword::Delete),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "extends" => Ok(Keyword::Extends),
            "for" => Ok(Keyword::For),
            "foreach" => Ok(Keyword::Foreach),
            "function" => Ok(Keyword::Function),
            "if" => Ok(Keyword::If),
            "local" => Ok(Keyword::Local),
            "null" => Ok(Keyword::Null),
            "resume" => Ok(Keyword::Resume),
            "return" => Ok(Keyword::Return),
            "switch" => Ok(Keyword::Switch),
            "this" => Ok(Keyword::This),
            "throw" => Ok(Keyword::Throw),
            "try" => Ok(Keyword::Try),
            "while" => Ok(Keyword::While),
            "yield" => Ok(Keyword::Yield),
            "constructor" => Ok(Keyword::Constructor),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "static" => Ok(Keyword::Static),
            "__LINE__" => Ok(Keyword::Line),
            "__FILE__" => Ok(Keyword::File),
            "rawcall" => Ok(Keyword::Rawcall),
            "do" => Ok(Keyword::Do),
            _ => Err(LexerError::InvalidKeyword(value.to_string())),
        }
    }
}

impl Into<&str> for &Keyword {
    fn into(self) -> &'static str {
        match self {
            Keyword::Base => "base",
            Keyword::Break => "break",
            Keyword::Case => "case",
            Keyword::Catch => "catch",
            Keyword::Class => "class",
            Keyword::Clone => "clone",
            Keyword::Continue => "continue",
            Keyword::Const => "const",
            Keyword::Default => "default",
            Keyword::Delete => "delete",
            Keyword::Else => "else",
            Keyword::Enum => "enum",
            Keyword::Extends => "extends",
            Keyword::For => "for",
            Keyword::Foreach => "foreach",
            Keyword::Function => "function",
            Keyword::If => "if",
            Keyword::Local => "local",
            Keyword::Null => "null",
            Keyword::Resume => "resume",
            Keyword::Return => "return",
            Keyword::Switch => "switch",
            Keyword::This => "this",
            Keyword::Throw => "throw",
            Keyword::Try => "try",
            Keyword::While => "while",
            Keyword::Yield => "yield",
            Keyword::Constructor => "constructor",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Static => "static",
            Keyword::Line => "__LINE__",
            Keyword::File => "__FILE__",
            Keyword::Rawcall => "rawcall",
            Keyword::Do => "do",
        }
    }
}

// !	!=	||	==	&&	>=	<=	>
// <=>	+	+=	-	-=	/	/=	*
// *=	%	%=	++	--	<-	=	&
// ^	|	~	>>	<<	>>>
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operator {
    Not,
    NotEqual,
    Or,
    Equal,
    And,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
    ThreeWayComparison,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Divide,
    DivideEqual,
    Multiply,
    MultiplyEqual,
    Modulo,
    ModuloEqual,
    Increment,
    Decrement,
    LeftArrow,
    Assign,
    Comma,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    BitwiseNot,
    RightShift,
    LeftShift,
    UnsignedRightShift,
    Typeof,
    InstanceOf,
    In,
}

impl From<&Operator> for &str {
    fn from(val: &Operator) -> Self {
        match val {
            Operator::Not => "!",
            Operator::NotEqual => "!=",
            Operator::Or => "||",
            Operator::Equal => "==",
            Operator::And => "&&",
            Operator::GreaterEqual => ">=",
            Operator::LessEqual => "<=",
            Operator::Greater => ">",
            Operator::Less => "<",
            Operator::ThreeWayComparison => "<=>",
            Operator::Plus => "+",
            Operator::PlusEqual => "+=",
            Operator::Minus => "-",
            Operator::MinusEqual => "-=",
            Operator::Divide => "/",
            Operator::DivideEqual => "/=",
            Operator::Multiply => "*",
            Operator::MultiplyEqual => "*=",
            Operator::Modulo => "%",
            Operator::ModuloEqual => "%=",
            Operator::Increment => "++",
            Operator::Decrement => "--",
            Operator::LeftArrow => "<-",
            Operator::Assign => "=",
            Operator::BitwiseAnd => "&",
            Operator::BitwiseXor => "^",
            Operator::BitwiseOr => "|",
            Operator::BitwiseNot => "~",
            Operator::RightShift => ">>",
            Operator::LeftShift => "<<",
            Operator::UnsignedRightShift => ">>>",
            Operator::Typeof => "typeof",
            Operator::InstanceOf => "instanceof",
            Operator::In => "in",
            Operator::Comma => ",",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Token {
    Identifier(String),
    Newline,
    Comment(String),
    MultiLineComment(String),
    String(String),
    MultiLineString(String),
    Integer(i64),
    Float(f64),
    Operator(Operator),
    Keyword(Keyword),
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Dot,
    Colon,
    QuestionMark,
    Semicolon,
    DoubleColon,
    Space,
    Tab,
    Indent(usize),
    EOF,
    Dummy,
}

impl Token {
    pub fn to_source_string(&self) -> String {
        match self {
            Token::Identifier(value) => value.clone(),
            Token::Newline => "\n".to_string(),
            Token::Comment(value) => format!("// {}", value.trim()),
            Token::MultiLineComment(value) => format!("/* {} */", value.trim()),
            Token::String(value) => format!("\"{}\"", value),
            Token::MultiLineString(value) => format!("@\"{}\"", value),
            Token::Integer(value) => value.to_string(),
            Token::Float(value) => {
                let whole = value.trunc();
                let fractional = value.fract();

                if fractional == 0.0 {
                    whole.to_string() + ".0"
                } else {
                    value.to_string()
                }
            }
            Token::Operator(value) => Into::<&str>::into(value).to_string(),
            Token::Keyword(value) => Into::<&str>::into(value).to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::LeftParenthesis => "(".to_string(),
            Token::RightParenthesis => ")".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::Dot => ".".to_string(),
            Token::Colon => ":".to_string(),
            Token::QuestionMark => "?".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::DoubleColon => "::".to_string(),
            Token::EOF => "".to_string(),
            Token::Space => " ".to_string(),
            Token::Tab => "\t".to_string(),
            //Token::Indent(spaces) => format!("<INDENT {}>", spaces),
            Token::Indent(spaces) => " ".repeat(*spaces * 2),
            Token::Dummy => "".to_string(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(value) => write!(f, "{}", value),
            Token::Newline => write!(f, "newline"),
            Token::Comment(value) => write!(f, "{}", value),
            Token::MultiLineComment(value) => write!(f, "{}", value),
            Token::String(value) => write!(f, "{}", value),
            Token::MultiLineString(value) => write!(f, "{}", value),
            Token::Integer(value) => write!(f, "{}", value),
            Token::Float(value) => write!(f, "{}", value),
            Token::Operator(value) => write!(f, "{}", Into::<&str>::into(value)),
            Token::Keyword(value) => write!(f, "{}", Into::<&str>::into(value)),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftParenthesis => write!(f, "("),
            Token::RightParenthesis => write!(f, ")"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Dot => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::QuestionMark => write!(f, "?"),
            Token::Semicolon => write!(f, ";"),
            Token::DoubleColon => write!(f, "::"),
            Token::EOF => write!(f, "EOF"),
            Token::Space => write!(f, " "),
            Token::Tab => write!(f, "<TAB>"),
            Token::Indent(spaces) => write!(f, "<INDENT>"),
            Token::Dummy => write!(f, "<DUMMY>"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenWithLocation {
    pub token: Token,
    pub from: Location,
    pub to: Location,
    pub index: usize,
}

impl TokenWithLocation {
    pub fn new(token: Token, from: Location, to: Location, index: usize) -> Self {
        Self {
            token,
            from,
            to,
            index,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexerErrorWithLocation {
    pub error: LexerError,
    pub from: Location,
    pub to: Location,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, skip_comments: bool) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
            location: Location {
                line: 0,
                linechar: 0,
            },
            token_counter: 0,
            skip_comments,
        }
    }

    pub fn peek(&self) -> Result<TokenWithLocation, LexerErrorWithLocation> {
        let mut lexer = self.clone();

        lexer.next()
    }

    pub fn peek_no_whitespace(&self) -> Result<TokenWithLocation, LexerErrorWithLocation> {
        let mut lexer = self.clone();

        lexer.skip_whitespace();
        lexer.next()
    }

    pub fn matches_tokens(&self, tokens: Vec<Token>) -> Result<bool, LexerErrorWithLocation> {
        let mut lexer = self.clone();

        for t in tokens {
            if t != lexer.next()?.token {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.position >= self.input.len() {
                return;
            }

            if let Some(_) = self.whitespace() {
                continue;
            }
            break;
        }
    }

    pub fn next(&mut self) -> Result<TokenWithLocation, LexerErrorWithLocation> {
        if self.position >= self.input.len() {
            let index = self.token_counter;
            return Ok(TokenWithLocation::new(
                Token::EOF,
                self.location.clone(),
                self.location.clone(),
                index,
            ));
        }

        let start_location = self.location.clone();

        if let Some(token) = self.whitespace() {
            let index = self.token_counter;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        }

        let start_location = self.location.clone();

        let mut comments = Vec::new();
        if self.skip_comments {
            while let Some(comment) = self.comment() {
                comments.push(comment);
                self.token_counter += 1;

                if let Some(token) = self.whitespace() {
                    let index = self.token_counter;
                    return Ok(TokenWithLocation::new(
                        token,
                        start_location,
                        self.location.clone(),
                        index,
                    ));
                }
            }
        } else if let Some(comment) = self.comment() {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                comment,
                start_location,
                self.location.clone(),
                index,
            ));
        }

        if let Some(token) = self.operator() {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        }

        if let Some(token) = self.identifier_or_keyword() {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        }

        let string_result = self.string();
        if let Ok(Some(token)) = string_result {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        } else if let Err(error) = string_result {
            return Err(LexerErrorWithLocation {
                error,
                from: start_location,
                to: self.location.clone(),
            });
        }

        let number_result = self.number();
        if let Ok(Some(token)) = number_result {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        } else if let Err(error) = number_result {
            return Err(LexerErrorWithLocation {
                error,
                from: start_location,
                to: self.location.clone(),
            });
        }

        if let Some(token) = self.punctuation() {
            let index = self.token_counter;
            self.token_counter += 1;
            return Ok(TokenWithLocation::new(
                token,
                start_location,
                self.location.clone(),
                index,
            ));
        }

        Err(LexerErrorWithLocation {
            error: LexerError::InvalidToken(self.current_char()),
            from: start_location,
            to: self.location.clone(),
        })
    }

    pub fn current_location(&self) -> Location {
        self.location.clone()
    }

    fn punctuation(&mut self) -> Option<Token> {
        let current_char = self.current_char()?;
        let token = match current_char {
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '.' => Token::Dot,
            ':' => {
                self.increment_position(false);
                if self.is_current_char(':') {
                    self.increment_position(false);
                    return Some(Token::DoubleColon);
                } else {
                    return Some(Token::Colon);
                }
            }
            '?' => Token::QuestionMark,
            ';' => Token::Semicolon,
            _ => return None,
        };

        self.increment_position(false);
        Some(token)
    }

    fn number(&mut self) -> Result<Option<Token>, LexerError> {
        if self.matches("0x") {
            self.increment_position(false);
            self.increment_position(false);
            let digits = self.take_all_hexadecimal_digits();
            return Ok(Some(Token::Integer(
                i64::from_str_radix(&digits, 16).unwrap(),
            )));
        }

        let current_char = self.current_char().ok_or(LexerError::UnexpectedEOF)?;

        if current_char.is_ascii_digit() {
            if self.matches("0.") {
                self.increment_position(false);
                self.increment_position(false);
                let digits = self.take_all_digits();

                if digits.is_empty() {
                    return Err(LexerError::InvalidToken(self.current_char()));
                }

                return Ok(Some(Token::Float(format!("0.{}", digits).parse().unwrap())));
            }

            if self.is_current_char('0') {
                self.increment_position(false);
                let digits = self.take_all_base8_digits();

                if digits.is_empty() {
                    return Ok(Some(Token::Integer(0)));
                }

                return Ok(Some(Token::Integer(
                    i64::from_str_radix(&digits, 8).unwrap(),
                )));
            }

            let digits = self.take_all_digits();
            if self.is_current_char('.') {
                self.increment_position(false);

                if self.is_current_char('e') || self.is_current_char('E') {
                    self.increment_position(false);
                    let mut sign = 1;
                    if self.is_current_char('-') {
                        sign = -1;
                        self.increment_position(false);
                    }

                    if self.is_current_char('+') {
                        self.increment_position(false);
                    }

                    let exponent = self.take_all_digits();

                    if exponent.is_empty() {
                        return Err(LexerError::InvalidToken(self.current_char()));
                    }

                    let float = digits.parse::<f64>().unwrap();
                    let exponent = exponent.parse::<i32>().unwrap();
                    let float = float * 10.0f64.powi(exponent * sign);

                    return Ok(Some(Token::Float(float)));
                }
                let digits = format!("{}.{}", digits, self.take_all_digits());
                if digits.is_empty() {
                    return Err(LexerError::InvalidToken(self.current_char()));
                }
                return Ok(Some(Token::Float(digits.parse::<f64>().unwrap())));
            } else {
                return Ok(Some(Token::Integer(digits.parse::<i64>().unwrap())));
            }
        }

        if self.is_current_char('\'') {
            self.increment_position(false);
            let next_char = self.current_char().ok_or(LexerError::UnexpectedEOF)?;
            self.increment_position(false);

            if self.is_current_char('\'') {
                self.increment_position(false);
                return Ok(Some(Token::Integer(next_char as i64)));
            } else {
                return Err(LexerError::InvalidToken(self.current_char()));
            }
        }

        Ok(None)
    }

    fn take_all_base8_digits(&mut self) -> String {
        let mut string = String::new();
        while self.is_not_eof() {
            let current_char = self.current_char().unwrap();
            if current_char.is_ascii_digit() && current_char != '8' && current_char != '9' {
                string.push(current_char);
                self.increment_position(false);
            } else {
                break;
            }
        }
        string
    }

    fn take_all_hexadecimal_digits(&mut self) -> String {
        let mut string = String::new();
        while self.is_not_eof() {
            let current_char = self.current_char().unwrap();
            if current_char.is_ascii_hexdigit() {
                string.push(current_char);
                self.increment_position(false);
            } else {
                break;
            }
        }
        string
    }

    fn take_all_digits(&mut self) -> String {
        let mut string = String::new();
        while self.is_not_eof() {
            let current_char = self.current_char().unwrap();
            if current_char.is_ascii_digit() {
                string.push(current_char);
                self.increment_position(false);
            } else {
                break;
            }
        }
        string
    }

    fn is_not_newline_and_eof(&mut self) -> bool {
        let current_char = self.current_char();

        if current_char.is_none() {
            return false;
        }

        current_char.unwrap() != '\n'
    }

    fn comment(&mut self) -> Option<Token> {
        if self.is_current_char('#') {
            self.increment_position(false);
            let mut string = String::new();
            while self.is_not_newline_and_eof() {
                string.push(self.current_char().unwrap());
                self.increment_position(false);
            }
            return Some(Token::Comment(string));
        }

        if self.matches("//") {
            self.increment_position(false);
            self.increment_position(false);
            let mut string = String::new();
            while self.is_not_newline_and_eof() {
                string.push(self.current_char().unwrap());
                self.increment_position(false);
            }
            return Some(Token::Comment(string));
        }
        if self.matches("/*") {
            self.increment_position(false);
            self.increment_position(false);

            let mut string = String::new();
            while !self.matches("*/") && self.is_not_eof() {
                string.push(self.current_char().unwrap());
                self.increment_position(self.is_current_char('\n'));
            }
            self.increment_position(false);
            self.increment_position(false);
            return Some(Token::MultiLineComment(string));
        }

        None
    }

    fn string(&mut self) -> Result<Option<Token>, LexerError> {
        if self.is_current_char('"') {
            self.increment_position(false);
            let mut string = String::new();
            loop {
                if !self.is_not_eof() {
                    return Err(LexerError::UnterminatedString);
                }

                if self.matches("\\\"") {
                    self.increment_position(false);
                    self.increment_position(false);
                    string.push('\\');
                    string.push('"');
                } else if self.matches("\"") {
                    self.increment_position(false);
                    break;
                } else {
                    if self.is_current_char('\n') {
                        return Err(LexerError::UnterminatedString);
                    }

                    string.push(self.current_char().unwrap());
                    self.increment_position(false);
                }
            }
            return Ok(Some(Token::String(string)));
        }

        if self.matches("@\"") {
            self.increment_position(false);
            self.increment_position(false);
            let mut string = String::new();
            loop {
                if !self.is_not_eof() {
                    return Err(LexerError::UnterminatedString);
                }

                if self.matches("\\\"") {
                    self.increment_position(false);
                    self.increment_position(false);
                    string.push('\\');
                    string.push('"');
                } else if self.matches("\"") {
                    self.increment_position(false);
                    break;
                } else {
                    string.push(self.current_char().unwrap());
                    self.increment_position(self.is_current_char('\n'));
                }
            }
            return Ok(Some(Token::MultiLineString(string)));
        }

        Ok(None)
    }

    fn operator(&mut self) -> Option<Token> {
        let current_char = self.current_char();

        if current_char.is_none() {
            return Some(Token::EOF);
        }

        let current_char = current_char.unwrap();

        match current_char {
            ',' => {
                self.increment_position(false);
                Some(Token::Operator(Operator::Comma))
            }
            '!' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::NotEqual));
                }
                Some(Token::Operator(Operator::Not))
            }
            '=' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::Equal));
                }
                Some(Token::Operator(Operator::Assign))
            }
            '&' => {
                self.increment_position(false);
                if self.is_current_char('&') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::And));
                }
                Some(Token::Operator(Operator::BitwiseAnd))
            }
            '|' => {
                self.increment_position(false);
                if self.is_current_char('|') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::Or));
                }
                Some(Token::Operator(Operator::BitwiseOr))
            }
            '^' => {
                self.increment_position(false);

                Some(Token::Operator(Operator::BitwiseXor))
            }
            '~' => {
                self.increment_position(false);
                Some(Token::Operator(Operator::BitwiseNot))
            }
            '+' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::PlusEqual));
                }
                if self.is_current_char('+') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::Increment));
                }
                Some(Token::Operator(Operator::Plus))
            }
            '-' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::MinusEqual));
                }
                if self.is_current_char('-') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::Decrement));
                }
                Some(Token::Operator(Operator::Minus))
            }
            '/' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::DivideEqual));
                }
                Some(Token::Operator(Operator::Divide))
            }
            '*' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::MultiplyEqual));
                }
                Some(Token::Operator(Operator::Multiply))
            }
            '%' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::ModuloEqual));
                }
                Some(Token::Operator(Operator::Modulo))
            }
            '<' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);

                    if self.is_current_char('>') {
                        self.increment_position(false);
                        return Some(Token::Operator(Operator::ThreeWayComparison));
                    }
                    return Some(Token::Operator(Operator::LessEqual));
                }
                if self.is_current_char('-') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::LeftArrow));
                }
                if self.is_current_char('<') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::LeftShift));
                }
                Some(Token::Operator(Operator::Less))
            }
            '>' => {
                self.increment_position(false);
                if self.is_current_char('=') {
                    self.increment_position(false);
                    return Some(Token::Operator(Operator::GreaterEqual));
                }
                if self.is_current_char('>') {
                    self.increment_position(false);

                    if self.is_current_char('>') {
                        self.increment_position(false);
                        return Some(Token::Operator(Operator::UnsignedRightShift));
                    }
                    return Some(Token::Operator(Operator::RightShift));
                }
                Some(Token::Operator(Operator::Greater))
            }
            _ => None,
        }
    }

    fn identifier_or_keyword(&mut self) -> Option<Token> {
        let current_char = self.current_char().unwrap();

        if current_char.is_alphabetic() || current_char == '_' {
            let mut identifier_name = "".to_string();
            while self.position < self.input.len() {
                let current_char = self.current_char().unwrap();
                if !current_char.is_alphanumeric() && current_char != '_' {
                    break;
                }

                identifier_name.push(current_char);
                self.increment_position(false);
            }

            if identifier_name == "in" {
                return Some(Token::Operator(Operator::In));
            }

            if identifier_name == "instanceof" {
                return Some(Token::Operator(Operator::InstanceOf));
            }

            if identifier_name == "typeof" {
                return Some(Token::Operator(Operator::Typeof));
            }

            if let Ok(keyword) = Keyword::try_from(identifier_name.as_str()) {
                return Some(Token::Keyword(keyword));
            }

            Some(Token::Identifier(identifier_name))
        } else {
            None
        }
    }

    fn whitespace(&mut self) -> Option<Token> {
        while self.position < self.input.len() {
            let current_char = self.current_char().unwrap();
            if !current_char.is_whitespace() {
                break;
            }

            if current_char == '\n' {
                self.increment_position(true);
                return Some(Token::Newline);
            } else {
                self.increment_position(false);
            }
        }

        None
    }

    fn matches(&mut self, expected: &str) -> bool {
        let mut position = self.position;

        if position + expected.len() > self.input.len() {
            return false;
        }

        for c in expected.chars() {
            if self.input[position] as char != c {
                return false;
            }
            position += 1;
        }

        true
    }

    fn is_not_eof(&self) -> bool {
        self.position < self.input.len()
    }

    fn current_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        if self.position < self.input.len() + 1
            && self.input[self.position] as char == '\r'
            && self.input[self.position + 1] as char == '\n'
        {
            return Some('\n');
        }

        Some(self.input[self.position] as char)
    }

    fn is_current_char(&self, c: char) -> bool {
        let current_char = self.current_char();

        if current_char.is_none() {
            return false;
        }

        current_char.unwrap() == c
    }

    fn increment_position(&mut self, line_break: bool) {
        if self.position + 1 < self.input.len()
            && self.input[self.position] as char == '\r'
            && self.input[self.position + 1] as char == '\n'
        {
            self.position += 2;
        } else {
            self.position += 1;
        }

        if line_break {
            self.location.line += 1;
            self.location.linechar = 0;
        } else {
            self.location.linechar += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use difference::assert_diff;

    use super::*;

    #[test]
    fn test_identifiers_and_whitespaces() {
        let input = "Hello world";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("Hello".to_string())
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("world".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_keywords() {
        let input = "Hello catch world try typeof instanceof in const __LIN __LINE__";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("Hello".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::Keyword(Keyword::Catch));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("world".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::Keyword(Keyword::Try));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Typeof)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::InstanceOf)
        );
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::In));
        assert_eq!(lexer.next().unwrap().token, Token::Keyword(Keyword::Const));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("__LIN".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::Keyword(Keyword::Line));
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_operators() {
        let input =
            "!	!=	||	==	&&	>=	<=	> < <=>	+	+=	-	-=	/	/=	* *=	%	%=	++	--	<-	= & ^	|	~	>>	<<	>>>";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::Not));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::NotEqual)
        );
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::Or));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Equal)
        );
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::And));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::GreaterEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::LessEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Greater)
        );
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::Less));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::ThreeWayComparison)
        );
        assert_eq!(lexer.next().unwrap().token, Token::Operator(Operator::Plus));
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::PlusEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Minus)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::MinusEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Divide)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::DivideEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Multiply)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::MultiplyEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Modulo)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::ModuloEqual)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Increment)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Decrement)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::LeftArrow)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::Assign)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::BitwiseAnd)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::BitwiseXor)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::BitwiseOr)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::BitwiseNot)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::RightShift)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::LeftShift)
        );
        assert_eq!(
            lexer.next().unwrap().token,
            Token::Operator(Operator::UnsignedRightShift)
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_string() {
        let input = "\"Hello world\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::String("Hello world".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_string_error() {
        let input = "\"Hello\nworld\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().err().unwrap(),
            LexerErrorWithLocation {
                error: LexerError::UnterminatedString,
                from: Location {
                    line: 0,
                    linechar: 0
                },
                to: Location {
                    line: 0,
                    linechar: 6
                }
            }
        );
    }

    #[test]
    fn test_string_multiline() {
        let input = "@\"Hello\n\nworld\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::MultiLineString("Hello\n\nworld".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_string_multiline_empty() {
        let input = "\"\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(lexer.next().unwrap().token, Token::String("".to_string()));
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_string_empty() {
        let input = "\"\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(lexer.next().unwrap().token, Token::String("".to_string()));
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_multiline_string_with_escapes() {
        let input = "@\"Hello\\\"world\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::MultiLineString("Hello\\\"world".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_string_with_escapes() {
        let input = "\"Hello\\\"world\"";
        let mut lexer = Lexer::new(input, false);
        assert_eq!(
            lexer.next().unwrap().token,
            Token::String("Hello\\\"world".to_string())
        );
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_comments() {
        let input = "// Hello world\n# Hello world\n// Hello world\n# Hello world";
        let mut lexer = Lexer::new(input, false);

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Comment(" Hello world".to_string())
        );

        assert_eq!(lexer.next().unwrap().token, Token::Newline);

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Comment(" Hello world".to_string())
        );

        assert_eq!(lexer.next().unwrap().token, Token::Newline);

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Comment(" Hello world".to_string())
        );

        assert_eq!(lexer.next().unwrap().token, Token::Newline);

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Comment(" Hello world".to_string())
        );

        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_multiline_comments() {
        let input =
            "/* Hello world\nHello world */ some identifier /* another comment */ identifier";
        let mut lexer = Lexer::new(input, false);

        assert_eq!(
            lexer.next().unwrap().token,
            Token::MultiLineComment(" Hello world\nHello world ".to_string())
        );

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("some".to_string())
        );

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("identifier".to_string())
        );

        assert_eq!(
            lexer.next().unwrap().token,
            Token::MultiLineComment(" another comment ".to_string())
        );

        assert_eq!(
            lexer.next().unwrap().token,
            Token::Identifier("identifier".to_string())
        );

        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn test_numbers() {
        let input = "0 0.0 123 12.12 123.0 123.e123 123.e+123 123.E-12 0x123 0123 'a' 'b'";
        let mut lexer = Lexer::new(input, false);

        const EPSILON: f64 = 1e-10;

        assert_eq!(lexer.next().unwrap().token, Token::Integer(0));
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 0.0).abs() < EPSILON),
            _ => panic!("Expected float"),
        }
        assert_eq!(lexer.next().unwrap().token, Token::Integer(123));
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 12.12).abs() < EPSILON),
            _ => panic!("Expected float"),
        }
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 123.0).abs() < EPSILON),
            _ => panic!("Expected float"),
        }
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 123e123).abs() < EPSILON * 123e123_f64.abs().max(1.0)),
            _ => panic!("Expected float"),
        }
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 123e123).abs() < EPSILON * 123e123_f64.abs().max(1.0)),
            _ => panic!("Expected float"),
        }
        match lexer.next().unwrap().token {
            Token::Float(f) => assert!((f - 123e-12).abs() < EPSILON),
            _ => panic!("Expected float"),
        }
        assert_eq!(lexer.next().unwrap().token, Token::Integer(0x123));
        assert_eq!(lexer.next().unwrap().token, Token::Integer(0o123));
        assert_eq!(lexer.next().unwrap().token, Token::Integer(97));
        assert_eq!(lexer.next().unwrap().token, Token::Integer(98));
        assert_eq!(lexer.next().unwrap().token, Token::EOF);
    }

    #[test]
    fn sanity_test_sample_code() {
        let input = "
class Test
{	
	constructor(a, b, c)
	{
        local array=[ 1, 2, 3, { x = 4, b = \"5\" } ];
		this.a = a;
		this.b = b;
		this.c = c;	
    }

	a = 0;
	b = 0;
	c = 0;
}
 
function Entity::Print()
{
    ::print(\"Entity::Print() called\");
    ::print(this.a);
    ::print(this.b);
    ::print(this.c);
}
";

        let mut lexer = Lexer::new(input, false);
        let mut tokens = Vec::new();

        let mut token = lexer.next().unwrap();
        while token.token != Token::EOF {
            tokens.push(token.token);
            token = lexer.next().unwrap();
        }
    }

    #[test]
    fn test_string_concat() {
        let input = "\"Hello \\\"\" + a + \"\\\"world\"";

        let mut lexer = Lexer::new(input, false);
        let mut tokens = Vec::new();

        let mut token = lexer.next().unwrap();
        while token.token != Token::EOF {
            tokens.push(token.token);
            token = lexer.next().unwrap();
        }

        assert_eq!(
            tokens,
            vec![
                Token::String("Hello \\\"".to_string()),
                Token::Operator(Operator::Plus),
                Token::Identifier("a".to_string()),
                Token::Operator(Operator::Plus),
                Token::String("\\\"world".to_string()),
            ]
        );
    }
}
