use std::rc::Rc;
use common::pointer::Pointer;
use crate::{parser::expr::{ExprPtr, types::{Typed, BOOL}, ExprT}, error::{CompilerError, SyntaxError, messages::IF_CONDITION_BOOLEAN_EXPECTED}};
use super::{StmtPtr, StmtType, env::EnvMutPtr, new_label, emit_jump, emit_label, emit_jump_stmt};


pub type IfElseStmtPtr = Rc<IfElseStmt>;
#[derive(Debug,PartialEq)]
pub struct IfElseStmt {
    condition:ExprPtr,
    if_stmt:StmtPtr,
    else_stmt:StmtPtr,
}
impl IfElseStmt {
    pub fn new(condition:&ExprPtr,if_stmt:&StmtPtr,else_stmt:&StmtPtr) 
        -> Result<IfElseStmt,CompilerError> {
        if condition.element_type() == BOOL {
            Ok(IfElseStmt{
                condition:Pointer::clone(condition),
                if_stmt:Pointer::clone(if_stmt),
                else_stmt:Pointer::clone(else_stmt)
            })
        } else {
            Err(SyntaxError::compiler_error(IF_CONDITION_BOOLEAN_EXPECTED))
        }
    }

    pub fn new_ptr(condition:&ExprPtr,if_stmt:&StmtPtr,else_stmt:&StmtPtr) 
        -> Result<IfElseStmtPtr,CompilerError> {
            let ifelse_stmt = IfElseStmt::new(condition,if_stmt,else_stmt)?;
            Ok(Pointer::new_pointer(ifelse_stmt))
    }
}

impl StmtType for IfElseStmt {
    fn gen(&self,env:&EnvMutPtr) -> Result<(),CompilerError> {
        let e = self.condition.reduce();
        let else_begin_label = new_label();
        let if_end_label = new_label();
        let if_stmt_str = format!("iffalse {}",e.to_string());
        emit_jump(if_stmt_str,else_begin_label);
        self.if_stmt.gen(env)?;
        emit_jump_stmt(if_end_label);
        emit_label(else_begin_label);
        self.else_stmt.gen(env)?;
        emit_label(if_end_label);
        Ok(())
    }
}

#[cfg(test)]
mod tests;