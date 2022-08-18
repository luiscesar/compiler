use std::rc::Rc;

use common::pointer::Pointer;

use super::{StmtPtr, StmtType};

pub type SeqStmtPtr = Rc<SeqStmt>;
#[derive(Debug, PartialEq)]
pub struct SeqStmt {
    first: StmtPtr,
    second: StmtPtr,
}
impl SeqStmt {
    pub fn new(first: &StmtPtr, second: &StmtPtr) -> SeqStmt {
        SeqStmt {
            first: Pointer::clone(first),
            second: Pointer::clone(second),
        }
    }
    pub fn new_ptr(first: &StmtPtr, second: &StmtPtr) -> SeqStmtPtr {
        let seq_stmt = SeqStmt::new(first, second);
        Pointer::new_pointer(seq_stmt)
    }
}
impl StmtType for SeqStmt {
    fn gen(&self, env: &super::env::EnvMutPtr) -> Result<(), crate::error::CompilerError> {
        self.first.gen(env)?;
        self.second.gen(env)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests;
