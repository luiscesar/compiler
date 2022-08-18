use super::{Env, EnvMutPtr};
use common::pointer::Pointer;

#[test]
fn test_parser_stmt_env_new_case1() {
    let env_ptr = Env::new_mut_ptr();

    push_label(&env_ptr, 1);
    push_label(&env_ptr, 2);
    let label = env_ptr.borrow().top_loop_label().unwrap();
    assert_eq!(label, 2);

    pop_label(&env_ptr);
    let label = env_ptr.borrow().top_loop_label().unwrap();
    assert_eq!(label, 1);
}

#[test]
fn test_parser_stmt_env_new_case2() {
    let env_ptr = get_env_ptr();
    push_label(&env_ptr, 1);
    push_label(&env_ptr, 2);
    let label = env_ptr.borrow().top_loop_label().unwrap();
    assert_eq!(label, 2);

    pop_label(&env_ptr);
    let label = env_ptr.borrow().top_loop_label().unwrap();
    assert_eq!(label, 1);
}

fn get_env_ptr() -> EnvMutPtr {
    Env::new_mut_ptr()
}

fn push_label(env_ptr_input: &EnvMutPtr, label: usize) {
    let env_ptr = Pointer::clone(env_ptr_input);
    env_ptr.borrow_mut().push_loop_label(label);
}

fn pop_label(env_ptr_input: &EnvMutPtr) {
    let env_ptr = Pointer::clone(env_ptr_input);
    env_ptr.borrow_mut().pop_loop_label();
}
