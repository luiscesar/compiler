use crate::error::CompilerError;
use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use self::{
    break_stmt::BreakStmtPtr, do_stmt::DoStmtPtr, env::EnvMutPtr, if_stmt::IfStmtPtr,
    ifelse_stmt::IfElseStmtPtr, seq_stmt::SeqStmtPtr, set_elem_stmt::SetElemStmtPtr,
    set_stmt::SetStmtPtr, while_stmt::WhileStmtPtr,
};

pub mod break_stmt;
pub mod do_stmt;
pub mod env;
pub mod if_stmt;
pub mod ifelse_stmt;
pub mod seq_stmt;
pub mod set_elem_stmt;
pub mod set_stmt;
pub mod while_stmt;

static LABEL: AtomicUsize = AtomicUsize::new(0);

pub fn new_label() -> usize {
    LABEL.fetch_add(1, Ordering::Relaxed)
}
pub fn label() -> usize {
    LABEL.fetch_add(0, Ordering::Relaxed)
}
pub fn emit_label(label: usize) {
    print!("L{}: ", label);
}
pub fn emit_jump(stmt_str: String, label: usize) {
    println!("{} goto L{}", stmt_str, label);
}
pub fn emit_jump_stmt(label: usize) {
    println!("goto L{}", label);
}
pub trait StmtType {
    fn gen(&self, env: &EnvMutPtr) -> Result<(), CompilerError>;
}

pub type StmtPtr = Rc<Stmt>;
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Null,
    If(IfStmtPtr),
    IfElse(IfElseStmtPtr),
    While(WhileStmtPtr),
    Do(DoStmtPtr),
    Break(BreakStmtPtr),
    Set(SetStmtPtr),
    SetElem(SetElemStmtPtr),
    Seq(SeqStmtPtr),
}

impl Stmt {
    pub fn gen(&self, env: &EnvMutPtr) -> Result<(), CompilerError> {
        match &self {
            Stmt::Null => Ok(()),
            Stmt::If(x) => x.gen(env),
            Stmt::IfElse(x) => x.gen(env),
            Stmt::While(x) => x.gen(env),
            Stmt::Do(x) => x.gen(env),
            Stmt::Break(x) => x.gen(env),
            Stmt::Set(x) => x.gen(env),
            Stmt::SetElem(x) => x.gen(env),
            Stmt::Seq(x) => x.gen(env),
        }
    }
}

#[cfg(test)]
mod tests;
