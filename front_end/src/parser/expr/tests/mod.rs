use common::pointer::Pointer;

use super::arith::ArithOperation;
use super::constant::TRUE;
use super::relational::RelationalOperator;
use super::types::INT;
use super::Expr;
use super::ExprT;
use crate::error::messages::ARITH_OPERATION_NUMBERS_EXPECTED;
use crate::parser::expr::arith::op::{ArithBinaryOperation, ArithBinaryOperator};
use crate::parser::expr::constant::Constant;
use crate::parser::expr::id::Id;
use crate::parser::expr::types::{Typed, FLOAT};

#[test]
fn test_parser_expr_reduce_case1() {
    let id_name = String::from("x");
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::FLOAT(2.0);
    let expr_constant_ptr = Expr::new_constant_expr_ptr(constant);

    let binary_op_plus_result =
        ArithBinaryOperation::new(ArithBinaryOperator::PLUS, &expr_id_ptr, &expr_constant_ptr);

    let binary_op_plus = binary_op_plus_result.unwrap();
    assert_eq!(binary_op_plus.element_type(), FLOAT);
}

#[test]
fn test_parser_expr_reduce_case2() {
    let id_name = String::from("x");
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::FLOAT(2.5);
    let expr_constant_ptr = Expr::new_constant_expr_ptr(constant);

    let binary_op_plus_result =
        ArithBinaryOperation::new(ArithBinaryOperator::PLUS, &expr_id_ptr, &expr_constant_ptr);

    let binary_op_plus = binary_op_plus_result.unwrap();

    let id_name = String::from("x");
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let binary_op_plus_ptr = Pointer::new_pointer(binary_op_plus);
    let expr_binary_op_plus_ptr =
        Pointer::new_pointer(Expr::ARITH(ArithOperation::BINARY(binary_op_plus_ptr)));

    let binary_op_minus_result = ArithBinaryOperation::new(
        ArithBinaryOperator::MINUS,
        &expr_id_ptr,
        &expr_binary_op_plus_ptr,
    );

    let binary_op_minus = binary_op_minus_result.unwrap();

    let t = binary_op_minus.reduce();

    let count = Id::count() - 1;
    let expected = format!("t{}", count);

    assert_eq!(t.to_string(), expected);
}

#[test]
fn test_parser_expr_reduce_case3() {
    let id_name = String::from("x");
    let expr_id_ptr = Expr::new_id_expr_ptr(id_name, INT);

    let constant = TRUE;
    let expr_constant_ptr = Expr::new_constant_expr_ptr(constant);

    let binary_op_plus_result =
        ArithBinaryOperation::new(ArithBinaryOperator::PLUS, &expr_id_ptr, &expr_constant_ptr);
    if let Err(x) = binary_op_plus_result {
        let msg = x.as_ref().to_string();
        let expected = ARITH_OPERATION_NUMBERS_EXPECTED;
        assert!(msg.contains(expected));
    } else {
        assert!(false)
    }
}

#[test]
fn test_parser_expr_reduce_case4() {
    let id_name = "x".to_string();
    let expr1 = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::INT(1);
    let expr2 = Expr::new_constant_expr_ptr(constant);

    let expr3 =
        Expr::new_arith_binary_operation_expr_ptr(ArithBinaryOperator::PLUS, &expr1, &expr2);
    let x = expr3.unwrap();
    assert_eq!(x.element_type(), INT);
}

#[test]
fn test_parser_expr_reduce_case5() {
    let id_name = "x".to_string();
    let expr1 = Expr::new_id_expr_ptr(id_name, INT);

    let constant = Constant::INT(1);
    let expr2 = Expr::new_constant_expr_ptr(constant);

    let expr3_result =
        Expr::new_arith_binary_operation_expr_ptr(ArithBinaryOperator::PLUS, &expr1, &expr2);

    let expr3 = expr3_result.unwrap();
    let constant = Constant::INT(1);
    let expr4 = Expr::new_constant_expr_ptr(constant);
    let rel_expr_result = Expr::new_relational_expr_ptr(RelationalOperator::LT, &expr3, &expr4);

    let rel_expr = rel_expr_result.unwrap();
    let count = Id::count();
    let expected = format!("t{} < 1", count);

    let rel_expr2 = rel_expr.reduce();
    assert_eq!(rel_expr2.to_string(), expected);
}
