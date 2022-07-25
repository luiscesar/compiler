use std::collections::LinkedList;
use super::mylist::MyList;

#[derive(Debug,PartialEq)]
pub struct MyLinkedList<T> {
    values:LinkedList<T>
}

impl<T> MyLinkedList<T> {
    fn new() -> Self {
        MyLinkedList { values: LinkedList::<T>::new() }
    }
}

impl<T> MyList<T> for MyLinkedList<T> {
    fn nil() -> Self {
        MyLinkedList::new()
    }

    fn cons(self:&mut Self, element:T) {
        self.values.push_front(element)
    }

    fn head(self:&mut Self) -> Option<&T> {
        self.values.front()
    }

    fn tail(self:&mut Self) {
        self.values.pop_front();
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use super::MyLinkedList;
    use super::MyList;

    #[test]
    fn test_mylinkedlist_case1() {
        let mylist:MyLinkedList<String> = MyLinkedList {values:LinkedList::new()};
        let mylist2:MyLinkedList<String> = MyLinkedList {values:LinkedList::new()};
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_mylinkedlist_case2() {
        let mylist:MyLinkedList<String> = 
            MyLinkedList {values:LinkedList::from([String::from("Hello")])};
        let mylist2:MyLinkedList<String> = 
            MyLinkedList {values:LinkedList::from([String::from("Hello")])};
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_mylinkedlist_new_case1() {
        let mylist:MyLinkedList<String> = 
            MyLinkedList::new();
        let mylist2:MyLinkedList<String> = 
            MyLinkedList { values: LinkedList::new() };
        assert_eq!(mylist,mylist2)    
    }

    #[test]
    fn test_mylinkedlist_nil_case1() {
        let mylist:MyLinkedList<String> = MyLinkedList::nil();
        let mylist2:MyLinkedList<String> = 
            MyLinkedList::new();
        assert_eq!(mylist,mylist2)
    }

    #[test]
    fn test_mylinkedlist_cons_case1() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        let mylist2:MyLinkedList<String> = 
            MyLinkedList {values:LinkedList::from([String::from("Hello")])};
        assert_eq!(mylist,mylist2)
    }

    #[test]
    fn test_mylinkedlist_cons_case2() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        mylist.cons(String::from("World"));
        let mylist2:MyLinkedList<String> = 
            MyLinkedList {values:LinkedList::from(
                [String::from("World"),String::from("Hello")])};
        assert_eq!(mylist,mylist2)
    }

    #[test]
    fn test_mylinkedlist_head_case1() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        let head = mylist.head();
        assert_eq!(head,None);
    }

    #[test]
    fn test_mylinkedlist_head_case2() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        let head = mylist.head();
        assert_eq!(head,Some(&String::from("Hello")));
    }

    #[test]
    fn test_mylinkedlist_head_case3() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        mylist.cons(String::from("World"));
        let head = mylist.head();
        assert_eq!(head,Some(&String::from("World")));
    }

    #[test]
    fn test_mylinkedlist_tail_case1() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.tail();
        let mut mylist2:MyLinkedList<String> = MyLinkedList::new();
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_mylinkedlist_tail_case2() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        mylist.tail();
        let mut mylist2:MyLinkedList<String> = MyLinkedList::new();
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_mylinkedlist_tail_case3() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        mylist.cons(String::from("World"));
        mylist.tail();
        let mut mylist2:MyLinkedList<String> = MyLinkedList::new();
        mylist2.cons(String::from("Hello"));
        assert_eq!(mylist,mylist2);
    }

    #[test]
    fn test_mylinkedlist_is_empty_case1() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        assert!(mylist.is_empty());
    }

    #[test]
    fn test_mylinkedlist_is_empty_case2() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        assert!(!mylist.is_empty());
    }

    #[test]
    fn test_mylinkedlist_is_empty_case3() {
        let mut mylist:MyLinkedList<String> = MyLinkedList::new();
        mylist.cons(String::from("Hello"));
        mylist.cons(String::from("World"));
        assert!(!mylist.is_empty());
    }
}