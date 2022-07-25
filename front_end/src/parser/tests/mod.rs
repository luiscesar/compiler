use common::{pointer::Pointer, collections::string::StringPtr};

use crate::{lexer::{token::{Token, Value, BLANK_TOKEN}}, parser::{expr::{types::{INT, FLOAT, BOOL, Type, BasicType}, id::IdPtr}, stmt::env::scope::{Scope, new_scope, ScopeStackMutPtr, new_scope_stack_mut_ptr, push}}, error::messages::{UNDECLARED_IDENTIFIER, EXPECTED_TOKEN, WHILE_CONDITION_BOOLEAN_EXPECTED, RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING, BREAK_WITHOUT_LOOP, IDENTIFIER_NAME_EXPECTED, INTEGER_EXPECTED, ARRAY_ACCESS_INDEX_INTEGER, VALUES_ASSIGNMENT_EQUAL_TYPES, ARRAY_DIMENSION_GREATER_THAN_ZERO, ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR, LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR, EXPRESSION_EXPECTED, ARITH_OPERATOR_EXPECTED, ARITH_OPERATION_NUMBERS_EXPECTED, LOGICAL_OPERATION_BOOLEANS_EXPECTED, RELATIONAL_OPERATOR_EXPECTED, DO_CONDITION_BOOLEAN_EXPECTED, IF_CONDITION_BOOLEAN_EXPECTED}};
use super::{Parser, expr::id::Id, stmt::env::Env};

#[test]
fn test_parser_new_case1() {
    //9-5+2
    let file_name = "resources/input1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let parser = parser_result.unwrap();
    assert_eq!(*parser.look,Token::Constant(Value::Int(9)));
}

#[test]
fn test_parser_move_case1() {
    //9-5+2
    let file_name = "resources/input1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    assert_eq!(*parser.look,Token::Constant(Value::Int(9)));

    let result = parser.move_scan().unwrap();
    assert_eq!(result,());
    assert_eq!(*parser.look,Token::Char('-'));

    let result = parser.move_scan().unwrap();
    assert_eq!(result,());
    assert_eq!(*parser.look,Token::Constant(Value::Int(5)));

    let result = parser.move_scan().unwrap();
    assert_eq!(result,());
    assert_eq!(*parser.look,Token::Char('+'));

    let result = parser.move_scan().unwrap();
    assert_eq!(result,());
    assert_eq!(*parser.look,Token::Constant(Value::Int(2)));

    let result = parser.move_scan().unwrap();
    assert_eq!(result,());
    assert_eq!(*parser.look,BLANK_TOKEN);
}

