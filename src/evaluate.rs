use std::fmt::Display;

use crate::syntax::SyntaxNode;

pub trait Evaluate {
    fn evaluate(&self) -> Result<Value, String>;
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Evaluate for SyntaxNode<'_> {
    fn evaluate(&self) -> Result<Value, String> {
        match self {
            SyntaxNode::BoolLiteral(val) => Ok(Value::Bool(*val)),
            SyntaxNode::NumberLiteral(val) => Ok(Value::Number(*val)),
            SyntaxNode::StringLiteral(val) => Ok(Value::String(String::from(*val))),
            SyntaxNode::NilLiteral => Ok(Value::Nil), 
            SyntaxNode::PlusUnary(n) => {
                let val = n.evaluate()?;
                match val {
                    Value::Number(num) => Ok(Value::Number(num)),
                    _ => Err(format!("Expected number for unary plus, got {:?}", val)),
                }
            },
            SyntaxNode::MinusUnary(n) => {
                let val = n.evaluate()?;
                match val {
                    Value::Number(num) => Ok(Value::Number(-num)),
                    _ => Err(format!("Expected number for unary minus, got {:?}", val)),
                }
            },
            SyntaxNode::Parens(expr) => expr.evaluate(),
            SyntaxNode::PlusBinary(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l.to_string() + r)),
                    _ => Err(format!("Type error in addition: {:?} + {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::MinusBinary(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => Err(format!("Type error in subtraction: {:?} - {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::MultiplyBinary(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => Err(format!("Type error in multiplication: {:?} * {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::DivideBinary(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    },
                    _ => Err(format!("Type error in division: {:?} / {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::And(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l && *r)),
                    (Value::Bool(l), Value::Nil) => Ok(Value::Nil),
                    (Value::Nil, Value::Bool(r)) => Ok(Value::Nil),
                    (Value::Nil, Value::Nil) => Ok(Value::Nil),
                    _ => Err(format!("Type error in logical AND: {:?} AND {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::Or(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l || *r)),
                    (Value::Bool(l), Value::Nil) => Ok(Value::Bool(*l)),
                    (Value::Nil, Value::Bool(r)) => Ok(Value::Bool(*r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Nil),
                    _ => Err(format!("Type error in logical OR: {:?} OR {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::Not(expr) => {
                let val = expr.evaluate()?;
                match val {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    Value::Nil => Ok(Value::Bool(true)),
                    Value::Number(_) => Ok(Value::Bool(false)),
                    _ => Err(format!("Type error in logical NOT: !{:?}", val)),
                }
            },
            SyntaxNode::Equal(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                Ok(Value::Bool(left_val == right_val))
            },
            SyntaxNode::NotEqual(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                Ok(Value::Bool(left_val != right_val))
            },
            SyntaxNode::Less(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                    _ => Err(format!("Type error in less than comparison: {:?} < {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::LessEqual(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                    _ => Err(format!("Type error in less than or equal comparison: {:?} <= {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::Greater(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                    _ => Err(format!("Type error in greater than comparison: {:?} > {:?}", left_val, right_val)),
                }
            },
            SyntaxNode::GreaterEqual(left, right) => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                    _ => Err(format!("Type error in greater than or equal comparison: {:?} >= {:?}", left_val, right_val)),
                }
            },
        }
    }
}