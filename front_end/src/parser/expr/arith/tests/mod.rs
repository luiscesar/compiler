
use common::pointer::Pointer;

use crate::parser::{expr::{types::{INT, Typed, BOOL}, constant::{TRUE, Constant}, id::Id}};


#[test]
fn test_parser_expr_arith_new_case1() {
    let id_name = "x".to_string();
    let id = Id::new(Pointer::new_pointer(id_name),INT);
    assert_eq!(id.element_type(),INT);
}

#[test]
fn test_parser_expr_arith_new_case2() {
    let constant = Constant::INT(2);
    assert_eq!(constant.element_type(),INT);
}

#[test]
fn test_parser_expr_arith_new_case3() {
    let constant = TRUE;
    assert_eq!(constant.element_type(),BOOL);
}