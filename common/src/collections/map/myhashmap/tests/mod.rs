use crate::{collections::string::StringPtr, pointer::Pointer};
use std::hash::Hash;

use super::MyHashMap;
use super::MyMap;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

#[test]
fn test_myhashmap_case1() {
    let mymap: MyHashMap<String, String> = MyHashMap {
        values: HashMap::new(),
    };
    let mymap2: MyHashMap<String, String> = MyHashMap {
        values: HashMap::new(),
    };
    assert_eq!(mymap, mymap2);
}

#[test]
fn test_myhashmap_new_case1() {
    let mymap: MyHashMap<String, String> = MyHashMap::new();
    let mymap2: MyHashMap<String, String> = MyHashMap {
        values: HashMap::new(),
    };
    assert_eq!(mymap, mymap2);
}

#[test]
fn test_myhashmap_nil_case1() {
    let mymap: MyHashMap<String, String> = MyHashMap::nil();
    let mymap2: MyHashMap<String, String> = MyHashMap {
        values: HashMap::new(),
    };
    assert_eq!(mymap, mymap2);
}

#[test]
fn test_myhashmap_put_case1() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);
    let mymap2: MyHashMap<String, String> = MyHashMap {
        values: HashMap::from([(String::from("Hello"), String::from("World"))]),
    };
    assert_eq!(mymap, mymap2);
    let key2 = String::from("Hello2");
    let value = String::from("World");
    mymap.put(key2, value);
}

#[test]
fn test_myhashmap_get_case1() {
    let mymap: MyHashMap<String, String> = MyHashMap::nil();

    let key = String::from("Hello");
    let value = None;
    assert_eq!(value, mymap.get(&key));
}

#[test]
fn test_myhashmap_get_case2() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    let value = String::from("World");
    let key = String::from("Hello");
    let value = Some(&value);
    assert_eq!(value, mymap.get(&key));
}

#[test]
fn test_myhashmap_remove_case1() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();

    let key = String::from("Hello");
    mymap.remove(&key);

    let value = None;
    assert_eq!(value, mymap.get(&key));
}

#[test]
fn test_myhashmap_remove_case2() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    let key = String::from("Hello");
    mymap.remove(&key);

    let value = None;
    assert_eq!(value, mymap.get(&key));
}

#[test]
fn test_hashmap_size_case1() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    assert_eq!(mymap.size(), 0);
}

#[test]
fn test_hashmap_size_case2() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    assert_eq!(mymap.size(), 1);
}

#[test]
fn test_hashmap_size_case3() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    assert_eq!(mymap.size(), 1);
}

#[test]
fn test_hashmap_size_case4() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    let key = String::from("Hello2");
    let value = String::from("World");
    mymap.put(key, value);

    assert_eq!(mymap.size(), 2);
}

#[test]
fn test_hashmap_size_case5() {
    let mut mymap: MyHashMap<String, String> = MyHashMap::nil();
    let key = String::from("Hello");
    let value = String::from("World");
    mymap.put(key, value);

    let key = String::from("Hello2");
    let value = String::from("World");
    mymap.put(key, value);

    let key = String::from("Hello");
    mymap.remove(&key);
    assert_eq!(mymap.size(), 1);
}

#[test]
fn test_hashmap_string_ptr_case1() {
    let key = StringPtr::new("key".to_string());
    let value = StringPtr::new("value".to_string());
    let mut map: HashMap<StringPtr, StringPtr> = HashMap::new();
    map.insert(key, value);

    let key1 = StringPtr::new("key".to_string());
    let v1 = map.get(&key1).unwrap();
    assert_eq!(*v1.to_string(), "value".to_string());

    let key2 = Pointer::clone(&key1);
    let v2 = map.get(&key2).unwrap();
    assert_eq!(*v2.to_string(), "value".to_string());

    let mut scopes: Vec<HashMap<StringPtr, StringPtr>> = Vec::new();
    scopes.push(map);

    let scopes: Vec<HashMap<StringPtr, StringPtr>> = Vec::new();
    let scopes_pointer = Pointer::new_mut_pointer(scopes);

    let mut map: HashMap<StringPtr, StringPtr> = HashMap::new();
    let key = StringPtr::new("key1".to_string());
    let value = StringPtr::new("value1".to_string());
    map.insert(key, value);
    scopes_pointer.as_ref().borrow_mut().push(map);

    let k = StringPtr::new("key1".to_string());
    let v = Pointer::clone((*scopes_pointer).borrow().get(0).unwrap().get(&k).unwrap());
    assert_eq!((*v).to_string(), "value1".to_string());

    let mut map: HashMap<StringPtr, StringPtr> = HashMap::new();
    let key = StringPtr::new("key1".to_string());
    let value = StringPtr::new("value2".to_string());
    map.insert(key, value);
    scopes_pointer.as_ref().borrow_mut().push(map);

    let k1 = StringPtr::new("key1".to_string());
    let v = find2(&k1, &scopes_pointer);
    assert_eq!(v.unwrap().as_ref().to_string(), "value2".to_string());

    scopes_pointer.as_ref().borrow_mut().pop();

    let k1 = StringPtr::new("key1".to_string());
    let v = find2(&k1, &scopes_pointer);
    assert_eq!(v.unwrap().as_ref().to_string(), "value1".to_string());
}

fn find(
    key: &StringPtr,
    scopes_ptr: &Rc<RefCell<Vec<HashMap<StringPtr, StringPtr>>>>,
) -> Option<StringPtr> {
    let scopes_ptr_clone = Pointer::clone(scopes_ptr);
    let scopes = scopes_ptr_clone.as_ref().borrow();
    let mut scopes_itr = scopes.iter().rev();
    let map_result = scopes_itr.find(|m| m.get(key) != None);
    if let Some(map) = map_result {
        let x = map.get(key).unwrap();
        Some(Pointer::clone(x))
    } else {
        None
    }
}
pub type Scopes<K: Hash + Eq, V: PartialEq> = Vec<HashMap<K, V>>;
pub type ScopesPtr<K: Hash + Eq, V: PartialEq> = Rc<RefCell<Scopes<K, V>>>;

fn find2<K, V>(key: &Rc<K>, scopes_ptr: &Rc<RefCell<Vec<HashMap<Rc<K>, Rc<V>>>>>) -> Option<Rc<V>>
where
    K: Hash + Eq,
    V: PartialEq,
{
    let scopes = scopes_ptr.as_ref().borrow();
    let mut scopes_itr = scopes.iter().rev();
    let map_result = scopes_itr.find(|m| m.get(key) != None);
    if let Some(map) = map_result {
        let x = map.get(key).unwrap();
        Some(Pointer::clone(x))
    } else {
        None
    }
}
