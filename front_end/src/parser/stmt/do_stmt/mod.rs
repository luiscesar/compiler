use std::rc::Rc;

use common::pointer::Pointer;

use crate::{parser::expr::{ExprPtr, types::{Typed, BOOL}, ExprT}, error::{CompilerError, SyntaxError, messages::DO_CONDITION_BOOLEAN_EXPECTED}};

use super::{StmtPtr, StmtType, env::EnvMutPtr, new_label, emit_label, emit_jump};


pub type DoStmtPtr = Rc<DoStmt>;
#[derive(Debug,PartialEq)]
pub struct DoStmt {
    condition:ExprPtr,
    stmt:StmtPtr,
}
impl DoStmt {
    pub fn new(condition:&ExprPtr,stmt:&StmtPtr) -> Result<DoStmt,CompilerError> {
        if condition.element_type() == BOOL {
            Ok(DoStmt{condition:Pointer::clone(condition),stmt:Pointer::clone(stmt)})
        } else {
            Err(SyntaxError::compiler_error(DO_CONDITION_BOOLEAN_EXPECTED))
        }
    }
    pub fn new_ptr(condition:&ExprPtr,stmt:&StmtPtr) -> Result<DoStmtPtr,CompilerError> {
        let do_stmt = DoStmt::new(condition,stmt)?;
        Ok(Pointer::new_pointer(do_stmt))
    }
}
impl StmtType for DoStmt {
    fn gen(&self,env:&EnvMutPtr) -> Result<(),CompilerError> {
        let begin_label = new_label();
        let end_label = new_label();
        emit_label(begin_label);
        env.borrow_mut().push_loop_label(end_label);
        self.stmt.gen(env)?;
        let e = self.condition.reduce();
        let do_stmt_str = format!("if {}",e.to_string());
        emit_jump(do_stmt_str, begin_label);
        env.borrow_mut().pop_loop_label();
        emit_label(end_label);
        Ok(())
    }
}

#[cfg(test)]
mod tests;