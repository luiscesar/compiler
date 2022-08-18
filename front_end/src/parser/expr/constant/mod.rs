use std::{
    fmt::{self, Display},
    rc::Rc,
};

use common::pointer::Pointer;

use crate::parser::expr::types::{Typed, BOOL, FLOAT, INT};

use super::{types::Type, Expr, ExprPtr, ExprT};

pub const TRUE: Constant = Constant::BOOL(true);
pub const FALSE: Constant = Constant::BOOL(false);

pub type ConstantPtr = Rc<Constant>;

#[derive(Debug, PartialEq)]
pub enum Constant {
    INT(i32),
    FLOAT(f64),
    BOOL(bool),
}

impl Typed for Constant {
    fn element_type(&self) -> Type {
        match &self {
            Constant::INT(_) => INT,
            Constant::FLOAT(_) => FLOAT,
            Constant::BOOL(_) => BOOL,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Constant::INT(x) => write!(f, "{}", x),
            Constant::FLOAT(x) => write!(f, "{}", x),
            Constant::BOOL(x) => write!(f, "{}", x),
        }
    }
}

impl ExprT for ConstantPtr {
    fn reduce(&self) -> ExprPtr {
        Pointer::new_pointer(Expr::CONST(Pointer::clone(self)))
    }
}
impl Typed for ConstantPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

#[cfg(test)]
mod tests;
