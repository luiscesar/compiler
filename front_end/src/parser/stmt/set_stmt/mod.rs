use std::rc::Rc;

use common::pointer::Pointer;

use crate::{parser::expr::{id::IdPtr, ExprPtr, types::Typed, ExprT}, error::{CompilerError, SyntaxError, messages::VALUES_ASSIGNMENT_EQUAL_TYPES}};

use super::StmtType;


pub type SetStmtPtr = Rc<SetStmt>;
#[derive(Debug,PartialEq)]
pub struct SetStmt {
    id:IdPtr,
    expr:ExprPtr,
}
impl SetStmt {
    pub fn new(id:&IdPtr,expr:&ExprPtr) -> Result<SetStmt,CompilerError> {
        if id.element_type() == expr.element_type() {
            Ok(SetStmt{id:Pointer::clone(id),expr:Pointer::clone(expr)})
        } else {
            Err(SyntaxError::compiler_error(VALUES_ASSIGNMENT_EQUAL_TYPES))
        }
    }
    pub fn new_ptr(id:&IdPtr,expr:&ExprPtr) -> Result<SetStmtPtr,CompilerError> {
        let set_stmt = SetStmt::new(id,expr)?;
        Ok(Pointer::new_pointer(set_stmt))
    }
}
impl StmtType for SetStmt {
    fn gen(&self,env:&super::env::EnvMutPtr) -> Result<(),CompilerError> {
        let e = self.expr.reduce();
        println!("{} = {}",self.id.to_string(),e.to_string());
        Ok(())
    }
}
#[cfg(test)]
mod tests;