#[test]
fn test_parser_decls_case1() {
    let file_name = "resources/input_decls1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let scope = parser.decls().unwrap();
    let expected_id_ptr = Id::new_ptr(Pointer::new_pointer("i".to_string()),INT);
    let id_ptr = scope.get(&Pointer::new_pointer("i".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);
}

#[test]
fn test_parser_decls_case2() {
    let file_name = "resources/input_decls2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let scope = parser.decls().unwrap();
    let expected_id_ptr = Id::new_ptr(Pointer::new_pointer("i".to_string()),FLOAT);
    let id_ptr = scope.get(&Pointer::new_pointer("i".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);
}

#[test]
fn test_parser_decls_case3() {
    let file_name = "resources/input_decls3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let scope = parser.decls().unwrap();

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("i".to_string()),FLOAT);
    let id_ptr = scope.get(&Pointer::new_pointer("i".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("a".to_string()),BOOL);
    let id_ptr = scope.get(&Pointer::new_pointer("a".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    let id_ptr = scope.get(&Pointer::new_pointer("x".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);
}

#[test]
fn test_parser_decls_case4() {
    let file_name = "resources/input_decls4.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let scope = parser.decls().unwrap();

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("i".to_string()),FLOAT);
    let id_ptr = scope.get(&Pointer::new_pointer("i".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("a".to_string()),Type::Array(BasicType::Bool, 10));
    let id_ptr = scope.get(&Pointer::new_pointer("a".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);

    let expected_id_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),Type::Array(BasicType::Int, 100));
    let id_ptr = scope.get(&Pointer::new_pointer("x".to_string())).unwrap();
    assert_eq!(*id_ptr,expected_id_ptr);
}

#[test]
fn test_parser_factor_case1() {
     let file_name = "resources/input_expr1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.factor().unwrap();
    assert_eq!(expr.to_string(),"9".to_string());
}

#[test]
fn test_parser_term_case1() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.term().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_expr_case2() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.expr().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_rel_case1() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.rel().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_equality_case1() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.rel().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_join_case1() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.join().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_bool_case1() {
    let file_name = "resources/input_expr2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    assert_eq!(expr.to_string(),"9 * 2".to_string());
}

#[test]
fn test_parser_expr_case3() {
    let file_name = "resources/input_expr3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.expr().unwrap();
    let expected_result = "9 * 2 + 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_rel_case3() {
    let file_name = "resources/input_expr3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.rel().unwrap();
    let expected_result = "9 * 2 + 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case3() {
    let file_name = "resources/input_expr3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = "9 * 2 + 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_rel_case4() {
    let file_name = "resources/input_expr4.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.rel().unwrap();
    let expected_result = "9 * 2 <= 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case4() {
    let file_name = "resources/input_expr4.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = "9 * 2 <= 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case5() {
    let file_name = "resources/input_expr5.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = "9 * 2 == 2 * 9".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_join_case6() {
    let file_name = "resources/input_expr6.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.join().unwrap();
    let expected_result = "9 * 2 == 2 * 9 && 9 * 2 > 1".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case7() {
    let file_name = "resources/input_expr7.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case8() {
    let file_name = "resources/input_expr8.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1 || true || 8 > 2".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case9() {
    let file_name = "resources/input_expr9.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1 || true || 8 > 2".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case10() {
    let file_name = "resources/input_expr10.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    
    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x".to_string()), id1_ptr);
    
    push(scope1, &parser.scope_stack_mut_ptr);

    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1 || true || 8 > x".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case11() {
    let file_name = "resources/input_expr10.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);
    let id2_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),BOOL);
    
    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x".to_string()), id1_ptr);
    
    let mut scope2:Scope<StringPtr,IdPtr> = new_scope(); 
    scope2.insert(Pointer::new_pointer("x".to_string()), id2_ptr);

    push(scope1, &parser.scope_stack_mut_ptr);
    push(scope2, &parser.scope_stack_mut_ptr);     
    let error = parser.bool().unwrap_err();
    let expected = RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING;
    assert!(error.to_string().contains(expected));
}

#[test]
fn test_parser_bool_case12() {
    let file_name = "resources/input_expr12.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer(
            "x".to_string()),
            Type::Array(BasicType::Int,10));
    
    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x".to_string()), id1_ptr);
    
    push(scope1, &parser.scope_stack_mut_ptr);

    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1 || true || 8 > x[2]".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_bool_case13() {
    let file_name = "resources/input_expr12.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer(
            "x".to_string()),
            Type::Array(BasicType::Int,10));
    
    let id2_ptr = 
        Id::new_ptr(Pointer::new_pointer("x".to_string()),INT);

    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x".to_string()), id1_ptr);

    let mut scope2:Scope<StringPtr,IdPtr> = new_scope(); 
    scope2.insert(Pointer::new_pointer("x".to_string()), id2_ptr);
    
    push(scope1, &parser.scope_stack_mut_ptr);
    push(scope2, &parser.scope_stack_mut_ptr);

    let error = parser.bool().unwrap_err();
    println!("{}",error.to_string());
    let expected = "Expected: )";
    assert!(error.to_string().contains(expected));
}

#[test]
fn test_parser_bool_case14() {
    let file_name = "resources/input_expr14.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer(
            "x".to_string()),
            Type::Array(BasicType::Int,10));
    
    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x".to_string()), id1_ptr);
    
    push(scope1, &parser.scope_stack_mut_ptr);

    let expr = parser.bool().unwrap();
    let expected_result = 
        "9 * 2 == 2 * 9 && 9 * 2 > 1 || 9 * 2 == 1 || true || 8 > x[2 + 1]".to_string();
    assert_eq!(expr.to_string(),expected_result);
}

#[test]
fn test_parser_block_case1() {
    let file_name = "resources/input_block1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case2() {
    let file_name = "resources/input_block2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case3() {
    let file_name = "resources/input_block3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case4() {
    let file_name = "resources/input_block4.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case5() {
    let file_name = "resources/input_block5.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case6() {
    let file_name = "resources/input_block6.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_case7() {
    let file_name = "resources/input_block7.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
}

#[test]
fn test_parser_block_gen_case7() {
    let file_name = "resources/input_block7.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let stmt = parser.block().unwrap();
    let env = Env::new_mut_ptr();
    let result = stmt.gen(&env).unwrap();
    assert_eq!(result,())
}

#[test]
fn test_parser_program_case1() {
    let file_name = "resources/input_program1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap();
    assert_eq!(result,());
}

#[test]
fn test_parser_program_case2() {
    let file_name = "resources/input_program2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap();
    assert_eq!(result,());
}

//pub const EXPECTED_TOKEN:&str = "Expected";
#[test]
pub fn test_parser_program_error_semicolon_case1() {
    let file_name = "resources/input_program_error1.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = EXPECTED_TOKEN;
    assert!(result.to_string().contains(expected));
}

//pub const UNDECLARED_IDENTIFIER:&str = "Undeclared identifier";
#[test]
pub fn test_parser_program_error_identifier_case2() {
    let file_name = "resources/input_program_error2.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = UNDECLARED_IDENTIFIER;
    assert!(result.to_string().contains(expected));
}

//pub const WHILE_CONDITION_BOOLEAN_EXPECTED:&str = "A while condition is expected to be boolean";
#[test]
pub fn test_parser_program_error_while_case3() {
    let file_name = "resources/input_program_error3.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = WHILE_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING:&str = "A relational operation is expecting values with the same ordering (numbers or booleans)";
#[test]
pub fn test_parser_program_error_relational_case4() {
    let file_name = "resources/input_program_error4.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING;
    assert!(result.to_string().contains(expected));
}

#[test]
pub fn test_parser_program_error_while_case5() {
    let file_name = "resources/input_program_error5.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = EXPECTED_TOKEN;
    assert!(result.to_string().contains(expected));
}

//pub const BREAK_WITHOUT_LOOP:&str = "A break statement is expected to be within a loop";
#[test]
pub fn test_parser_program_error_break_case6() {
    let file_name = "resources/input_program_error6.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = BREAK_WITHOUT_LOOP;
    assert!(result.to_string().contains(expected));
}

//pub const INTEGER_EXPECTED:&str = "Integer expected";
#[test]
pub fn test_parser_program_error_integer_array_dimension_case7() {
    let file_name = "resources/input_program_error7.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = ARRAY_DIMENSION_GREATER_THAN_ZERO;
    assert!(result.to_string().contains(expected));
}

//pub const ARRAY_ACCESS_INDEX_INTEGER:&str = "An array access index is expected to be integer";
#[test]
pub fn test_parser_program_error_integer_array_access_index_integer_case8() {
    let file_name = "resources/input_program_error8.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = ARRAY_ACCESS_INDEX_INTEGER;
    assert!(result.to_string().contains(expected));
}

//pub const VALUES_ASSIGNMENT_EQUAL_TYPES:&str = "Values in assignment are expected to have equal types";
#[test]
pub fn test_parser_program_error_values_assingment_types_case9() {
    let file_name = "resources/input_program_error9.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = VALUES_ASSIGNMENT_EQUAL_TYPES;
    assert!(result.to_string().contains(expected));
}

//pub const IDENTIFIER_NAME_EXPECTED:&str = "Identifier name expected";
#[test]
pub fn test_parser_program_error_integer_name_case10() {
    let file_name = "resources/input_program_error10.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = IDENTIFIER_NAME_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const ARRAY_DIMENSION_GREATER_THAN_ZERO:&str = "Array dimension must be an integer greater than zero";
#[test]
pub fn test_parser_program_error_array_dimension_greater_than_zero_case11() {
    let file_name = "resources/input_program_error11.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = ARRAY_DIMENSION_GREATER_THAN_ZERO;
    assert!(result.to_string().contains(expected));
}

//pub const ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR:&str = "An arith expression is required under an arith unary operator";
#[test]
pub fn test_parser_program_error_arith_expr_unary_operator_case12() {
    let file_name = "resources/input_program_error12.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR;
    assert!(result.to_string().contains(expected));
}

//pub const LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR:&str = "A logical expression is required under a logical unary operator";
#[test]
pub fn test_parser_program_error_logical_expr_unary_operator_case13() {
    let file_name = "resources/input_program_error13.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR;
    assert!(result.to_string().contains(expected));
}

//pub const EXPRESSION_EXPECTED:&str = "An expression is expected";
#[test]
pub fn test_parser_program_error_expression_expected_case14() {
    let file_name = "resources/input_program_error14.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = EXPRESSION_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const ARITH_OPERATION_NUMBERS_EXPECTED:&str = "An arith operation is expecting numbers as operands";
#[test]
pub fn test_parser_program_error_arith_operation_numbers_expected_case15() {
    let file_name = "resources/input_program_error15.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = ARITH_OPERATION_NUMBERS_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const LOGICAL_OPERATION_BOOLEANS_EXPECTED:&str = "A logical operation is expecting booleans as operands";
#[test]
pub fn test_parser_program_error_boolean_operation_booleans_expected_case16() {
    let file_name = "resources/input_program_error16.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = LOGICAL_OPERATION_BOOLEANS_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const DO_CONDITION_BOOLEAN_EXPECTED:&str = "A do condition is expected to be boolean";
#[test]
pub fn test_parser_program_error_do_condition_boolean_expected_case17() {
    let file_name = "resources/input_program_error17.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = DO_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}

//pub const IF_CONDITION_BOOLEAN_EXPECTED:&str = "An if condition is expected to be boolean";
#[test]
pub fn test_parser_program_error_if_condition_boolean_expected_case18() {
    let file_name = "resources/input_program_error18.txt".to_string();
    let parser_result = 
        Parser::new(file_name);
    let mut parser = parser_result.unwrap();
    let result = parser.program().unwrap_err();
    println!("{}",result.to_string());
    let expected = IF_CONDITION_BOOLEAN_EXPECTED;
    assert!(result.to_string().contains(expected));
}