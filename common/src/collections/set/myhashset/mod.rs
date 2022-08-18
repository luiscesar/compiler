use super::myset::MySet;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq)]
pub struct MyHashSet<T>
where
    T: Eq + Hash,
{
    values: HashSet<T>,
}

impl<T: Eq + Hash> MyHashSet<T> {
    pub fn new() -> MyHashSet<T> {
        let v = HashSet::with_capacity(10);
        MyHashSet { values: v }
    }
}

impl<T: Eq + Hash> MySet<T> for MyHashSet<T> {
    fn add(&mut self, element: T) {
        self.values.insert(element);
    }

    fn remove(self: &mut Self, element: &T) {
        self.values.remove(element);
    }

    fn contains(&self, element: &T) -> bool {
        self.values.contains(&element)
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn nil() -> Self {
        MyHashSet::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::set::myhashset::MyHashSet;
    use crate::collections::set::myset::MySet;

    #[test]
    pub fn test_myhashset_new_case1() {
        println!("test_my_hash_set_new_case1");
        let my_hash_set: MyHashSet<i32> = MyHashSet::new();
        println!("size {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_new_case2() {
        let my_hash_set: MyHashSet<String> = MyHashSet::new();
        println!("size {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_nil_case1() {
        let my_hash_set: MyHashSet<String> = MyHashSet::nil();
        println!("size {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_add_case1() {
        println!("test_my_hash_set_add_case1");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        println!("size {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 1);
    }

    #[test]
    pub fn test_myhashset_add_case2() {
        let element = String::from("Hello");
        let mut my_hash_set: MyHashSet<String> = MyHashSet::new();
        my_hash_set.add(element);
        println!("size {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 1);
    }

    #[test]
    pub fn test_myhashset_remove_case1() {
        println!("test_my_hash_set_remove_case1");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.remove(&1);
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_remove_case2() {
        let mut my_hash_set: MyHashSet<String> = MyHashSet::new();
        let element = String::from("Hello");
        my_hash_set.add(element);
        let element = String::from("Hello");
        my_hash_set.remove(&element);
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_contains_case1() {
        println!("test_my_hash_set_contains_case1");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        assert!(my_hash_set.contains(&1));
    }

    #[test]
    pub fn test_myhashset_size_case1() {
        println!("test_my_hash_set_size_case1");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.remove(&1);
        println!("size = {}", my_hash_set.size());
        assert_eq!(my_hash_set.size(), 0);
    }

    #[test]
    pub fn test_myhashset_is_empty_case1() {
        println!("test_my_hash_set_size_case1");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.remove(&1);
        println!("size = {}", my_hash_set.size());
        assert!(my_hash_set.is_empty());
    }

    #[test]
    pub fn test_myhashset_is_empty_case2() {
        println!("test_my_hash_set_is_empty_case2");
        let mut my_hash_set: MyHashSet<i32> = MyHashSet::new();
        my_hash_set.add(1);
        println!("size = {}", my_hash_set.size());
        assert!(!my_hash_set.is_empty());
    }

    #[test]
    pub fn test_myhashset_is_empty_case3() {
        println!("test_my_hash_set_is_empty_case3");
        let my_hash_set: MyHashSet<i32> = MyHashSet::new();
        println!("size = {}", my_hash_set.size());
        assert!(my_hash_set.is_empty());
    }
}
