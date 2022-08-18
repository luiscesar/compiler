use super::SeqStmt;
use crate::parser::{
    expr::{
        constant::{Constant, TRUE},
        id::Id,
        types::INT,
        Expr,
    },
    stmt::{env::Env, if_stmt::IfStmt, set_stmt::SetStmt, Stmt, StmtType},
};
use common::pointer::Pointer;

#[test]
fn test_parser_stmt_seq_new_case1() {
    let id_ptr = Id::new_ptr(Pointer::new_pointer("x".to_string()), INT);
    let const_expr_ptr = Expr::new_constant_expr_ptr(Constant::INT(1));
    let set_stmt_ptr_result = SetStmt::new_ptr(&id_ptr, &const_expr_ptr);
    let set_stmt_ptr = set_stmt_ptr_result.unwrap();
    let expr = Expr::new_constant_expr_ptr(TRUE);
    let stmt = Pointer::new_pointer(Stmt::Null);
    let if_stmt_ptr_result = IfStmt::new_ptr(&expr, &stmt);
    let if_stmt_ptr = if_stmt_ptr_result.unwrap();
    let first = Stmt::Set(set_stmt_ptr);
    let second = Stmt::If(if_stmt_ptr);
    let first_ptr = Pointer::new_pointer(first);
    let second_ptr = Pointer::new_pointer(second);
    let seq_stmt_ptr_result = SeqStmt::new_ptr(&first_ptr, &second_ptr);
}

#[test]
fn test_parser_stmt_seq_gen_case1() {
    let id_ptr = Id::new_ptr(Pointer::new_pointer("x".to_string()), INT);
    let const_expr_ptr = Expr::new_constant_expr_ptr(Constant::INT(1));
    let set_stmt_ptr_result = SetStmt::new_ptr(&id_ptr, &const_expr_ptr);
    let set_stmt_ptr = set_stmt_ptr_result.unwrap();
    let expr = Expr::new_constant_expr_ptr(TRUE);
    let stmt = Pointer::new_pointer(Stmt::Null);
    let if_stmt_ptr_result = IfStmt::new_ptr(&expr, &stmt);
    let if_stmt_ptr = if_stmt_ptr_result.unwrap();
    let first = Stmt::Set(set_stmt_ptr);
    let second = Stmt::If(if_stmt_ptr);
    let first_ptr = Pointer::new_pointer(first);
    let second_ptr = Pointer::new_pointer(second);
    let seq_stmt_ptr_result = SeqStmt::new_ptr(&first_ptr, &second_ptr);
    let env = Env::new_mut_ptr();
    let result = seq_stmt_ptr_result.gen(&env).unwrap();
    assert_eq!(result, ());
}
