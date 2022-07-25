use common::pointer::Pointer;

use crate::{parser::{expr::{Expr, types::{INT}, constant::Constant, id::Id}, stmt::{env::Env, StmtType}}, error::messages::VALUES_ASSIGNMENT_EQUAL_TYPES};
use super::SetStmt;

#[test]
fn test_parser_stmt_set_new_case1() {
    let id_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    let const_expr_ptr = Expr::new_constant_expr_ptr(Constant::INT(1));
    let set_stmt_ptr_result = 
        SetStmt::new_ptr(&id_ptr,&const_expr_ptr);
    let set_stmt_ptr = set_stmt_ptr_result.unwrap();
}

#[test]
fn test_parser_stmt_set_new_case2() {
    let id_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    let const_expr_ptr = Expr::new_constant_expr_ptr(Constant::FLOAT(1.0));
    let set_stmt_ptr_result = 
        SetStmt::new(&id_ptr,&const_expr_ptr);
    let error = set_stmt_ptr_result.unwrap_err();
    let expected = VALUES_ASSIGNMENT_EQUAL_TYPES;
    assert!(error.to_string().contains(expected));
}

#[test]
fn test_parser_stmt_set_gen_case1() {
    let id_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    let const_expr_ptr = Expr::new_constant_expr_ptr(Constant::INT(1));
    let set_stmt_ptr_result = 
        SetStmt::new(&id_ptr,&const_expr_ptr);
    let set_stmt_ptr = set_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let gen_result = set_stmt_ptr.gen(&env).unwrap();
    assert_eq!(gen_result,());
}