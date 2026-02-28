use crate::tokens::{TokinizationError, Token};

pub struct TokenIterator<'a> {
    input: &'a str,
    position: usize,
    is_eof: bool,
}

fn reserved_or_identifier<'a>(ident_str: &'a str) -> Token<'a> {
    match ident_str {
        "and" => Token::And(ident_str),
        "class" => Token::Class(ident_str),
        "else" => Token::Else(ident_str),
        "false" => Token::False(ident_str),
        "for" => Token::For(ident_str),
        "fun" => Token::Fun(ident_str),
        "if" => Token::If(ident_str),
        "nil" => Token::Nil(ident_str),
        "or" => Token::Or(ident_str),
        "print" => Token::Print(ident_str),
        "return" => Token::Return(ident_str),
        "super" => Token::Super(ident_str),
        "this" => Token::This(ident_str),
        "true" => Token::True(ident_str),
        "var" => Token::Var(ident_str),
        "while" => Token::While(ident_str),
        _ => Token::Identifier(ident_str),
    }
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        TokenIterator {
            input,
            position: 0,
            is_eof: false,
        }
    }

    fn find_line_number(&self, pos: usize) -> usize {
        self.input[..pos].chars().filter(|&c| c == '\n').count() + 1
    }

    fn read_char(&mut self) -> char {
        let ch = self.input[self.position..].chars().next().unwrap();
        self.position += ch.len_utf8();
        ch
    }

    fn read_until_rn(&mut self) -> &'a str {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            self.read_char();
        }
        &self.input[start..self.position]
    }

    fn read_while<F>(&mut self, condition: F) -> Option<&'a str>
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if !condition(ch) {
                return Some(&self.input[start..self.position]);
            }
            self.read_char();
        }
        None
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, TokinizationError> {
        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }
        return match self.read_char() {
            x if x.is_whitespace() => self.next_token(),
            x if x.is_digit(10) => {
                let tmp = self.position - 1;
                let _ = self.read_while(|c| c.is_digit(10) || c == '.');
                let num_str = &self.input[tmp..self.position];
                Ok(Token::Number(num_str, num_str.parse::<f64>().unwrap()))
            }
            x if x.is_alphabetic() || x == '_' => {
                let tmp = self.position - 1;
                let _ = self.read_while(|c| c.is_alphanumeric() || c == '_');
                let ident_str = &self.input[tmp..self.position];
                Ok(reserved_or_identifier(ident_str))
            }
            '(' => Ok(Token::LeftParen("(")),
            ')' => Ok(Token::RightParen(")")),
            '{' => Ok(Token::LeftBrace("{")),
            '}' => Ok(Token::RightBrace("}")),
            '*' => Ok(Token::Star("*")),
            '.' => Ok(Token::Dot(".")),
            ',' => Ok(Token::Comma(",")),
            '+' => Ok(Token::Plus("+")),
            '-' => Ok(Token::Minus("-")),
            ';' => Ok(Token::Semicolon(";")),
            '/' => {
                if self.peek_char() == Some('/') {
                    self.read_until_rn();
                    self.next_token()
                } else {
                    Ok(Token::Slash("/"))
                }
            }
            '=' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Ok(Token::EqualEqual("=="))
                } else {
                    Ok(Token::Equal("="))
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Ok(Token::BangEqual("!="))
                } else {
                    Ok(Token::Bang("!"))
                }
            }
            '<' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Ok(Token::LessEqual("<="))
                } else {
                    Ok(Token::Less("<"))
                }
            }
            '>' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Ok(Token::GreaterEqual(">="))
                } else {
                    Ok(Token::Greater(">"))
                }
            }
            '"' => {
                let tmp = self.position - 1;
                let val = self.read_while(|c| c != '\"');
                if let Some(v) = val {
                    self.read_char();
                    Ok(Token::StringLiteral(&self.input[tmp..self.position], v))
                } else {
                    let line_number = self.find_line_number(self.position);
                    Err(TokinizationError::UnterminatedStringLiteral(line_number))
                }
            }
            c => {
                let pos = self.position;
                let line_number = self.find_line_number(pos);
                Err(TokinizationError::UnrecognizedCharacter(
                    c.to_string(),
                    line_number,
                ))
            }
        };
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token<'a>, TokinizationError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }
        let token = self.next_token();
        if let Ok(Token::EOF) = token {
            self.is_eof = true;
            Some(Ok(Token::EOF))
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iterator() {
        let input = "";
        let mut token_iterator = TokenIterator::new(input);
        let token = token_iterator.next().unwrap().unwrap();
        assert_eq!(token.to_string(), "EOF  null");
    }

    #[test]
    fn test_token_iterator_1() {
        let input = "({*.,+*})";

        let token_iterator = TokenIterator::new(input);
        let tokens = token_iterator.map(|t| t.unwrap()).collect::<Vec<_>>();
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].to_string(), "LEFT_PAREN ( null");
        assert_eq!(tokens[1].to_string(), "LEFT_BRACE { null");
        assert_eq!(tokens[2].to_string(), "STAR * null");
        assert_eq!(tokens[3].to_string(), "DOT . null");
        assert_eq!(tokens[4].to_string(), "COMMA , null");
        assert_eq!(tokens[5].to_string(), "PLUS + null");
        assert_eq!(tokens[6].to_string(), "STAR * null");
        assert_eq!(tokens[7].to_string(), "RIGHT_BRACE } null");
        assert_eq!(tokens[8].to_string(), "RIGHT_PAREN ) null");
        assert_eq!(tokens.last().unwrap().to_string(), "EOF  null");
    }

    #[test]
    fn test_token_iterator_equal_and_bang() {
        let input = "=!=!==";
        let token_iterator = TokenIterator::new(input);
        let tokens = token_iterator.map(|t| t.unwrap()).collect::<Vec<_>>();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].to_string(), "EQUAL = null");
        assert_eq!(tokens[1].to_string(), "BANG_EQUAL != null");
        assert_eq!(tokens[2].to_string(), "BANG_EQUAL != null");
        assert_eq!(tokens[3].to_string(), "EQUAL = null");
        assert_eq!(tokens[4].to_string(), "EOF  null");
    }

    #[test]
    fn test_token_iterator_string_literal() {
        let input = "\"Hello, World!\"";
        let token_iterator = TokenIterator::new(input);
        let tokens = token_iterator.map(|t| t.unwrap()).collect::<Vec<_>>();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].to_string(),
            "STRING \"Hello, World!\" Hello, World!"
        );
        assert_eq!(tokens[1].to_string(), "EOF  null");
    }

    #[test]
    fn test_token_iterator_string_literal_negative() {
        let input = "\"Hello, World!";
        let token_iterator = TokenIterator::new(input);
        let tokens_err = token_iterator
            .map(|t| t.err())
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(tokens_err.len(), 1);
        assert_eq!(
            tokens_err[0].to_string(),
            "[line 1] Error: Unterminated string."
        );
    }

    #[test]
    fn test_token_iterator_equal() {
        let input = "\"world\" == \"world\"";
        let token_iterator = TokenIterator::new(input);
        let tokens = token_iterator.map(|t| t.unwrap()).collect::<Vec<_>>();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].to_string(), "STRING \"world\" world");
        assert_eq!(tokens[1].to_string(), "EQUAL_EQUAL == null");
        assert_eq!(tokens[2].to_string(), "STRING \"world\" world");
        assert_eq!(tokens[3].to_string(), "EOF  null");
    }
}