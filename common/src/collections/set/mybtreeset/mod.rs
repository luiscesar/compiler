use super::myset::MySet;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct MyBTreeSet<T>
where
    T: Eq,
{
    values: BTreeSet<T>,
}

impl<T: Eq> MyBTreeSet<T> {
    pub fn from(vals: BTreeSet<T>) -> MyBTreeSet<T> {
        let mut myset = MyBTreeSet { values: vals };
        myset
    }
}

impl<T: Eq + Ord> MySet<T> for MyBTreeSet<T> {
    fn add(self: &mut Self, element: T) {
        self.values.insert(element);
    }

    fn remove(self: &mut Self, element: &T) {
        self.values.remove(element);
    }

    fn contains(&self, element: &T) -> bool {
        self.values.contains(element)
    }

    fn size(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn nil() -> Self {
        MyBTreeSet::from(BTreeSet::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::set::mybtreeset::MyBTreeSet;
    use crate::collections::set::myset::MySet;
    use std::collections::BTreeSet;

    #[test]
    fn test_mybtreeset_case1() {
        println!("test_mybtreeset_case1");
    }

    #[test]
    fn test_mybtreeset_new_case1() {
        println!("test_mybtreeset_new_case1: begin");
        let mut v: BTreeSet<String> = BTreeSet::new();
        let mut v2: BTreeSet<String> = BTreeSet::new();
        let myset = MyBTreeSet::from(v);
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: v2 };
        println!("myset {:?}", myset);
        assert_eq!(myset, expected_myset)
    }

    #[test]
    fn test_mybtreeset_nil_case1() {
        let mut v2: BTreeSet<String> = BTreeSet::new();
        let myset = MyBTreeSet::nil();
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: v2 };
        println!("myset {:?}", myset);
        assert_eq!(myset, expected_myset)
    }

    #[test]
    fn test_mybtreeset_add_case1() {
        let mut v: BTreeSet<String> = BTreeSet::new();
        let mut v2: BTreeSet<String> = BTreeSet::new();
        v2.insert(String::from("Hello"));
        let mut myset = MyBTreeSet::from(v);
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: v2 };
        myset.add(String::from("Hello"));
        println!("expected_myset {:?}", expected_myset);
        println!("myset {:?}", myset);
        assert_eq!(myset, expected_myset);
    }

    #[test]
    fn test_mybtreeset_remove_case1() {
        let v: BTreeSet<String> = BTreeSet::from([String::from("Hello")]);
        let v2: BTreeSet<String> = BTreeSet::from([String::from("Hello")]);
        let mut myset = MyBTreeSet::from(v);
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: v2 };
        myset.add(String::from("Hello2"));
        myset.remove(&String::from("Hello2"));
        assert_eq!(myset, expected_myset);
    }

    #[test]
    fn test_mybtreeset_size_case1() {
        let v: BTreeSet<String> = BTreeSet::from([String::from("He"), String::from("llo")]);
        let myset = MyBTreeSet::from(v);
        assert_eq!(myset.size(), 2);
    }

    #[test]
    fn test_mybtreeset_is_empty_case1() {
        let mut v: BTreeSet<String> = BTreeSet::new();
        let myset = MyBTreeSet::from(v);
        assert!(myset.is_empty());
    }

    #[test]
    fn test_mybtreeset_is_empty_case2() {
        let mut v: BTreeSet<String> = BTreeSet::from([String::from("Hello")]);
        let myset = MyBTreeSet::from(v);
        assert!(!myset.is_empty());
    }
}
