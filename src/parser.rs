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

    pub fn parse_program(&mut self) -> Result<SyntaxNode<'a>, String> {
        let mut lines = Vec::new();

        while let Some(t) = self.tokens.peek() {
            match t {
                Token::EOF => break,
                Token::Print(_) => {
                    let _ = self.tokens.next();
                    let expr = self.parse_binary(0)?;
                    let _ = self.match_token(|t| matches!(t, Token::Semicolon(_)))?;
                    lines.push(SyntaxNode::Print(Box::new(expr)));
                }
                _ => {
                    let expr = self.parse_binary(0)?;
                    let _ = self.match_token(|t| matches!(t, Token::Semicolon(_)))?;
                    lines.push(SyntaxNode::Statement(Box::new(expr)));
                }
            }
        }
        Ok(SyntaxNode::Program(lines))
    }

    fn parse_paren_unalry_or_literal(&mut self) -> Result<SyntaxNode<'a>, String> {
        match self.tokens.next() {
            Some(Token::True(_)) => Ok(SyntaxNode::BoolLiteral(true)),
            Some(Token::False(_)) => Ok(SyntaxNode::BoolLiteral(false)),
            Some(Token::Nil(_)) => Ok(SyntaxNode::NilLiteral), // Treat nil as false
            Some(Token::Number(_, val)) => Ok(SyntaxNode::NumberLiteral(val)),
            Some(Token::StringLiteral(_, v)) => Ok(SyntaxNode::StringLiteral(v)),
            Some(Token::Minus(_)) => {
                let t = self.parse_binary(Token::get_highest_precedence())?;
                Ok(SyntaxNode::MinusUnary(Box::new(t)))
            }
            Some(Token::Plus(_)) => {
                let t = self.parse_binary(Token::get_highest_precedence())?;
                Ok(SyntaxNode::PlusUnary(Box::new(t)))
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
                    Token::EqualEqual(_) => SyntaxNode::Equal(Box::new(left), Box::new(right)),
                    Token::BangEqual(_) => SyntaxNode::NotEqual(Box::new(left), Box::new(right)),
                    Token::Less(_) => SyntaxNode::Less(Box::new(left), Box::new(right)),
                    Token::LessEqual(_) => SyntaxNode::LessEqual(Box::new(left), Box::new(right)),
                    Token::Greater(_) => SyntaxNode::Greater(Box::new(left), Box::new(right)),
                    Token::GreaterEqual(_) => {
                        SyntaxNode::GreaterEqual(Box::new(left), Box::new(right))
                    }
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

    #[test]
    fn test_parse_complex_expression() {
        let tokens = vec![
            Token::LeftParen("("),
            Token::Number("17.0", 17.0),
            Token::Minus("-"),
            Token::Number("64.0", 64.0),
            Token::RightParen(")"),
            Token::GreaterEqual(">="),
            Token::Minus("-"),
            Token::LeftParen("("),
            Token::Number("73.0", 73.0),
            Token::Slash("/"),
            Token::Number("17.0", 17.0),
            Token::Plus("+"),
            Token::Number("35.0", 35.0),
            Token::RightParen(")"),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast.to_string(),
            "(>= (group (- 17.0 64.0)) (- (group (+ (/ 73.0 17.0) 35.0))))"
        );
    }

    //"hello"!="foo"
    #[test]
    fn test_parse_not_equal() {
        let tokens = vec![
            Token::StringLiteral("\"hello\"", "hello"),
            Token::BangEqual("!="),
            Token::StringLiteral("\"foo\"", "foo"),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast.to_string(), "(!= hello foo)");
    }

    // "world" == "world"
    #[test]
    fn test_parse_equal() {
        let tokens = vec![
            Token::StringLiteral("\"world\"", "world"),
            Token::EqualEqual("=="),
            Token::StringLiteral("\"world\"", "world"),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert_eq!(ast.to_string(), "(== world world)");
    }
}
