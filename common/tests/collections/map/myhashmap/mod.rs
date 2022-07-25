use common::collections::map::myhashmap::MyHashMap;
use std::{collections::HashMap, time::Instant};
use common::collections::map::mymap::MyMap;
use rand::Rng;

const MAX:i32 = 100_000;

#[test]
fn integration_test_map_myhashmap_nil_case1() {
    let my_map:MyHashMap<String,String> = MyHashMap::nil();
    let expected_map:MyHashMap<String,String> = MyHashMap::nil();
    assert_eq!(my_map,expected_map);
}

#[test]
fn integration_test_map_myhashmap_put_case1() {
    let mut my_map:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    my_map.put(key, value);

    let mut expected_map:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    expected_map.put(key, value);
    assert_eq!(my_map,expected_map);
}

#[test]
fn integration_test_map_myhashmap_put_case2() {
    let mut my_map:MyHashMap<String,String> = MyHashMap::nil();

    let now = Instant::now();
    for index in 1..(MAX+1) {
        let key = String::from("Hello") + &index.to_string();
        let value = String::from("World");
        my_map.put(key, value);
    }
    let elapsed_time = now.elapsed();

    let max = MAX as usize;
    assert_eq!(my_map.size(),max);

    println!("integration_test_map_myhashmap_put_case2 (milliseconds): {}", elapsed_time.as_millis());
    println!("integration_test_map_myhashmap_put_case2 (nanos): {}", elapsed_time.as_nanos());
}

#[test]
fn integration_test_map_myhashmap_get_case1() {
    let mymap:MyHashMap<String,String> = MyHashMap::nil();

    let key = String::from("Hello");
    let value = None;
    assert_eq!(value,mymap.get(&key));
}

#[test]
fn integration_test_map_myhashmap_get_case2() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    let value = String::from("World");
    let key = String::from("Hello");
    let value = Some(&value);
    assert_eq!(value,mymap.get(&key));
}

#[test]
fn integration_test_map_myhashmap_get_case3() {
    let mut my_map:MyHashMap<String,String> = MyHashMap::nil();

    
    for index in 1..(MAX+1) {
        let key = String::from("Hello") + &index.to_string();
        let value = String::from("World");
        my_map.put(key, value);
    }
    let index = rand::thread_rng().gen_range(1..MAX);
    let key = String::from("Hello") + &index.to_string();

    let now = Instant::now();
    let value = my_map.get(&key);
    let elapsed_time = now.elapsed();

    let raw_value = String::from("World");
    let expected_value = Some(&raw_value);
    assert_eq!(value,expected_value);

    println!("integration_test_map_myhashmap_get_case3 (milliseconds): {}",
        elapsed_time.as_millis());
    
    println!("integration_test_map_myhashmap_get_case3 (nanoseconds): {}",
        elapsed_time.as_nanos());
    
}

#[test]
fn integration_test_map_myhashmap_remove_case1() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
  
    let key = String::from("Hello");
    mymap.remove(&key);

    let value = None;
    assert_eq!(value,mymap.get(&key));
}

#[test]
fn integration_test_map_myhashmap_remove_case2() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    let key = String::from("Hello");
    mymap.remove(&key);

    let value = None;
    assert_eq!(value,mymap.get(&key));
}

#[test]
fn integration_test_map_myhashmap_remove_case3() {
    let mut my_map:MyHashMap<String,String> = MyHashMap::nil();
    
    for index in 1..(MAX+1) {
        let key = String::from("Hello") + &index.to_string();
        let value = String::from("World");
        my_map.put(key, value);
    }
    let index = rand::thread_rng().gen_range(1..MAX);
    let key = String::from("Hello") + &index.to_string();

    let now = Instant::now();
    my_map.remove(&key);
    let elapsed_time = now.elapsed();

    assert_eq!(my_map.get(&key),None);
    
    println!("integration_test_map_myhashmap_remove_case3 (milliseconds): {}",
    elapsed_time.as_millis());

    println!("integration_test_map_myhashmap_remove_case3 (nanoseconds): {}",
    elapsed_time.as_nanos());
}

#[test]
fn test_hashmap_size_case1() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    assert_eq!(mymap.size(),0);
}

#[test]
fn test_hashmap_size_case2() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    assert_eq!(mymap.size(),1);
}

#[test]
fn test_hashmap_size_case3() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    assert_eq!(mymap.size(),1);
}

#[test]
fn test_hashmap_size_case4() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    let key = String::from("Hello2");
    let value = String::from("World");
    mymap.put(key,value);
    
    assert_eq!(mymap.size(),2);
}

#[test]
fn test_hashmap_size_case5() {
    let mut mymap:MyHashMap<String,String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key,value);

    let key = String::from("Hello2");
    let value = String::from("World");
    mymap.put(key,value);
    
    let key = String::from("Hello");
    mymap.remove(&key);
    assert_eq!(mymap.size(),1);
}