use std::time::Instant;
use rand::Rng;

use common::collections::set::myset::MySet;
use common::collections::set::myhashset::MyHashSet;

const MAX:i32 = 10_000;

#[test]
pub fn integration_test_set_myhashset_new_case1() {
    let my_hash_set: MyHashSet<i32> = MyHashSet::new();
    println!("size {}", my_hash_set.size());
    assert_eq!(my_hash_set.size(), 0);
}

#[test]
pub fn integration_test_set_myhashset_new_case2() {
    let my_hash_set: MyHashSet<String> = MyHashSet::new();
    println!("size {}", my_hash_set.size());
    assert_eq!(my_hash_set.size(), 0);
}

#[test]
pub fn integration_test_set_myhashset_add_case1() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();

    let now = Instant::now();

    for element in 1..MAX {
        my_hash_set.add(element);
    }
    let elapsed_time = now.elapsed();

    let max_usize:usize = MAX as usize;
    assert_eq!(my_hash_set.size(),max_usize-1);
    println!("myhashset add case1 (milliseconds): {}", elapsed_time.as_millis());
    println!("myhashset add case1 (nanoseconds): {}", elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_add_case2() {
    let mut my_hash_set: MyHashSet<String> = MyHashSet::new();
    
    let now = Instant::now();
    for index in 1..MAX {
        let element = String::from("Hello") + &index.to_string();
        my_hash_set.add(element);
    }
    let elapsed_time = now.elapsed();
    let max_usize:usize = MAX as usize;
    assert_eq!(my_hash_set.size(),max_usize-1);
    println!("myhashset add case2 (milliseconds): {}", elapsed_time.as_millis());
    println!("myhashset add case2 (nanoseconds): {}", elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_remove_case1() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::nil();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let value = rand::thread_rng().gen_range(1..MAX);

    let now = Instant::now();
    my_hash_set.remove(&value);
    let elapsed_time = now.elapsed();

    let expected_size = MAX as usize - 2;
    assert_eq!(my_hash_set.size(),expected_size);
    println!("myhashset remove case1 (milliseconds): {}", elapsed_time.as_millis());
    println!("myhashset remove case1 (nanoseconds): {}", elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_remove_case2() {
    let mut my_hash_set: MyHashSet<String> = MyHashSet::nil();
    
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
    assert_eq!(my_hash_set.size(),expected_size);
    println!("myhashset remove case2 (milliseconds): {}", elapsed_time.as_millis());
    println!("myhashset remove case2 (nanoseconds): {}", elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_contains_case1() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    let value = rand::thread_rng().gen_range(1..MAX);
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let contains = my_hash_set.contains(&value);
    let elapsed_time = now.elapsed();
    assert!(contains);
    println!("myhashset contains case1 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset contains case1 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_contains_case2() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    let value = MAX;
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    
    let now = Instant::now();
    let contains = my_hash_set.contains(&value);
    let elapsed_time = now.elapsed();

    assert!(!contains);
    println!("myhashset contains case2 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset contains case2 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_size_case1() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();

    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    assert_eq!(size,0);
    println!("myhashset size case1 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset size case1 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_size_case2() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    let expected_size:usize = MAX as usize - 1;
    assert_eq!(size,expected_size);
    println!("myhashset size case2 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset size case2 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_size_case3() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    for index in 1..MAX {
        my_hash_set.remove(&index);
    }
    let now = Instant::now();
    let size = my_hash_set.size();
    let elapsed_time = now.elapsed();
    let expected_size:usize = 0;
    assert_eq!(size,expected_size);
    println!("myhashset size case3 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset size case3 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_is_empty_case1() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    my_hash_set.add(1);
    my_hash_set.remove(&1);

    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();

    assert!(is_empty);
    println!("myhashset is_empty case1 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset is_empty case1 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_is_empty_case2() {
    let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
    for index in 1..MAX {
        my_hash_set.add(index);
    }
    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();
    assert!(!is_empty);
    println!("myhashset is_empty case2 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset is_empty case2 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}

#[test]
pub fn integration_test_set_myhashset_is_empty_case3() {
    let my_hash_set: MyHashSet<i32> = MyHashSet::new();
    let now = Instant::now();
    let is_empty = my_hash_set.is_empty();
    let elapsed_time = now.elapsed();
    assert!(is_empty);
    println!("myhashset is_empty case3 (milliseconds): {}", 
        elapsed_time.as_millis());
    println!("myhashset is_empty case3 (nanoseconds): {}", 
        elapsed_time.as_nanos());
}