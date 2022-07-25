use std::rc::Rc;

use common::pointer::Pointer;

use crate::{parser::expr::{ExprPtr, types::{Typed, BOOL}, ExprT}, error::{CompilerError, SyntaxError, messages::WHILE_CONDITION_BOOLEAN_EXPECTED}};

use super::{StmtPtr, StmtType, env::EnvMutPtr, new_label, emit_label, emit_jump, emit_jump_stmt};

pub type WhileStmtPtr = Rc<WhileStmt>;

#[derive(Debug,PartialEq)]
pub struct WhileStmt {
    condition:ExprPtr,
    stmt:StmtPtr,
}
impl WhileStmt {
    pub fn new(condition:&ExprPtr,stmt:&StmtPtr) -> Result<WhileStmt,CompilerError> {
        if condition.element_type() == BOOL {
            Ok(WhileStmt{condition:Pointer::clone(condition),stmt:Pointer::clone(stmt)})
        } else {
            Err(SyntaxError::compiler_error(WHILE_CONDITION_BOOLEAN_EXPECTED))
        }
    }
    pub fn new_ptr(condition:&ExprPtr,stmt:&StmtPtr) -> Result<WhileStmtPtr,CompilerError> {
        let while_stmt = WhileStmt::new(condition,stmt)?;
        Ok(Pointer::new_pointer(while_stmt))
    }
}
impl StmtType for WhileStmt {
    fn gen(&self,env:&EnvMutPtr) -> Result<(),CompilerError> {
        let e = self.condition.reduce();
        let begin_label = new_label();
        let end_label = new_label();
        env.borrow_mut().push_loop_label(end_label);

        emit_label(begin_label);
        let while_stmt_str = format!("iffalse {}",e.to_string());
        emit_jump(while_stmt_str,end_label);
        self.stmt.gen(env)?;
        emit_jump_stmt(begin_label);
        emit_label(end_label);
        
        env.borrow_mut().pop_loop_label();
        Ok(())
    }
}
#[cfg(test)]
mod tests;