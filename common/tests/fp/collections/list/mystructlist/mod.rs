use std::time::Instant;
use rand::Rng;

use common::fp::collections::list::mystructlist::MyStructList;
use std::vec::Vec;

const MAX:i32 = 10_000;


#[derive(Debug,PartialEq)]
struct IpAddrInfo(i32,String);

#[derive(Debug,PartialEq)]
struct IpAddr(IpAddrInfo);

#[test]
fn integration_test_fp_list_mystructlist_nil_case1() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let mylist2:MyStructList<String> = MyStructList::nil();
    assert_eq!(mylist,mylist2);
}

#[test]
fn integration_test_fp_list_mystructlist_nil_case2() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let mylist2:MyStructList<IpAddr> = MyStructList::nil();
    assert_eq!(mylist,mylist2);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_case1() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let head1 = String::from("Hello");
    let mylist = MyStructList::cons(head1, mylist);
    println!("mylist {:?}", mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_case2() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let head1 = String::from("Hello");
    let mylist = MyStructList::cons(head1, mylist);
    let head2 = String::from("World");
    let mylist = MyStructList::cons(head2, mylist);
    println!("mylist {:?}", mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_case3() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    println!("mylist {:?}", mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_case4() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    let head2 = IpAddr(IpAddrInfo(1,String::from("World")));
    let mylist = MyStructList::cons(head2, mylist);

    println!("mylist {:?}", mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_head_case1() {
    let mylist:MyStructList<String> = MyStructList::nil();
    
    let expected_head = None;
    assert_eq!(MyStructList::head(&mylist),expected_head);
}

#[test]
fn integration_test_fp_list_mystructlist_head_case2() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let head1 = String::from("Hello");
    let mylist = MyStructList::cons(head1, mylist);

    let head2 = String::from("Hello");
    let expected_head = Some(&head2);
    assert_eq!(MyStructList::head(&mylist),expected_head);
}

#[test]
fn integration_test_fp_list_mystructlist_head_case3() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);

    let head2 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let expected_head = Some(&head2);
    assert_eq!(MyStructList::head(&mylist),expected_head);
}

#[test]
fn integration_test_fp_list_mystructlist_tail_case1() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let head1 = String::from("Hello");
    let mylist = MyStructList::cons(head1, mylist);
    let mylist = MyStructList::tail(mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_tail_case2() {
    let mylist:MyStructList<String> = MyStructList::nil();
    let head1 = String::from("Hello");
    let mylist = MyStructList::cons(head1, mylist);
    let head2 = String::from("World");
    let mylist = MyStructList::cons(head2, mylist);
    let mylist = MyStructList::tail(mylist);

    let expected_mylist:MyStructList<String> = MyStructList::nil();
    let head = String::from("Hello");
    let expected_mylist = 
        MyStructList::cons(head, expected_mylist);

    assert_eq!(mylist,expected_mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_tail_case3() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    let mylist = MyStructList::tail(mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_tail_case4() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    let head2 = IpAddr(IpAddrInfo(1,String::from("World")));
    let mylist = MyStructList::cons(head2, mylist);
    let mylist = MyStructList::tail(mylist);

    let expected_mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let expected_mylist = 
        MyStructList::cons(head, expected_mylist); 
    
    assert_eq!(mylist,expected_mylist);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_tail_head_case1() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    let mylist = MyStructList::tail(mylist);

    assert_eq!(MyStructList::head(&mylist),None);
}

#[test]
fn integration_test_fp_list_mystructlist_cons_tail_head_case2() {
    let mylist:MyStructList<IpAddr> = MyStructList::nil();
    let head1 = IpAddr(IpAddrInfo(1,String::from("Hello")));
    let mylist = MyStructList::cons(head1, mylist);
    let head2 = IpAddr(IpAddrInfo(1,String::from("World")));
    let mylist = MyStructList::cons(head2, mylist);

    let mylist = MyStructList::tail(mylist);

    let expected_head = IpAddr(IpAddrInfo(1,String::from("Hello")));
    assert_eq!(MyStructList::head(&mylist),Some(&expected_head));
}

#[test]
fn performance_test_fp_list_mystructlist_cons_case1() {
    let mut mylist:MyStructList<String> = MyStructList::nil();

    let now = Instant::now();
    for index in 1..=MAX {
        let element = String::from("Hello") + &index.to_string();
        mylist = MyStructList::cons(element,mylist);
    }
    let elapased_time = now.elapsed();
    assert!(!MyStructList::is_empty(mylist));
    println!("performance fp list cons case1 (millis) {:?}", elapased_time.as_millis());
    println!("performance fp list cons case1 (nanos) {:?}", elapased_time.as_nanos());
}