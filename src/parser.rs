use crate::syntax::*;
use crate::tokens::*;

pub struct Parser<'a> {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<SyntaxNode<'a>, String> {
        self.parse_paren_unalry_or_literal()
    }

    fn parse_paren_unalry_or_literal(&mut self) -> Result<SyntaxNode<'a>, String> {
        match self.tokens.next() {
            Some(Token::True(_)) => Ok(SyntaxNode::BoolLiteral(true)),
            Some(Token::False(_)) => Ok(SyntaxNode::BoolLiteral(false)),
            Some(Token::Nil(_)) => Ok(SyntaxNode::NilLiteral), // Treat nil as false
            Some(Token::Number(_, val)) => Ok(SyntaxNode::NumberLiteral(val)),
            Some(Token::StringLiteral(_, v)) => Ok(SyntaxNode::StringLiteral(v)),
            Some(Token::Minus(_)) => {
                let operand = self.match_token(|tok| matches!(tok, Token::Number(_, _)))?;
                if let Token::Number(_, val) = operand {
                    Ok(SyntaxNode::MinusUnary(Box::new(SyntaxNode::NumberLiteral(
                        val,
                    ))))
                } else {
                    Err("Expected a number after '-'".to_string())
                }
            }
            Some(Token::Plus(_)) => {
                let operand = self.match_token(|tok| matches!(tok, Token::Number(_, _)))?;
                if let Token::Number(_, val) = operand {
                    Ok(SyntaxNode::PlusUnary(Box::new(SyntaxNode::NumberLiteral(
                        val,
                    ))))
                } else {
                    Err("Expected a number after '+'".to_string())
                }
            }
            Some(Token::LeftParen(_)) => {
                let expr = self.parse_paren_unalry_or_literal()?;
                self.match_token(|tok| matches!(tok, Token::RightParen(_)))?;
                Ok(SyntaxNode::Parens(Box::new(expr)))
            }
            Some(Token::Bang(_)) => {
                let operand = self.parse_paren_unalry_or_literal()?; // NOT has high precedence
                Ok(SyntaxNode::Not(Box::new(operand)))
            }
            Some(x) => Err(format!("Unexpected token: {}", x.to_string())),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn match_token(&mut self, expected: fn(&Token<'a>) -> bool) -> Result<Token<'a>, String> {
        match self.tokens.peek() {
            Some(tok) if expected(tok) => Ok(self.tokens.next().unwrap()),
            Some(tok) => Err(format!("Unexpected token: {}", tok.to_string())),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_literal() {
        let tokens = vec![Token::Number("42.0", 42.0)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::NumberLiteral(42.0)));
        assert_eq!(ast.to_string(), "42.0");
    }

    #[test]
    fn test_parse_bool_literal() {
        let tokens = vec![Token::True("true")];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::BoolLiteral(true)));
        assert_eq!(ast.to_string(), "true");
    }

    #[test]
    fn test_parse_nil_literal() {
        let tokens = vec![Token::Nil("nil")];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::NilLiteral));
        assert_eq!(ast.to_string(), "nil");
    }

    #[test]
    fn test_parse_parens() {
        let tokens = vec![Token::LeftParen("("), Token::Nil("nil"), Token::RightParen(")")];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::Parens(_)));
        assert_eq!(ast.to_string(), "(group nil)");
    }
}
