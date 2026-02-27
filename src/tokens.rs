pub enum Token<'a> {
    Var(&'a str),
    Identifier(&'a str),
    StringLiteral(&'a str),
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

}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Token::Var(name) => format!("VAR {} null", name),
            Token::Identifier(name) => format!("IDENTIFIER {} null", name),
            Token::StringLiteral(value) => format!("STRING {} null", value),
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
        }
    }
}

pub struct TokenIterator<'a> {
    input: &'a str,
    position: usize,
    is_eof: bool,
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        TokenIterator {
            input,
            position: 0,
            is_eof: false,
        }
    }

    fn read_char(&mut self) -> char {
        let ch = self.input[self.position..].chars().next().unwrap();
        self.position += ch.len_utf8();
        ch
    }

    pub fn next_token(&mut self) -> Token<'a> {
        if self.position >= self.input.len() {
            return Token::EOF;
        }
        return match self.read_char() {
            '(' => Token::LeftParen("("),
            ')' => Token::RightParen(")"),
            '{' => Token::LeftBrace("{"),
            '}' => Token::RightBrace("}"),
            '*' => Token::Star("*"),
            '.' => Token::Dot("."),
            ',' => Token::Comma(","),
            '+' => Token::Plus("+"),
            '-' => Token::Minus("-"),
            ';' => Token::Semicolon(";"),
            '/' => Token::Slash("/"),
            _ => {
                if self.position >= self.input.len() {
                    return Token::EOF;
                }
                // For simplicity, we treat any other character as EOF in this example.
                // In a real implementation, you would handle identifiers, string literals, etc.
                return Token::EOF;
            }
        };
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof {
            return None;
        }
        let token = self.next_token();
        if let Token::EOF = token {
            self.is_eof = true;
            Some(Token::EOF)
        } else {
            Some(token)
        }
    }
}

// LEFT_PAREN ( null
// LEFT_BRACE { null
// STAR * null
// DOT . null
// COMMA , null
// PLUS + null
// MINUS - null
// SEMICOLON ; null
// SLASH / null
// RIGHT_BRACE } null
// RIGHT_PAREN ) null
// EOF  null

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_iterator() {
        let input = "\r\n";
        let mut token_iterator = TokenIterator::new(input);
        let token = token_iterator.next().unwrap();
        assert_eq!(token.to_string(), "EOF  null");
    }

    #[test]
    fn test_token_iterator_1() {
        let input = "({*.,+*})";
        let mut token_iterator = TokenIterator::new(input);
        let tokens = token_iterator.collect::<Vec<_>>();
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
}
