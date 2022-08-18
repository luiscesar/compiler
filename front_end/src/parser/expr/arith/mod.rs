use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::parser::expr::types::Typed;

use self::op::{ArithBinaryOperationPtr, ArithUnaryOperationPtr};

use super::{types::Type, ExprT};
pub mod op;

pub type ArithOperationPtr = Rc<ArithOperation>;

pub trait ArithOperationT: ExprT {}

#[derive(Debug, PartialEq)]
pub enum ArithOperation {
    BINARY(ArithBinaryOperationPtr),
    UNARY(ArithUnaryOperationPtr),
}

impl ArithOperation {}

impl ExprT for ArithOperation {
    fn reduce(&self) -> super::ExprPtr {
        match &self {
            ArithOperation::BINARY(x) => x.reduce(),
            ArithOperation::UNARY(x) => x.reduce(),
        }
    }
}
impl Typed for ArithOperation {
    fn element_type(&self) -> Type {
        match &self {
            ArithOperation::BINARY(x) => x.element_type(),
            ArithOperation::UNARY(x) => x.element_type(),
        }
    }
}
impl Display for ArithOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ArithOperation::BINARY(x) => x.fmt(f),
            ArithOperation::UNARY(x) => x.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests;
