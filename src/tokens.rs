use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    StringLiteral(&'a str, &'a str),
    EOF,
    LeftParen(&'a str),
    RightParen(&'a str),
    LeftBrace(&'a str),
    RightBrace(&'a str),
    Star(&'a str),
    Dot(&'a str),
    Comma(&'a str),
    Plus(&'a str),
    Minus(&'a str),
    Semicolon(&'a str),
    Slash(&'a str),
    EqualEqual(&'a str),
    Equal(&'a str),
    Bang(&'a str),
    BangEqual(&'a str),
    Less(&'a str),
    LessEqual(&'a str),
    Greater(&'a str),
    GreaterEqual(&'a str),
    Number(&'a str, f64),
    Class(&'a str),
    Else(&'a str),
    False(&'a str),
    For(&'a str),
    Fun(&'a str),
    If(&'a str),
    Nil(&'a str),
    Or(&'a str),
    Print(&'a str),
    Return(&'a str),
    Super(&'a str),
    This(&'a str),
    True(&'a str),
    While(&'a str),
    Var(&'a str),
    And(&'a str),
}

#[derive(Debug)]
pub enum TokinizationError {
    UnrecognizedCharacter(String, usize),
    UnterminatedStringLiteral(usize),
}

impl Display for TokinizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl std::error::Error for TokinizationError {}

impl TokinizationError {
    pub fn to_string(&self) -> String {
        match self {
            TokinizationError::UnrecognizedCharacter(ch, line) => {
                format!("[line {}] Error: Unexpected character: {}", line, ch)
            }
            TokinizationError::UnterminatedStringLiteral(line) => {
                format!("[line {}] Error: Unterminated string.", line)
            }
        }
    }
}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Token::Var(name) => format!("VAR {} null", name),
            Token::Identifier(name) => format!("IDENTIFIER {} null", name),
            Token::StringLiteral(l, v) => format!("STRING {} {}", l, v),
            Token::EOF => "EOF  null".to_string(),
            Token::LeftParen(_) => "LEFT_PAREN ( null".to_string(),
            Token::RightParen(_) => "RIGHT_PAREN ) null".to_string(),
            Token::LeftBrace(_) => "LEFT_BRACE { null".to_string(),
            Token::RightBrace(_) => "RIGHT_BRACE } null".to_string(),
            Token::Star(_) => "STAR * null".to_string(),
            Token::Dot(_) => "DOT . null".to_string(),
            Token::Comma(_) => "COMMA , null".to_string(),
            Token::Plus(_) => "PLUS + null".to_string(),
            Token::Minus(_) => "MINUS - null".to_string(),
            Token::Semicolon(_) => "SEMICOLON ; null".to_string(),
            Token::Slash(_) => "SLASH / null".to_string(),
            Token::EqualEqual(_) => "EQUAL_EQUAL == null".to_string(),
            Token::Equal(_) => "EQUAL = null".to_string(),
            Token::Bang(_) => "BANG ! null".to_string(),
            Token::BangEqual(_) => "BANG_EQUAL != null".to_string(),
            Token::Less(_) => "LESS < null".to_string(),
            Token::LessEqual(_) => "LESS_EQUAL <= null".to_string(),
            Token::Greater(_) => "GREATER > null".to_string(),
            Token::GreaterEqual(_) => "GREATER_EQUAL >= null".to_string(),
            Token::Number(l, v) => format!("NUMBER {} {:?}", l, v),
            Token::And(name) => format!("AND {} null", name),
            Token::Class(name) => format!("CLASS {} null", name),
            Token::Else(name) => format!("ELSE {} null", name),
            Token::False(name) => format!("FALSE {} null", name),
            Token::For(name) => format!("FOR {} null", name),
            Token::Fun(name) => format!("FUN {} null", name),
            Token::If(name) => format!("IF {} null", name),
            Token::Nil(name) => format!("NIL {} null", name),
            Token::Or(name) => format!("OR {} null", name),
            Token::Print(name) => format!("PRINT {} null", name),
            Token::Return(name) => format!("RETURN {} null", name),
            Token::Super(name) => format!("SUPER {} null", name),
            Token::This(name) => format!("THIS {} null", name),
            Token::True(name) => format!("TRUE {} null", name),
            Token::While(name) => format!("WHILE {} null", name),
        }
    }

    pub fn get_precedence(&self) -> Option<u8> {
        match self {
            Token::And(_) | Token::Or(_) | Token::EqualEqual(_) | Token::BangEqual(_) => Some(1),
            Token::Less(_)
            | Token::LessEqual(_)
            | Token::Greater(_)
            | Token::GreaterEqual(_) => Some(2),
            Token::Plus(_) | Token::Minus(_) => Some(3),
            Token::Star(_) | Token::Slash(_) => Some(4),
            _ => None,
        }
    }

    pub fn get_highest_precedence() -> u8 {
        Token::Star("*").get_precedence().unwrap() + 1
    }
}


