use std::vec::Vec;
use super::mylist::MyList;
 
#[derive(Debug,PartialEq)]
pub struct MyVecList<T> {
    values:Vec<T>,
}

impl<T> MyVecList<T> {
    fn new() -> MyVecList<T> {
        let vals:Vec<T> = Vec::with_capacity(100_000);
        MyVecList{values:vals}
    }
    fn from(vals:Vec<T>) -> MyVecList<T> {
        MyVecList{values:vals}
    }
}

impl<T> MyList<T> for MyVecList<T> {
    fn nil() -> Self {
        MyVecList::new()
    }

    fn cons(self:&mut Self, element:T) {
        self.values.push(element);
    }

    fn head(self:&mut Self) -> Option<&T> {
        match self {
           x if self.is_empty() => None,
            _ => Some(&self.values[self.values.len()-1]),
        }
        /* 
        if self.values.is_empty() {
            None
        } else {
            Some(&self.values[self.values.len()-1])
        }
        */
    }

    fn tail(self:&mut Self) {
        self.values.pop();
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::mylist::MyList;
    use super::MyVecList;
    use std::vec::Vec;

    use std::time::Instant;
    use rand::Rng;
    const MAX:i32 = 100_000;

    #[test]
    pub fn test_list_myveclist_case1() {
        let my_list:MyVecList<String> = MyVecList{values:Vec::new()};
    }

    #[test]
    pub fn test_list_myveclist_new_case1() {
        let my_list:MyVecList<String> = MyVecList::new();
        let my_list2:MyVecList<String> = MyVecList{values:Vec::new()};
        assert_eq!(my_list,my_list2);
    }

    #[test]
    pub fn test_list_myveclist_nil_case1() {
        let my_list:MyVecList<String> = MyVecList::nil();
        let my_list2:MyVecList<String> = MyVecList::new();
        assert_eq!(my_list,my_list2);
    }

    #[test]
    pub fn test_list_myveclist_cons_case1() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        let expected_list:MyVecList<String> = 
            MyVecList::from(Vec::from(["Hello".to_string()]));
        assert_eq!(my_list,expected_list);
    }

    #[test]
    pub fn test_list_myveclist_cons_case2() {
        let mut my_list: MyVecList<String> = MyVecList::nil();
        
        let now = Instant::now();
        for index in 1..(MAX+1) {
            let element = String::from("Hello") + &index.to_string();
            my_list.cons(element);
        }
        let elapased_time = now.elapsed();
        assert!(!my_list.is_empty());

        println!("MAX {}", MAX);
        println!("integration test myvecdequelist case1 (nanos): {}", 
            elapased_time.as_nanos());
        println!("integration test myvecdequelist case1 (millis): {}", 
            elapased_time.as_millis());
    }

    #[test]
    pub fn test_list_myveclist_is_empty_case1() {
        let my_list:MyVecList<String> = MyVecList::nil();
        assert!(my_list.is_empty());
    }

    #[test]
    pub fn test_list_myveclist_is_empty_case2() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        assert!(!my_list.is_empty());
    }

    #[test]
    pub fn test_list_myveclist_head_case1() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let head = my_list.head();
        let expected_head = None;
        assert_eq!(head,expected_head);
    }

    #[test]
    pub fn test_list_myveclist_head_case2() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        let head = my_list.head();
        let element = String::from("Hello");
        let expected_head = Some(&element);
        assert_eq!(head,expected_head);
    }

    #[test]
    pub fn test_list_myveclist_head_case3() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        let element = String::from("World");
        my_list.cons(element);
        let head = my_list.head();
        let element = String::from("World");
        let expected_head = Some(&element);
        assert_eq!(head,expected_head);
    }

    #[test]
    pub fn test_list_myveclist_tail_case1() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        my_list.tail();
        let head = my_list.head();
        let expected_head = None;
        assert_eq!(head,expected_head);
    }

    #[test]
    pub fn test_list_myveclist_tail_case2() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        my_list.tail();
        let head = my_list.head();
        let expected_head = None;
        assert_eq!(head,expected_head);
    }

    #[test]
    pub fn test_list_myveclist_tail_case3() {
        let mut my_list:MyVecList<String> = MyVecList::nil();
        let element = String::from("Hello");
        my_list.cons(element);
        let element = String::from("World");
        my_list.cons(element);
        my_list.tail();
        let head = my_list.head();
        let element = String::from("Hello");
        let expected_head = Some(&element);
        assert_eq!(head,expected_head);
    }

}