use common::pointer::Pointer;

use crate::parser::{expr::{types::INT, Expr, constant::Constant, relational::RelationalOperator}, stmt::{Stmt, env::Env, StmtType}};

use super::DoStmt;

#[test]
fn test_parser_stmt_do_new_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let stmt = Pointer::new_pointer(Stmt::Null);
    let do_stmt_ptr_result = 
            DoStmt::new_ptr(&condition,&stmt);
    let do_stmt_ptr = do_stmt_ptr_result.unwrap();
}

#[test]
fn test_parser_stmt_do_gen_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let stmt = Pointer::new_pointer(Stmt::Null);
    let do_stmt_ptr_result = 
            DoStmt::new_ptr(&condition,&stmt);
    let do_stmt_ptr = do_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let result = do_stmt_ptr.gen(&env).unwrap();
}