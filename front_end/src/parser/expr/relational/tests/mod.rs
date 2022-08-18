use crate::parser::expr::{
    arith::op::ArithBinaryOperator, constant::Constant, id::Id, types::INT, Expr, ExprT,
};

use super::RelationalOperator;

#[test]
fn test_parser_expr_relational_new_case1() {
    let id_name = "x".to_string();
    let id_expr_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::INT(2);
    let constant_expr_ptr = Expr::new_constant_expr_ptr(constant);

    let operator = RelationalOperator::LT;

    let relational_expr_result =
        Expr::new_relational_expr_ptr(operator, &id_expr_ptr, &constant_expr_ptr);

    if let Ok(relational_expr) = relational_expr_result {
        assert_eq!((*relational_expr).to_string(), "x < 2");
    } else {
        assert!(false)
    }
}

#[test]
fn test_parser_expr_relational_new_case2() {
    let id_name = "x".to_string();
    let id_expr_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::INT(2);
    let constant_expr_ptr = Expr::new_constant_expr_ptr(constant);

    let expr1_result = Expr::new_arith_binary_operation_expr_ptr(
        ArithBinaryOperator::PLUS,
        &id_expr_ptr,
        &constant_expr_ptr,
    );
    let expr1_ptr = expr1_result.unwrap();

    let count = Id::count();
    let expected = format!("y < t{}", count);

    let expr_ptr = expr1_ptr.reduce();
    let id_name = "y".to_string();
    let id_expr_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let operator = RelationalOperator::LT;

    let relational_expr_result = Expr::new_relational_expr_ptr(operator, &id_expr_ptr, &expr_ptr);

    let relational_expr = relational_expr_result.unwrap();

    assert_eq!((*relational_expr).to_string(), expected);
}
