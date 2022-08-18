use rand::Rng;
use std::time::Instant;

use common::collections::queue::mylinkedlistqueue::MyLinkedListQueue;
use common::collections::queue::myqueue::MyQueue;

const MAX: i32 = 100_000;

#[test]
pub fn integration_test_queue_mylinkedlistqueue_new_case1() {
    let myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
    let expected_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
    assert_eq!(myqueue, expected_queue);
}

#[test]
pub fn integration_test_queue_mylinkedlistqueue_enqueue_case1() {
    let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
    let element = String::from("Hello");
    myqueue.enqueue(element);

    let first = myqueue.first();
    let element = String::from("Hello");
    let expected_first = Some(&element);
    assert_eq!(first, expected_first);
}

#[test]
pub fn integration_test_queue_mylinkedlistqueue_enqueue_case2() {
    let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();

    let now = Instant::now();
    for index in 1..(MAX + 1) {
        let element = String::from("Hello") + &index.to_string();
        my_queue.enqueue(element);
    }
    let elapased_time = now.elapsed();

    let first = my_queue.first();
    let element = String::from("Hello") + &1.to_string();
    let expected_first = Some(&element);
    assert_eq!(first, expected_first);

    let max = MAX as usize;
    assert_eq!(my_queue.size(), max);

    println!(
        "integration test mylinkedlist case1 (nanos): {}",
        elapased_time.as_nanos()
    );
    println!(
        "integration test mylinkedlist case1 (millis): {}",
        elapased_time.as_millis()
    );
}

#[test]
pub fn integration_test_queue_mylinkedlistqueue_dequeue_case1() {
    let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();

    for index in 1..(MAX + 1) {
        let element = String::from("Hello") + &index.to_string();
        my_queue.enqueue(element);
    }
    let now = Instant::now();
    for index in 1..(MAX + 1) {
        my_queue.dequeue();
    }
    let elapased_time = now.elapsed();

    assert!(my_queue.is_empty());

    println!(
        "integration test mylinkedlist case1 (nanos): {}",
        elapased_time.as_nanos()
    );
    println!(
        "integration test mylinkedlist case1 (millis): {}",
        elapased_time.as_millis()
    );
}
