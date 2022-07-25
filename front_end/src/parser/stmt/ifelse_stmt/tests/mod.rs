use common::pointer::Pointer;

use crate::{parser::{expr::{Expr, types::INT, constant::{Constant}, relational::RelationalOperator}, stmt::{Stmt, StmtType, env::Env}}, error::messages::IF_CONDITION_BOOLEAN_EXPECTED};

use super::IfElseStmt;

#[test]
fn test_parser_stmt_ifelse_new_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let if_stmt = Pointer::new_pointer(Stmt::Null);
    let else_stmt = Pointer::new_pointer(Stmt::Null);
    let ifelse_stmt_ptr_result = 
            IfElseStmt::new_ptr(&condition,&if_stmt,&else_stmt);
    let ifelse_stmt_ptr = ifelse_stmt_ptr_result.unwrap();
}

#[test]
fn test_parser_stmt_ifelse_new_case2() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT); 
    let if_stmt = Pointer::new_pointer(Stmt::Null);
    let else_stmt = Pointer::new_pointer(Stmt::Null);

    let ifelse_stmt_result = 
        IfElseStmt::new_ptr(&x,&if_stmt,&else_stmt);
    if let Err(error) = ifelse_stmt_result {
        let expected = IF_CONDITION_BOOLEAN_EXPECTED;
        assert!(error.to_string().contains(expected));
    } else {
        assert!(false);
    }      
}

#[test]
fn test_parser_stmt_ifelse_gen_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let if_stmt = Pointer::new_pointer(Stmt::Null);
    let else_stmt = Pointer::new_pointer(Stmt::Null);
    let ifelse_stmt_ptr_result = 
            IfElseStmt::new_ptr(&condition,&if_stmt,&else_stmt);
    let ifelse_stmt_ptr = ifelse_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let result = ifelse_stmt_ptr.gen(&env).unwrap();
    assert_eq!(result,());
}