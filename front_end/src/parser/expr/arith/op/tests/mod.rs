
use common::pointer::Pointer;
use crate::parser::{expr::types::{INT, FLOAT, Typed}, expr::{constant::Constant,Expr}};

use super::ArithBinaryOperation;


#[test]
fn test_parser_expr_arith_op_binary_new_case1() {
    let id_name = "x".to_string();
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name,INT);

    let constant = Constant::INT(2);
    let expr_constant_ptr = Expr::new_constant_expr_ptr(constant);

    let binary_op_plus_result = 
        ArithBinaryOperation::new(super::ArithBinaryOperator::PLUS,
            &expr_id_ptr,&expr_constant_ptr);

    let binary_op_plus = binary_op_plus_result.unwrap();
    assert_eq!(binary_op_plus.element_type(),INT);
}

#[test]
fn test_parser_expr_arith_op_binary_new_case2() {
    let id_name = "x".to_string();
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name,INT);

    let constant = Constant::FLOAT(2.0);
    let expr_constant_ptr = Expr::new_constant_expr_ptr(constant);
    let binary_op_plus_result = 
        ArithBinaryOperation::new(super::ArithBinaryOperator::PLUS,
            &expr_id_ptr,&expr_constant_ptr);

    let binary_op_plus =binary_op_plus_result.unwrap();
    assert_eq!(binary_op_plus.element_type(),FLOAT);
}