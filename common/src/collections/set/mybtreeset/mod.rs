use super::myset::MySet;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct MyBTreeSet<T>
where
    T: Eq,
{
    values: BTreeSet<T>,
}

impl<T:Eq> MyBTreeSet<T> {
    pub fn new() -> MyBTreeSet<T> {
        MyBTreeSet { values: BTreeSet::new() }
    }
}

impl<T:Ord, const N: usize> From<[T; N]> for MyBTreeSet<T> {
    fn from(arr: [T; N]) -> Self {
        if N == 0 {
            return MyBTreeSet{values:BTreeSet::new()}
        }
        
        let mut my_set = MyBTreeSet::new();
        for x in arr {
            my_set.add(x);
        }
        my_set
        
        //let values = BTreeSet::from(arr);
        //MyBTreeSet { values }
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
        MyBTreeSet::new()
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
        let myset = MyBTreeSet::new();
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: BTreeSet::new() };
        assert_eq!(myset, expected_myset)
    }

    #[test]
    fn test_mybtreeset_nil_case1() {
        let myset = MyBTreeSet::nil();
        let expected_myset: MyBTreeSet<String> = MyBTreeSet { values: BTreeSet::new() };
        println!("myset {:?}", myset);
        assert_eq!(myset, expected_myset)
    }

    #[test]
    fn test_mybtreeset_add_case1() {
        let mut myset = MyBTreeSet::new();
        let expected_myset: MyBTreeSet<String> = MyBTreeSet::from(["Hello".to_string()]);
        myset.add(String::from("Hello"));
        println!("expected_myset {:?}", expected_myset);
        println!("myset {:?}", myset);
        assert_eq!(expected_myset, myset);
    }

    #[test]
    fn test_mybtreeset_remove_case1() {
        let mut myset = 
            MyBTreeSet::from([String::from("Hello")]);
        let expected_myset: MyBTreeSet<String> = 
            MyBTreeSet { values: BTreeSet::from([String::from("Hello")]) };
        myset.add(String::from("Hello2"));
        myset.remove(&String::from("Hello2"));
        assert_eq!(myset, expected_myset);
    }

    #[test]
    fn test_mybtreeset_size_case1() {
        let myset:MyBTreeSet<String> = 
            MyBTreeSet::from([String::from("He"), String::from("llo")]);
        assert_eq!(myset.size(), 2);
    }

    #[test]
    fn test_mybtreeset_is_empty_case1() {
        let myset:MyBTreeSet<String> = MyBTreeSet::from([]);
        assert!(myset.is_empty());
    }

    #[test]
    fn test_mybtreeset_is_empty_case2() {
        let myset:MyBTreeSet<String>= MyBTreeSet::from([String::from("Hello")]);
        assert!(!myset.is_empty());
    }
}
