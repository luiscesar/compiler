use common::pointer::Pointer;

use crate::parser::{expr::{types::INT, Expr, constant::Constant, relational::RelationalOperator}, stmt::{Stmt, env::Env, StmtType}};

use super::WhileStmt;

#[test]
fn test_parser_stmt_while_new_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let stmt = Pointer::new_pointer(Stmt::Null);
    let while_stmt_ptr_result = 
        WhileStmt::new_ptr(&condition,&stmt);
    let while_stmt_ptr = while_stmt_ptr_result.unwrap();
}

#[test]
fn test_parser_stmt_while_gen_case1() {
    let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    let condition = condition_result.unwrap();
    let stmt = Pointer::new_pointer(Stmt::Null);
    let while_stmt_ptr_result = 
            WhileStmt::new_ptr(&condition,&stmt);

    let while_stmt_ptr = while_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let result = while_stmt_ptr.gen(&env).unwrap();
    assert_eq!(result,());
}