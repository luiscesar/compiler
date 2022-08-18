use super::{BasicType, Type, BOOL, FLOAT, INT};

#[test]
fn test_parser_types_new_int_case1() {
    let t = INT;
    assert_eq!(t, INT);
}

#[test]
fn test_parser_types_new_float_case1() {
    let t = FLOAT;
    assert_eq!(t, FLOAT)
}

#[test]
fn test_parser_types_new_bool_case1() {
    let t = BOOL;
    assert_eq!(t, BOOL)
}

#[test]
fn test_parser_types_new_array_int_case1() {
    let t = Type::Array(BasicType::Int, 10);
    assert_eq!(t, Type::Array(BasicType::Int, 10));
}
