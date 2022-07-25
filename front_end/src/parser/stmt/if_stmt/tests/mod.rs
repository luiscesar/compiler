use common::pointer::Pointer;

use crate::parser::{expr::{Expr, constant::TRUE}, stmt::{Stmt,env::Env}};

use super::IfStmt;

#[test]
fn test_parser_stmt_if_new_case1() {
    let expr = Expr::new_constant_expr_ptr(TRUE);
    let stmt = Pointer::new_pointer(Stmt::Null);
    let if_stmt_ptr_result = 
        IfStmt::new_ptr(&expr,&stmt);
    let if_stmt_ptr = if_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let stmt = Stmt::If(if_stmt_ptr);
    let result = stmt.gen(&env).unwrap();
    assert_eq!(result,());
}