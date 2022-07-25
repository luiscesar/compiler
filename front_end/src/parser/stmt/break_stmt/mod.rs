use std::rc::Rc;

use common::pointer::Pointer;

use crate::error::{CompilerError, SyntaxError, messages::BREAK_WITHOUT_LOOP};

use super::{StmtType, emit_jump_stmt};


pub type BreakStmtPtr = Rc<BreakStmt>;
#[derive(Debug,PartialEq)]
pub struct BreakStmt {}
impl BreakStmt {
    pub fn new() -> BreakStmt {
        BreakStmt { }
    }
    pub fn new_ptr() -> BreakStmtPtr {
        Pointer::new_pointer(BreakStmt::new())
    }
}
impl StmtType for BreakStmt {
    fn gen(&self,env:&super::env::EnvMutPtr) -> Result<(),CompilerError> {
        let label_option = env.borrow_mut().top_loop_label();
        if let Some(label) = label_option {
            emit_jump_stmt(label);
            Ok(())
        } else {
            Err(SyntaxError::compiler_error(BREAK_WITHOUT_LOOP))
        }
    }
}
#[cfg(test)]
mod tests;