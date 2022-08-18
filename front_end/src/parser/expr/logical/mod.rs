use self::op::{LogicalBinaryOperationPtr, LogicalUnaryOperationPtr};
use super::{
    types::{Type, Typed, BOOL},
    ExprT,
};
use std::{fmt::Display, rc::Rc};

pub mod op;

pub type LogicalOperationPtr = Rc<LogicalOperation>;

pub trait LogicalOperationT: ExprT {}

#[derive(Debug, PartialEq)]
pub enum LogicalOperation {
    BINARY(LogicalBinaryOperationPtr),
    UNARY(LogicalUnaryOperationPtr),
}

impl ExprT for LogicalOperation {
    fn reduce(&self) -> super::ExprPtr {
        match &self {
            LogicalOperation::BINARY(x) => x.reduce(),
            LogicalOperation::UNARY(x) => x.reduce(),
        }
    }
}
impl Typed for LogicalOperation {
    fn element_type(&self) -> Type {
        BOOL
    }
}
impl Display for LogicalOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LogicalOperation::BINARY(x) => x.fmt(f),
            LogicalOperation::UNARY(x) => x.fmt(f),
        }
    }
}

impl ExprT for LogicalOperationPtr {
    fn reduce(&self) -> super::ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for LogicalOperationPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

#[cfg(test)]
mod tests;
