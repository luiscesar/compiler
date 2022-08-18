use std::rc::Rc;

use crate::parser::{
    expr::id::Id,
    expr::types::{Typed, INT},
};

//#[test]
fn test_parser_expr_new_id_tmp_name_case1() {
    let new_id_tmp = Id::new_id_tmp_name();
    assert_eq!(new_id_tmp, "t0".to_string());

    let new_id_tmp = Id::new_id_tmp_name();
    assert_eq!(new_id_tmp, "t1".to_string());

    let new_id_tmp = Id::new_id_tmp_name();
    assert_eq!(new_id_tmp, "t2".to_string());
}

#[test]
fn test_parser_expr_id_tmp_new_case1() {
    let t = Rc::new(INT);
    let id = Id::new_id_tmp(INT);
    assert_eq!(id.element_type(), INT);
    assert_eq!(*id.name, "t0".to_string());

    let id = Id::new_id_tmp(INT);
    assert_eq!(id.element_type(), INT);
    assert_eq!(*id.name, "t1".to_string());
}
