use rand::Rng;
use std::time::Instant;

use common::collections::set::mybtreeset::MyBTreeSet;
use common::collections::set::myset::MySet;

const MAX: i32 = 10_000;

#[test]
pub fn integration_test_set_mybtreeset_nil_case1() {
    let my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    println!("size {}", my_hash_set.size());
    assert_eq!(my_hash_set.size(), 0);
}

#[test]
pub fn integration_test_set_mybtreeset_nil_case2() {
    let my_hash_set: MyBTreeSet<String> = MyBTreeSet::nil();
    println!("size {}", my_hash_set.size());
    assert_eq!(my_hash_set.size(), 0);
}

#[test]
pub fn integration_test_set_mybtreeset_add_case1() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();

    let now = Instant::now();

    for element in 1..MAX {
        my_hash_set.add(element);
    }
    let elapsed_time = now.elapsed();

    let max_usize: usize = MAX as usize;
    assert_eq!(my_hash_set.size(), max_usize - 1);
    println!(
        "mybtreeset add case1 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset add case1 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_add_case2() {
    let mut my_hash_set: MyBTreeSet<String> = MyBTreeSet::nil();

    let now = Instant::now();
    for index in 1..MAX {
        let element = String::from("Hello") + &index.to_string();
        my_hash_set.add(element);
    }
    let elapsed_time = now.elapsed();
    let max_usize: usize = MAX as usize;
    assert_eq!(my_hash_set.size(), max_usize - 1);
    println!(
        "mybtreeset add case2 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset add case2 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_remove_case1() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let value = rand::thread_rng().gen_range(1..MAX);

    let now = Instant::now();
    my_hash_set.remove(&value);
    let elapsed_time = now.elapsed();

    let expected_size = MAX as usize - 2;
    assert_eq!(my_hash_set.size(), expected_size);
    println!(
        "mybtreeset remove case1 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset remove case1 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_remove_case2() {
    let mut my_hash_set: MyBTreeSet<String> = MyBTreeSet::nil();

    for index in 1..MAX {
        let element = String::from("Hello") + &index.to_string();
        my_hash_set.add(element);
    }
    let value = rand::thread_rng().gen_range(1..MAX);
    let element = String::from("Hello") + &value.to_string();

    let now = Instant::now();
    my_hash_set.remove(&element);
    let elapsed_time = now.elapsed();

    let expected_size = MAX as usize - 2;
    assert_eq!(my_hash_set.size(), expected_size);
    println!(
        "mybtreeset remove case2 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset remove case2 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_contains_case1() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    let value = rand::thread_rng().gen_range(1..MAX);
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let contains = my_hash_set.contains(&value);
    let elapsed_time = now.elapsed();
    assert!(contains);
    println!(
        "mybtreeset contains case1 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset contains case1 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_contains_case2() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    let value = MAX;
    for index in 1..MAX {
        my_hash_set.add(index);
    }

    let now = Instant::now();
    let contains = my_hash_set.contains(&value);
    let elapsed_time = now.elapsed();

    assert!(!contains);
    println!(
        "mybtreeset contains case2 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset contains case2 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_size_case1() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();

    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    assert_eq!(size, 0);
    println!(
        "mybtreeset size case1 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset size case1 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_size_case2() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    let expected_size: usize = MAX as usize - 1;
    assert_eq!(size, expected_size);
    println!(
        "mybtreeset size case2 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset size case2 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_size_case3() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    for index in 1..MAX {
        my_hash_set.remove(&index);
    }
    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    let expected_size: usize = 0;
    assert_eq!(size, expected_size);
    println!(
        "mybtreeset size case3 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset size case3 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_is_empty_case1() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    my_hash_set.add(1);
    my_hash_set.remove(&1);

    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();

    assert!(is_empty);
    println!(
        "mybtreeset is_empty case1 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset is_empty case1 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_is_empty_case2() {
    let mut my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();
    assert!(!is_empty);
    println!(
        "mybtreeset is_empty case2 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset is_empty case2 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}

#[test]
pub fn integration_test_set_mybtreeset_is_empty_case3() {
    let my_hash_set: MyBTreeSet<i32> = MyBTreeSet::nil();
    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();
    assert!(is_empty);
    println!(
        "mybtreeset is_empty case3 (milliseconds): {}",
        elapsed_time.as_millis()
    );
    println!(
        "mybtreeset is_empty case3 (nanoseconds): {}",
        elapsed_time.as_nanos()
    );
}
