use common::fp::collections::list::myenumlist::MyEnumList;
use common::fp::collections::list::myenumlist::MyList;
use rand::Rng;
use std::rc::Rc;
use std::time::Instant;

const MAX: usize = 10_000;

#[derive(Debug, PartialEq)]
struct IpAddrInfo(i32, String);
#[derive(Debug, PartialEq)]
struct IpAddr(IpAddrInfo);

#[test]
fn integration_test_fp_list_myenumlist_cons_case1() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    println!("my_list {:?}", my_list);
}

#[test]
fn integration_test_fp_list_myenumlist_cons_case2() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    let head = String::from("Hello");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    println!("my_list {:?}", my_list);
}

#[test]
fn integration_test_fp_list_myenumlist_head_case1() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    assert_eq!(MyEnumList::head(my_list), None);
}

#[test]
fn integration_test_fp_list_myenumlist_head_case2() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    let head = String::from("Hello");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    let expected_head = String::from("Hello");
    assert_eq!(MyEnumList::head(my_list), Some(Rc::new(expected_head)));
}

#[test]
fn integration_test_fp_list_myenumlist_head_case3() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    let head = String::from("Hello");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    let head = String::from("World");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    let expected_head = String::from("World");
    assert_eq!(MyEnumList::head(my_list), Some(Rc::new(expected_head)));
}

#[test]
fn integration_test_fp_list_myenumlist_tail_case1() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    assert_eq!(MyEnumList::tail(my_list), None);
}

#[test]
fn integration_test_fp_list_myenumlist_tail_case2() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    let head = String::from("Hello");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    let expected_tail: Option<Box<MyEnumList<String>>> = Some(Box::new(MyEnumList::Nil()));
    assert_eq!(MyEnumList::tail(my_list), expected_tail);
}

#[test]
fn integration_test_fp_list_myenumlist_tail_case3() {
    let my_list: MyEnumList<String> = MyEnumList::Nil();
    let head = String::from("Hello");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));
    let head = String::from("World");
    let my_list = MyEnumList::Cons(Rc::new(head), Box::new(my_list));

    let expected_tail = Box::new(MyEnumList::Cons(
        Rc::new(String::from("Hello")),
        Box::new(MyEnumList::Nil()),
    ));
    assert_eq!(MyEnumList::tail(my_list), Some(expected_tail));
}

#[test]
fn performance_test_fp_list_enumlist_cons_case1() {
    let mut my_list: MyEnumList<String> = MyEnumList::Nil();
    let now = Instant::now();
    for index in 1..(MAX + 1) {
        let element = String::from("Hello") + &index.to_string();
        my_list = MyEnumList::Cons(Rc::new(element), Box::new(my_list));
    }
    let elapased_time = now.elapsed();
    assert_eq!(
        MyEnumList::head(my_list),
        Some(Rc::new(String::from("Hello") + &MAX.to_string()))
    );
    println!(
        "integration test performance enumlist cons case1 (millis): {}",
        elapased_time.as_millis()
    );
    println!(
        "integration test performance enumlist cons case1 (nanos): {}",
        elapased_time.as_nanos()
    );
}
