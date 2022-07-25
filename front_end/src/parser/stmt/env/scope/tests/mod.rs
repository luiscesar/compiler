
use common::{pointer::Pointer, collections::string::StringPtr};

use crate::parser::{expr::{types::{INT,FLOAT,Typed,Type, BasicType},id::{Id,IdPtr}}, 
    stmt::env::scope::{push,pop,find, ScopeStackMutPtr, new_scope_stack_mut_ptr, new_scope, Scope}};

#[test]
fn test_parser_stmt_env_scope_find_case1() {
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer("x1".to_string()),INT);
    let id2_ptr = 
        Id::new_ptr(Pointer::new_pointer("x1".to_string()),FLOAT);
    
    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x1".to_string()), id1_ptr);
    
    let mut scope2:Scope<StringPtr,IdPtr> = new_scope(); 
    scope2.insert(Pointer::new_pointer("x1".to_string()), id2_ptr);

    let scope_stack_mut_ptr:ScopeStackMutPtr<StringPtr,IdPtr> = new_scope_stack_mut_ptr();
    
    push(scope1,&scope_stack_mut_ptr);
    push(scope2,&scope_stack_mut_ptr);

    let key = "x1".to_string();

    let value = 
        find(&Pointer::new_pointer(key), &scope_stack_mut_ptr);
    assert_eq!(value.unwrap().element_type(),FLOAT);

    pop(&scope_stack_mut_ptr);

    let key = "x1".to_string();
    let value = 
        find(&Pointer::new_pointer(key), &scope_stack_mut_ptr);
    assert_eq!(value.unwrap().element_type(),INT);
}

#[test]
fn test_parser_stmt_env_scope_find_case2() {
    let id1_ptr = 
        Id::new_ptr(Pointer::new_pointer("x1".to_string()),INT);
    let id2_ptr = 
        Id::new_ptr(Pointer::new_pointer("x1".to_string()),FLOAT);
    
    let id3_ptr = 
        Id::new_ptr(Pointer::new_pointer("x1".to_string()),Type::Array(BasicType::Int, 10));

    let mut scope1:Scope<StringPtr,IdPtr> = new_scope();
    scope1.insert(Pointer::new_pointer("x1".to_string()), id1_ptr);
    
    let mut scope2:Scope<StringPtr,IdPtr> = new_scope();
    scope2.insert(Pointer::new_pointer("x1".to_string()), id2_ptr);

    let mut scope3:Scope<StringPtr,IdPtr> = new_scope();
    scope3.insert(Pointer::new_pointer("x1".to_string()), id3_ptr);

    let scope_stack_mut_ptr:ScopeStackMutPtr<StringPtr,IdPtr> = 
        new_scope_stack_mut_ptr();

    push(scope1,&scope_stack_mut_ptr);
    push(scope2,&scope_stack_mut_ptr);
    push(scope3,&scope_stack_mut_ptr);

    let key = "x1".to_string();

    let value = 
        find(&Pointer::new_pointer(key), &scope_stack_mut_ptr);
    let x = value.unwrap().element_type();
    match x {
        Type::Array(..) => {assert!(true)},
        _ => assert!(false),
    }

    pop(&scope_stack_mut_ptr);

    let key = "x1".to_string();
    let value = 
        find(&Pointer::new_pointer(key), &scope_stack_mut_ptr);
    assert_eq!(value.unwrap().element_type(),FLOAT);

}
