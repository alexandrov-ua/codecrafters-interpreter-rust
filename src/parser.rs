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
        self.parse_binary(0)
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
                let expr = self.parse_binary(0)?;
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

    fn parse_binary(&mut self, min_precedence: u8) -> Result<SyntaxNode<'a>, String> {
        let mut left = self.parse_paren_unalry_or_literal()?;
        while let Some(op) = self.tokens.peek() {
            let precedence = op.get_precedence();

            if let Some(precedence) = precedence {
                if precedence <= min_precedence {
                    break;
                }

                let op_token = self.tokens.next().unwrap();
                let right = self.parse_binary(precedence)?;

                left = match op_token {
                    Token::And(_) => SyntaxNode::And(Box::new(left), Box::new(right)),
                    Token::Or(_) => SyntaxNode::Or(Box::new(left), Box::new(right)),
                    Token::Plus(_) => SyntaxNode::PlusBinary(Box::new(left), Box::new(right)),
                    Token::Minus(_) => SyntaxNode::MinusBinary(Box::new(left), Box::new(right)),
                    Token::Star(_) => SyntaxNode::MultiplyBinary(Box::new(left), Box::new(right)),
                    Token::Slash(_) => SyntaxNode::DivideBinary(Box::new(left), Box::new(right)),
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }
        Ok(left)
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
        let tokens = vec![
            Token::LeftParen("("),
            Token::Nil("nil"),
            Token::RightParen(")"),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::Parens(_)));
        assert_eq!(ast.to_string(), "(group nil)");
    }

    #[test]
    fn test_parse_not() {
        let tokens = vec![Token::Bang("!"), Token::True("true")];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::Not(_)));
        assert_eq!(ast.to_string(), "(! true)");
    }

    #[test]
    fn test_parse_and_or() {
        let tokens = vec![
            Token::True("true"),
            Token::And("and"),
            Token::False("false"),
            Token::Or("or"),
            Token::Nil("nil"),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::Or(_, _)));
        assert_eq!(ast.to_string(), "(OR (AND true false) nil)");
    }

    #[test]
    fn test_parse_binary_precedence() {
        let tokens = vec![
            Token::Number("1.0", 1.0),
            Token::Plus("+"),
            Token::Number("2.0", 2.0),
            Token::Star("*"),
            Token::Number("3.0", 3.0),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::PlusBinary(_, _)));
        assert_eq!(ast.to_string(), "(+ 1.0 (* 2.0 3.0))");
    }

    #[test]
    fn test_parse_binary_precedence_paren() {
        let tokens = vec![
            Token::LeftParen("("),
            Token::Number("1.0", 1.0),
            Token::Plus("+"),
            Token::Number("2.0", 2.0),
            Token::RightParen(")"),
            Token::Star("*"),
            Token::Number("3.0", 3.0),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast.to_string(), "(* (group (+ 1.0 2.0)) 3.0)");
    }

    #[test]
    fn test_parse_binary_precedence_1() {
        let tokens = vec![
            Token::Number("1.0", 1.0),
            Token::Star("*"),
            Token::Number("2.0", 2.0),
            Token::Plus("+"),
            Token::Number("3.0", 3.0),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, SyntaxNode::PlusBinary(_, _)));
        assert_eq!(ast.to_string(), "(+ (* 1.0 2.0) 3.0)");
    }
}
