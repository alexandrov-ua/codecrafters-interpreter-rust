use std::fmt::Display;

pub enum SyntaxNode<'a> {
    BoolLiteral(bool),
    And(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    Or(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    Not(Box<SyntaxNode<'a>>),
    Equal(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    Less(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    LessEqual(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    Greater(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    GreaterEqual(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    NotEqual(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>), 
    NumberLiteral(f64),
    StringLiteral(&'a str),
    PlusBinary(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    MinusBinary(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    MultiplyBinary(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    DivideBinary(Box<SyntaxNode<'a>>, Box<SyntaxNode<'a>>),
    PlusUnary(Box<SyntaxNode<'a>>),
    MinusUnary(Box<SyntaxNode<'a>>),
    Parens(Box<SyntaxNode<'a>>),
    Program(Vec<SyntaxNode<'a>>),
    Print(Box<SyntaxNode<'a>>),
    Statement(Box<SyntaxNode<'a>>),
    NilLiteral, // Added to represent nil as a literal
}

impl Display for SyntaxNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxNode::BoolLiteral(val) => write!(f, "{}", val),
            SyntaxNode::And(left, right) => write!(f, "(AND {} {})", left, right),
            SyntaxNode::Or(left, right) => write!(f, "(OR {} {})", left, right),
            SyntaxNode::Not(expr) => write!(f, "(! {})", expr),
            SyntaxNode::NumberLiteral(val) => write!(f, "{:?}", val),
            SyntaxNode::StringLiteral(val) => write!(f, "{}", val),
            SyntaxNode::PlusBinary(left, right) => write!(f, "(+ {} {})", left, right),
            SyntaxNode::MinusBinary(left, right) => write!(f, "(- {} {})", left, right),
            SyntaxNode::MultiplyBinary(left, right) => write!(f, "(* {} {})", left, right),
            SyntaxNode::DivideBinary(left, right) => write!(f, "(/ {} {})", left, right),
            SyntaxNode::PlusUnary(expr) => write!(f, "(+ {})", expr),
            SyntaxNode::MinusUnary(expr) => write!(f, "(- {})", expr),
            SyntaxNode::Parens(expr) => write!(f, "(group {})", expr),
            SyntaxNode::NilLiteral => write!(f, "nil"), // Represent nil as a string
            SyntaxNode::Equal(left, right) => write!(f, "(== {} {})", left, right),
            SyntaxNode::Less(left, right) => write!(f, "(< {} {})", left, right),
            SyntaxNode::LessEqual(left, right) => write!(f, "(<= {} {})", left, right),
            SyntaxNode::Greater(left, right) => write!(f, "(> {} {})", left, right),
            SyntaxNode::GreaterEqual(left, right) => write!(f, "(>= {} {})", left, right),
            SyntaxNode::NotEqual(left, right) => write!(f, "(!= {} {})", left, right),
            SyntaxNode::Program(v)=> write!(f, "{}", v.get(0).unwrap()),
            SyntaxNode::Print(s) => write!(f, "(print {})", s),
            SyntaxNode::Statement(i) => write!(f, "{}", i),
        }
    }
}


