use front_end::{
    error::messages::{
        ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR, ARITH_OPERATION_NUMBERS_EXPECTED,
        ARRAY_ACCESS_INDEX_INTEGER, ARRAY_DIMENSION_GREATER_THAN_ZERO, BREAK_WITHOUT_LOOP,
        DO_CONDITION_BOOLEAN_EXPECTED, EXPECTED_TOKEN, EXPRESSION_EXPECTED,
        IDENTIFIER_NAME_EXPECTED, IF_CONDITION_BOOLEAN_EXPECTED, INTEGER_EXPECTED,
        LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR, LOGICAL_OPERATION_BOOLEANS_EXPECTED,
        RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING, UNDECLARED_IDENTIFIER,
        VALUES_ASSIGNMENT_EQUAL_TYPES, WHILE_CONDITION_BOOLEAN_EXPECTED,
    },
    parser::Parser,
};

#[test]
pub fn integration_test_parser_program_case1() {
    let file_name = "resources/input_program1.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap();
    assert_eq!(result, ());
}

#[test]
pub fn integration_test_parser_program_case2() {
    let file_name = "resources/input_program2.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap();
    assert_eq!(result, ());
}

//pub const EXPECTED_TOKEN:&str = "Expected";
#[test]
pub fn integration_test_parser_program_error_semicolon_case1() {
    let file_name = "resources/input_program_error1.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = EXPECTED_TOKEN;
    assert!(result.to_string().contains(expected));
}

//pub const UNDECLARED_IDENTIFIER:&str = "Undeclared identifier";
#[test]
pub fn integration_test_parser_program_error_identifier_case2() {
    let file_name = "resources/input_program_error2.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = UNDECLARED_IDENTIFIER;
    assert!(result.to_string().contains(expected));
}

//pub const WHILE_CONDITION_BOOLEAN_EXPECTED:&str = "A while condition is expected to be boolean";
#[test]
pub fn integration_test_parser_program_error_while_case3() {
    let file_name = "resources/input_program_error3.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = WHILE_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING:&str = "A relational operation is expecting values with the same ordering (numbers or booleans)";
#[test]
pub fn integration_test_parser_program_error_relational_case4() {
    let file_name = "resources/input_program_error4.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING;
    assert!(result.to_string().contains(expected));
}

#[test]
pub fn integration_test_parser_program_error_while_case5() {
    let file_name = "resources/input_program_error5.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = EXPECTED_TOKEN;
    assert!(result.to_string().contains(expected));
}

//pub const BREAK_WITHOUT_LOOP:&str = "A break statement is expected to be within a loop";
#[test]
pub fn integration_test_parser_program_error_break_case6() {
    let file_name = "resources/input_program_error6.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = BREAK_WITHOUT_LOOP;
    assert!(result.to_string().contains(expected));
}

//pub const INTEGER_EXPECTED:&str = "Integer expected";
#[test]
pub fn integration_test_parser_program_error_integer_array_dimension_case7() {
    let file_name = "resources/input_program_error7.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = ARRAY_DIMENSION_GREATER_THAN_ZERO;
    assert!(result.to_string().contains(expected));
}

//pub const ARRAY_ACCESS_INDEX_INTEGER:&str = "An array access index is expected to be integer";
#[test]
pub fn integration_test_parser_program_error_integer_array_access_index_integer_case8() {
    let file_name = "resources/input_program_error8.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = ARRAY_ACCESS_INDEX_INTEGER;
    assert!(result.to_string().contains(expected));
}

//pub const VALUES_ASSIGNMENT_EQUAL_TYPES:&str = "Values in assignment are expected to have equal types";
#[test]
pub fn integration_test_parser_program_error_values_assingment_types_case9() {
    let file_name = "resources/input_program_error9.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = VALUES_ASSIGNMENT_EQUAL_TYPES;
    assert!(result.to_string().contains(expected));
}

//pub const IDENTIFIER_NAME_EXPECTED:&str = "Identifier name expected";
#[test]
pub fn integration_test_parser_program_error_integer_name_case10() {
    let file_name = "resources/input_program_error10.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = IDENTIFIER_NAME_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const ARRAY_DIMENSION_GREATER_THAN_ZERO:&str = "Array dimension must be an integer greater than zero";
#[test]
pub fn integration_test_parser_program_error_array_dimension_greater_than_zero_case11() {
    let file_name = "resources/input_program_error11.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = ARRAY_DIMENSION_GREATER_THAN_ZERO;
    assert!(result.to_string().contains(expected));
}

//pub const ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR:&str = "An arith expression is required under an arith unary operator";
#[test]
pub fn integration_test_parser_program_error_arith_expr_unary_operator_case12() {
    let file_name = "resources/input_program_error12.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR;
    assert!(result.to_string().contains(expected));
}

//pub const LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR:&str = "A logical expression is required under a logical unary operator";
#[test]
pub fn integration_test_parser_program_error_logical_expr_unary_operator_case13() {
    let file_name = "resources/input_program_error13.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR;
    assert!(result.to_string().contains(expected));
}

//pub const EXPRESSION_EXPECTED:&str = "An expression is expected";
#[test]
pub fn integration_test_parser_program_error_expression_expected_case14() {
    let file_name = "resources/input_program_error14.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = EXPRESSION_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const ARITH_OPERATION_NUMBERS_EXPECTED:&str = "An arith operation is expecting numbers as operands";
#[test]
pub fn integration_test_parser_program_error_arith_operation_numbers_expected_case15() {
    let file_name = "resources/input_program_error15.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = ARITH_OPERATION_NUMBERS_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const LOGICAL_OPERATION_BOOLEANS_EXPECTED:&str = "A logical operation is expecting booleans as operands";
#[test]
pub fn integration_test_parser_program_error_boolean_operation_booleans_expected_case16() {
    let file_name = "resources/input_program_error16.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = LOGICAL_OPERATION_BOOLEANS_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const DO_CONDITION_BOOLEAN_EXPECTED:&str = "A do condition is expected to be boolean";
#[test]
pub fn integration_test_parser_program_error_do_condition_boolean_expected_case17() {
    let file_name = "resources/input_program_error17.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = DO_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const IF_CONDITION_BOOLEAN_EXPECTED:&str = "An if condition is expected to be boolean";
#[test]
pub fn integration_test_parser_program_error_if_condition_boolean_expected_case18() {
    let file_name = "resources/input_program_error18.txt".to_string();
    let parser_result = Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}", result.to_string());
    let expected = IF_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}
