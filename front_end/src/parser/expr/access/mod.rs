use std::{fmt::Display, rc::Rc};

use super::{
    id::Id,
    types::{BasicType, Type, TypePtr, Typed, INT},
    ExprPtr, ExprT,
};
use crate::{
    error::{messages::ARRAY_ACCESS_INDEX_INTEGER, CompilerError, SyntaxError},
    parser::expr::Expr,
};
use common::collections::string::StringPtr;
use common::pointer::Pointer;

pub type AccessPtr = Rc<Access>;
#[derive(Debug, PartialEq)]
pub struct Access {
    array_name: StringPtr,
    array_type: BasicType,
    index: ExprPtr,
}
impl Access {
    pub fn new(
        array_name: &StringPtr,
        array_type: BasicType,
        index: &ExprPtr,
    ) -> Result<Access, CompilerError> {
        if index.element_type() == INT {
            let access_obj = Access {
                array_name: Pointer::clone(array_name),
                array_type: array_type,
                index: Pointer::clone(index),
            };
            Ok(access_obj)
        } else {
            Err(SyntaxError::compiler_error(ARRAY_ACCESS_INDEX_INTEGER))
        }
    }

    pub fn new_ptr(
        array_name: &StringPtr,
        array_type: BasicType,
        index: &ExprPtr,
    ) -> Result<AccessPtr, CompilerError> {
        let access = Access::new(array_name, array_type, index)?;
        Ok(Pointer::new_pointer(access))
    }

    pub fn reduce_loc(&self) -> AccessPtr {
        let e = self.index.reduce();
        let access_obj = Access {
            array_name: Pointer::clone(&self.array_name),
            array_type: self.array_type,
            index: Pointer::clone(&e),
        };
        let access_new_ptr = Pointer::new_pointer(access_obj);
        access_new_ptr
    }
}
impl ExprT for Access {
    fn reduce(&self) -> ExprPtr {
        let e = self.index.reduce();
        let t = Id::new_id_tmp(self.element_type());
        println!(
            "{} = {}[{}]",
            t.to_string(),
            &self.array_name,
            e.to_string()
        );
        let p = Pointer::new_pointer(t);
        let expr = Expr::ID(p);
        Pointer::new_pointer(expr)
    }
}

impl Typed for Access {
    fn element_type(&self) -> Type {
        Type::Basic(self.array_type)
    }
}
impl Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.array_name, self.index.to_string())
    }
}
impl ExprT for AccessPtr {
    fn reduce(&self) -> super::ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for AccessPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

#[cfg(test)]
mod tests;
