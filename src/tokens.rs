pub enum Token<'a>{
    Var(&'a str),
    Identifier(&'a str),
    StringLiteral(&'a str),
    EOF
}

impl<'a> Token<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Token::Var(name) => format!("VAR {} null", name),
            Token::Identifier(name) => format!("IDENTIFIER {} null", name),
            Token::StringLiteral(value) => format!("STRING {} null", value),
            Token::EOF => "EOF  null".to_string(),
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
        TokenIterator { input, position: 0, is_eof: false }
    }

    pub fn next_token(&mut self) -> Token<'a> {

        if self.position >= self.input.len() {
            return Token::EOF;
        }
        Token::EOF
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
}