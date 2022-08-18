use super::mymap::MyMap;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct MyHashMap<K: Eq + Hash, V> {
    values: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MyHashMap<K, V> {
    pub fn new() -> Self {
        MyHashMap {
            values: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> MyMap<K, V> for MyHashMap<K, V> {
    fn nil() -> Self {
        MyHashMap {
            values: HashMap::new(),
        }
    }

    fn put(self: &mut Self, key: K, value: V) {
        self.values.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }

    fn remove(self: &mut Self, key: &K) {
        self.values.remove(key);
    }

    fn size(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests;
