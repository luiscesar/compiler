use std::rc::Rc;

use common::pointer::Pointer;

use crate::{
    error::{messages::IF_CONDITION_BOOLEAN_EXPECTED, CompilerError, SyntaxError},
    parser::expr::{
        types::{Typed, BOOL},
        ExprPtr, ExprT,
    },
};

use super::{emit_jump, emit_label, env::EnvMutPtr, new_label, StmtPtr, StmtType};

pub type IfStmtPtr = Rc<IfStmt>;

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    condition: ExprPtr,
    stmt: StmtPtr,
}
impl IfStmt {
    pub fn new(condition: &ExprPtr, stmt: &StmtPtr) -> Result<IfStmt, CompilerError> {
        if condition.element_type() == BOOL {
            Ok(IfStmt {
                condition: Pointer::clone(condition),
                stmt: Pointer::clone(stmt),
            })
        } else {
            Err(SyntaxError::compiler_error(IF_CONDITION_BOOLEAN_EXPECTED))
        }
    }
    pub fn new_ptr(condition: &ExprPtr, stmt: &StmtPtr) -> Result<IfStmtPtr, CompilerError> {
        let if_stmt = IfStmt::new(condition, stmt)?;
        Ok(Pointer::new_pointer(if_stmt))
    }
}
impl StmtType for IfStmt {
    fn gen(&self, env: &EnvMutPtr) -> Result<(), CompilerError> {
        let e = self.condition.reduce();
        let stmt_str = format!("iffalse {}", e.to_string());
        let label = new_label();
        emit_jump(stmt_str, label);
        self.stmt.gen(env)?;
        emit_label(label);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
