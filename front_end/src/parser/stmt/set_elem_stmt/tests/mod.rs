use common::pointer::Pointer;

use crate::{
    error::messages::VALUES_ASSIGNMENT_EQUAL_TYPES,
    parser::{
        expr::{
            access::Access,
            constant::Constant,
            types::{basic_type, BasicType, Type},
            Expr,
        },
        stmt::{env::Env, StmtType},
    },
};

use super::SetElemStmt;

#[test]
fn test_parser_stmt_set_elem_new_case1() {
    // array type
    let array_type = Type::Array(BasicType::Int, 10);
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let constant = Constant::INT(2);
    let index = Expr::new_constant_expr_ptr(constant);
    // access expr ptr
    let access_ptr_result = Access::new_ptr(&name, basic_type(array_type), &index);
    let access_ptr = access_ptr_result.unwrap();
    let constant = Constant::INT(5);
    let expr_ptr = Expr::new_constant_expr_ptr(constant);
    let set_elem_stmt_ptr_result = SetElemStmt::new_ptr(&access_ptr, &expr_ptr);
    let set_elem_stmt_ptr = set_elem_stmt_ptr_result.unwrap();
}

#[test]
fn test_parser_stmt_set_elem_error_case1() {
    // array type
    let array_type = Type::Array(BasicType::Int, 10);
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let constant = Constant::INT(2);
    let index = Expr::new_constant_expr_ptr(constant);
    // access expr ptr
    let access_ptr_result = Access::new_ptr(&name, basic_type(array_type), &index);
    let access_ptr = access_ptr_result.unwrap();
    let constant = Constant::FLOAT(5.0);
    let expr_ptr = Expr::new_constant_expr_ptr(constant);
    let error = SetElemStmt::new_ptr(&access_ptr, &expr_ptr).unwrap_err();
    let expected = VALUES_ASSIGNMENT_EQUAL_TYPES;
    assert!(error.to_string().contains(expected));
}

#[test]
fn test_parser_stmt_set_elem_gen_case1() {
    // array type
    let array_type = Type::Array(BasicType::Int, 10);
    // array name
    let name = Pointer::new_pointer(String::from("a"));
    // index
    let constant = Constant::INT(2);
    let index = Expr::new_constant_expr_ptr(constant);
    // access expr ptr
    let access_ptr_result = Access::new_ptr(&name, basic_type(array_type), &index);
    let access_ptr = access_ptr_result.unwrap();
    let constant = Constant::INT(5);
    let expr_ptr = Expr::new_constant_expr_ptr(constant);
    let set_elem_stmt_ptr_result = SetElemStmt::new_ptr(&access_ptr, &expr_ptr);
    let set_elem_stmt_ptr = set_elem_stmt_ptr_result.unwrap();
    let env = Env::new_mut_ptr();
    let gen_result = set_elem_stmt_ptr.gen(&env).unwrap();
    assert_eq!(gen_result, ());
}
