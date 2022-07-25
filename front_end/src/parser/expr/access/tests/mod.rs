use common::pointer::Pointer;

use crate::{parser::expr::{types::{INT, BasicType, Typed}, Expr, constant::Constant}, error::messages::ARRAY_ACCESS_INDEX_INTEGER};

use super::Access;

#[test]
fn test_parser_expr_access_new_case1() {
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let constant = Constant::INT(2);
    let index = Expr::new_constant_expr_ptr(constant);
    // access expr result
    let access_expr_result = 
        Access::new_ptr(&name,BasicType::Int,&index);
    // access expr
    let access_expr = access_expr_result.unwrap();
    // Check array name
    assert_eq!(*access_expr.array_name,"a".to_string());
    // Check array type
    assert_eq!(access_expr.element_type(),INT);
    // Check display
    assert_eq!(access_expr.to_string(),"a[2]".to_string());
}

#[test]
fn test_parser_expr_access_new_case2() {
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let id_name = String::from("x");
    let index = Expr::new_id_expr_ptr(id_name,INT);
    // access expr result
    let access_expr_result = 
        Access::new_ptr(&name,BasicType::Int,&index);
    // access expr
    let access_expr = access_expr_result.unwrap();
    // Check array name
    assert_eq!(*access_expr.array_name,"a".to_string());
    // Check array type
    assert_eq!(access_expr.element_type(),INT);
    // Check display
    assert_eq!(access_expr.to_string(),"a[x]".to_string());
}

#[test]
fn test_parser_expr_access_new_case3() {
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let constant = Constant::FLOAT(2.0);
    let index = Expr::new_constant_expr_ptr(constant);
    let error = 
        Access::new_ptr(&name,BasicType::Int,&index).unwrap_err();
    let expected = ARRAY_ACCESS_INDEX_INTEGER;
    assert!(error.to_string().contains(expected));
}
