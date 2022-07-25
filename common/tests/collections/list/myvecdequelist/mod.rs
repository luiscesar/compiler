use std::time::Instant;
use rand::Rng;

use common::collections::list::mylist::MyList;
use common::collections::list::myvecdequelist::MyVecDequeList;

const MAX:i32 = 100_000;

#[test]
pub fn integration_test_list_myvecdequelist_case1() {
    let my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    assert!(my_list.is_empty());
}

#[test]
pub fn integration_test_list_myvecdequelist_cons_case1() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    let element = String::from("Hello");
    my_list.cons(element);
    assert!(!my_list.is_empty());
}

#[test]
pub fn integration_test_list_myvecdequelist_cons_case2() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    
    let now = Instant::now();
    for index in 1..(MAX+1) {
        let element = String::from("Hello") + &index.to_string();
        my_list.cons(element);
    }
    let elapased_time = now.elapsed();
    assert!(!my_list.is_empty());

    println!("MAX {}", MAX);
    println!("integration test myvecdequelist case1 (nanos): {}", 
        elapased_time.as_nanos());
    println!("integration test myvecdequelist case1 (millis): {}", 
        elapased_time.as_millis());
}

#[test]
pub fn integration_test_list_myvecdequelist_head_case1() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    let head = my_list.head();
    let expected_head = None;
    assert_eq!(head,expected_head)
}

#[test]
pub fn integration_test_list_myvecdequelist_head_case2() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    let element = String::from("Hello");
    my_list.cons(element);
    let head = my_list.head();
    let element = String::from("Hello");
    let expected_head = Some(&element);
    assert_eq!(head,expected_head)
}

#[test]
pub fn integration_test_list_myvecdequelist_head_case3() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    
    let element = String::from("Hello");
    my_list.cons(element);
    let element = String::from("World");
    my_list.cons(element);
    let element = String::from("Hello2");
    my_list.cons(element);

    let head = my_list.head();
    let element = String::from("Hello2");
    let expected_head = Some(&element);
    assert_eq!(head,expected_head)
}

#[test]
pub fn integration_test_list_myvecdequelist_head_case4() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
   
    for index in 1..(MAX+1) {
        let element = String::from("Hello") + &index.to_string();
        my_list.cons(element);
    }

    let now = Instant::now();
    let head = my_list.head();
    let elapased_time = now.elapsed();

    let value = String::from("Hello") + &MAX.to_string();
    let expected_head = Some(&value);
    assert_eq!(head,expected_head);

    println!("integration test myvecdequelist case1 (millis): {}", 
        elapased_time.as_millis());
    println!("integration test myvecdequelist case1 (nanos): {}", 
        elapased_time.as_nanos());
}

#[test]
pub fn integration_test_list_myvecdequelist_tail_case1() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    my_list.tail();
    let head = my_list.head();
    let expected_head = None;
    assert_eq!(head,expected_head);
}

#[test]
pub fn integration_test_list_myvecdequelist_tail_case2() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    let element = String::from("Hello");
    my_list.cons(element);
    my_list.tail();
    let head = my_list.head();
    let expected_head = None;
    assert_eq!(head,expected_head);
}

#[test]
pub fn integration_test_list_myvecdequelist_tail_case3() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    
    let element = String::from("Hello");
    my_list.cons(element);
    let element = String::from("World");
    my_list.cons(element);
    let element = String::from("Hello2");
    my_list.cons(element);

    my_list.tail();

    let head = my_list.head();
    let value = String::from("World");
    let expected_head = Some(&value);
    assert_eq!(head,expected_head);
}

#[test]
pub fn integration_test_list_myvecdequelist_tail_case4() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
   
    for index in 1..(MAX+1) {
        let element = String::from("Hello") + &index.to_string();
        my_list.cons(element);
    }

    let now = Instant::now();
    my_list.tail();
    let elapased_time = now.elapsed();

    let head = my_list.head();
    let value = String::from("Hello") + &(MAX-1).to_string();
    let expected_head = Some(&value);
    assert_eq!(head,expected_head);

    println!("integration test myvecdequelist case1 (millis): {}", 
        elapased_time.as_millis());
    println!("integration test myvecdequelist case1 (nanos): {}", 
        elapased_time.as_nanos());
}

#[test]
pub fn integration_test_list_myvecdequelist_is_empty_case1() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    assert!(my_list.is_empty());
}

#[test]
pub fn integration_test_list_myvecdequelist_is_empty_case2() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    let element = String::from("Hello");
    my_list.cons(element);
    assert!(!my_list.is_empty());
}

#[test]
pub fn integration_test_list_myvecdequelist_is_empty_case3() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
    
    let element = String::from("Hello");
    my_list.cons(element);
    let element = String::from("World");
    my_list.cons(element);
    let element = String::from("Hello2");
    my_list.cons(element);

    assert!(!my_list.is_empty());
}

#[test]
pub fn integration_test_list_myvecdequelist_is_empty_case4() {
    let mut my_list: MyVecDequeList<String> = MyVecDequeList::nil();
   
    for index in 1..(MAX+1) {
        let element = String::from("Hello") + &index.to_string();
        my_list.cons(element);
    }

    let now = Instant::now();
    let is_empty = my_list.is_empty();
    let elapased_time = now.elapsed();

    assert!(!is_empty);
    println!("integration test myvecdequelist case1 (millis): {}", 
        elapased_time.as_millis());
    println!("integration test myvecdequelist case1 (nanos): {}", 
        elapased_time.as_nanos());
}