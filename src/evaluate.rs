use std::fmt::{Display};

use crate::syntax::SyntaxNode;
use std::collections::HashMap;

pub trait Evaluate {
    fn evaluate(&self, context: &mut ValiableContext) -> Result<Value, String>;
}

#[derive(Debug, PartialEq, Clone)]
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

pub struct ValiableContext {
    variables: Vec<HashMap<String, Value>>,
}

impl ValiableContext {
    pub fn new() -> Self {
        ValiableContext {
            variables: vec![HashMap::new()],
        }
    }

    fn set_in_current_scoupe(&mut self, name: &str, value: Value) {
        self.variables
            .last_mut()
            .unwrap()
            .insert(name.to_string(), value);
    }

    fn has_in_current_scoupe(&self, name: &str) -> bool {
        self.variables.last().unwrap().contains_key(name)
    }

    fn set_variable(&mut self, name: &str, value: Value) {
        for scope in self.variables.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return;
            }
        }
        self.set_in_current_scoupe(name, value);
    }

    fn get_variable(&self, name: &str) -> Option<&Value> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    fn has_variable(&self, name: &str) -> bool {
        for scope in self.variables.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn new_child_scoupe(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn exit_scoupe(&mut self) {
        if self.variables.len() < 2 {
            panic!("Cannot exit the global scope");
        }
        self.variables.pop();
    }
}

impl Evaluate for SyntaxNode<'_> {
    fn evaluate(&self, context: &mut ValiableContext) -> Result<Value, String> {
        match self {
            SyntaxNode::BoolLiteral(val) => Ok(Value::Bool(*val)),
            SyntaxNode::NumberLiteral(val) => Ok(Value::Number(*val)),
            SyntaxNode::StringLiteral(val) => Ok(Value::String(String::from(*val))),
            SyntaxNode::NilLiteral => Ok(Value::Nil),
            SyntaxNode::PlusUnary(n) => {
                let val = n.evaluate(context)?;
                match val {
                    Value::Number(num) => Ok(Value::Number(num)),
                    _ => Err(format!("Expected number for unary plus, got {:?}", val)),
                }
            }
            SyntaxNode::MinusUnary(n) => {
                let val = n.evaluate(context)?;
                match val {
                    Value::Number(num) => Ok(Value::Number(-num)),
                    _ => Err(format!("Expected number for unary minus, got {:?}", val)),
                }
            }
            SyntaxNode::Parens(expr) => expr.evaluate(context),
            SyntaxNode::PlusBinary(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l.to_string() + r)),
                    _ => Err(format!(
                        "Type error in addition: {:?} + {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::MinusBinary(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => Err(format!(
                        "Type error in subtraction: {:?} - {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::MultiplyBinary(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => Err(format!(
                        "Type error in multiplication: {:?} * {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::DivideBinary(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    _ => Err(format!(
                        "Type error in division: {:?} / {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::And(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l && *r)),
                    (Value::Bool(_), Value::Nil) => Ok(Value::Nil),
                    (Value::Nil, Value::Bool(_)) => Ok(Value::Nil),
                    (Value::Nil, Value::Nil) => Ok(Value::Nil),
                    _ => Err(format!(
                        "Type error in logical AND: {:?} AND {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::Or(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(*l || *r)),
                    (Value::Bool(l), Value::Nil) => Ok(Value::Bool(*l)),
                    (Value::Nil, Value::Bool(r)) => Ok(Value::Bool(*r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Nil),
                    _ => Err(format!(
                        "Type error in logical OR: {:?} OR {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::Not(expr) => {
                let val = expr.evaluate(context)?;
                match val {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    Value::Nil => Ok(Value::Bool(true)),
                    Value::Number(_) => Ok(Value::Bool(false)),
                    _ => Err(format!("Type error in logical NOT: !{:?}", val)),
                }
            }
            SyntaxNode::Equal(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                Ok(Value::Bool(left_val == right_val))
            }
            SyntaxNode::NotEqual(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                Ok(Value::Bool(left_val != right_val))
            }
            SyntaxNode::Less(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                    _ => Err(format!(
                        "Type error in less than comparison: {:?} < {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::LessEqual(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                    _ => Err(format!(
                        "Type error in less than or equal comparison: {:?} <= {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::Greater(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                    _ => Err(format!(
                        "Type error in greater than comparison: {:?} > {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::GreaterEqual(left, right) => {
                let left_val = left.evaluate(context)?;
                let right_val = right.evaluate(context)?;
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                    _ => Err(format!(
                        "Type error in greater than or equal comparison: {:?} >= {:?}",
                        left_val, right_val
                    )),
                }
            }
            SyntaxNode::Statement(e) => e.evaluate(context),
            SyntaxNode::Print(e) => {
                println!("{}", e.evaluate(context)?);
                Ok(Value::Nil)
            }
            SyntaxNode::Scoupe(v) => {
                for i in v {
                    match i {
                        SyntaxNode::Scoupe(_) => {
                            context.new_child_scoupe();
                            i.evaluate(context)?;
                            context.exit_scoupe();
                        }
                        _ => {
                            i.evaluate(context)?;
                        }
                    };
                }
                Ok(Value::Nil)
            }
            SyntaxNode::Variable(name, expr) => {
                if !context.has_in_current_scoupe(name) {
                    context.set_in_current_scoupe(name, Value::Nil);
                }
                let val = expr.evaluate(context)?;
                context.set_variable(name, val.clone());
                Ok(val)
            }
            SyntaxNode::Identifier(name) => {
                if let Some(val) = context.get_variable(name) {
                    Ok(val.clone())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            SyntaxNode::Assign(name, expr) => {
                if context.has_variable(name) {
                    let val = expr.evaluate(context)?;
                    context.set_variable(name, val.clone());
                    Ok(val)
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            SyntaxNode::IfElse(cond, true_st, false_st) => match cond.evaluate(context)? {
                Value::Bool(val) => {
                    if val {
                        Ok(true_st.evaluate(context)?)
                    } else if let Some(fs) = false_st {
                        Ok(fs.evaluate(context)?)
                    } else{
                        Ok(Value::Nil)
                    }
                }
                o => Err(format!("Expected Bool found: {}", o)),
            },
        }
    }
}
