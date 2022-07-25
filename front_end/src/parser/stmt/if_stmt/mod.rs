use std::rc::Rc;

use common::pointer::Pointer;

use crate::{parser::expr::{ExprPtr, types::{Typed, BOOL}, ExprT}, error::{CompilerError, SyntaxError, messages::IF_CONDITION_BOOLEAN_EXPECTED}};

use super::{StmtType, StmtPtr, emit_label, emit_jump, new_label, env::EnvMutPtr};

pub type IfStmtPtr = Rc<IfStmt>;

#[derive(Debug,PartialEq)]
pub struct IfStmt {
    condition:ExprPtr,
    stmt:StmtPtr,
}
impl IfStmt {
    pub fn new(condition:&ExprPtr,stmt:&StmtPtr) -> Result<IfStmt,CompilerError> {
        if condition.element_type() == BOOL {
            Ok(IfStmt{condition:Pointer::clone(condition),stmt:Pointer::clone(stmt)})
        } else {
            Err(SyntaxError::compiler_error(IF_CONDITION_BOOLEAN_EXPECTED))
        }
    }
    pub fn new_ptr(condition:&ExprPtr,stmt:&StmtPtr) -> Result<IfStmtPtr,CompilerError> {
        let if_stmt = IfStmt::new(condition,stmt)?;
        Ok(Pointer::new_pointer(if_stmt))
    }
}
impl StmtType for IfStmt {
    fn gen(&self,env:&EnvMutPtr) -> Result<(),CompilerError> {
        let e = self.condition.reduce();
        let stmt_str = format!("iffalse {}",e.to_string());
        let label = new_label();
        emit_jump(stmt_str,label);
        self.stmt.gen(env)?;
        emit_label(label);
        Ok(())
    }
}

#[cfg(test)]
mod tests;