use common::pointer::Pointer;

use crate::parser::{expr::{types::INT, constant::Constant, relational::RelationalOperator, Expr}, stmt::{Stmt, while_stmt::WhileStmt, env::Env, StmtType}};

use super::BreakStmt;

#[test]
fn test_parser_stmt_break_new_case1() {
     let id_name = String::from("x");
    let x = Expr::new_id_expr_ptr(id_name,INT);
    let one = Expr::new_constant_expr_ptr(Constant::INT(1));
    let condition_result = 
        Expr::new_relational_expr_ptr(RelationalOperator::LT, &x, &one);
    
    let condition = condition_result.unwrap();
    let break_stmt_ptr = BreakStmt::new_ptr();
    let stmt = Pointer::new_pointer(Stmt::Break(break_stmt_ptr));
    let while_stmt_ptr_result = 
            WhileStmt::new_ptr(&condition,&stmt);
    let while_stmt_ptr = while_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let result = while_stmt_ptr.gen(&env);
    if let Err(_) = result {
        assert!(false);
    }
}
