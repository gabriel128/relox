use std::fmt::Display;

use crate::errors::ReloxError;
use crate::Result;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Number(f32),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::Nil => write!(f,"nil")
        }

    }
}

impl std::ops::Add for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs + rhs)),
            _ => Err(ReloxError::new_fatal_error("Tried to add oranges with apples".to_string()))
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs - rhs)),
            _ => Err(ReloxError::new_fatal_error("Tried to substract oranges with apples".to_string()))
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs * rhs)),
            _ => Err(ReloxError::new_fatal_error("Tried to multiply oranges with apples".to_string()))
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs / rhs)),
            _ => Err(ReloxError::new_fatal_error("Tried to divide oranges with apples".to_string()))
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Value>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(val) => Ok(Self::Number(-val)),
            _ => Err(ReloxError::new_fatal_error("Tried to negate unegable(?)".to_string()))
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}
