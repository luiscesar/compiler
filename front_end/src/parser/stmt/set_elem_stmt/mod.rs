use std::rc::Rc;

use common::pointer::Pointer;

use crate::{parser::expr::{access::AccessPtr, ExprPtr, types::Typed, ExprT}, error::{CompilerError, SyntaxError, messages::VALUES_ASSIGNMENT_EQUAL_TYPES}};

use super::StmtType;


pub type SetElemStmtPtr = Rc<SetElemStmt>;
#[derive(Debug,PartialEq)]
pub struct SetElemStmt {
    access:AccessPtr,
    expr:ExprPtr,
}
impl SetElemStmt {
    pub fn new(access:&AccessPtr,expr:&ExprPtr) -> Result<SetElemStmt,CompilerError> {
        if access.element_type() == expr.element_type() {
            Ok(SetElemStmt{access:Pointer::clone(access),expr:Pointer::clone(expr)})
        } else {
            Err(SyntaxError::compiler_error(VALUES_ASSIGNMENT_EQUAL_TYPES))
        }
    }
    pub fn new_ptr(access:&AccessPtr,expr:&ExprPtr) -> Result<SetElemStmtPtr,CompilerError> {
        let set_elem_stmt = SetElemStmt::new(access,expr)?;
        Ok(Pointer::new_pointer(set_elem_stmt))
    }
}
impl StmtType for SetElemStmt {
    fn gen(&self,env:&super::env::EnvMutPtr) -> Result<(),CompilerError> {
        let a = self.access.reduce_loc();
        let e = self.expr.reduce();
        println!("{} = {}",a.to_string(),e.to_string());
        Ok(())
    }
}
#[cfg(test)]
mod tests;