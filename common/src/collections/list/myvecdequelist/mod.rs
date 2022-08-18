use super::mylist::MyList;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct MyVecDequeList<T> {
    values: VecDeque<T>,
}

impl<T> MyVecDequeList<T> {
    pub fn new() -> MyVecDequeList<T> {
        let vals: VecDeque<T> = VecDeque::with_capacity(10);
        MyVecDequeList { values: vals }
    }

    pub fn from(vals: VecDeque<T>) -> MyVecDequeList<T> {
        MyVecDequeList { values: vals }
    }
}

impl<T> MyList<T> for MyVecDequeList<T> {
    fn nil() -> Self {
        MyVecDequeList::new()
    }

    fn cons(self: &mut Self, element: T) {
        self.values.push_back(element);
    }

    fn head(self: &mut Self) -> Option<&T> {
        self.values.back()
    }

    fn tail(self: &mut Self) -> () {
        let head = self.values.pop_back();
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyVecDequeList;
    use crate::collections::list::mylist::MyList;
    use std::collections::VecDeque;

    #[test]
    fn test_myvecdequelist_create_case1() {
        let mylist: MyVecDequeList<String> = MyVecDequeList {
            values: VecDeque::new(),
        };
    }

    #[test]
    fn test_myvecdequelist_new_case1() {
        let mylist: MyVecDequeList<String> = MyVecDequeList::new();
        println!("mylist {:?}", mylist);
        let empty_list = VecDeque::new();
        assert_eq!(mylist, MyVecDequeList { values: empty_list });
    }

    #[test]
    fn test_myvecdequelist_from_case1() {
        let vs = ["Hello"];
        let vdqs = VecDeque::from([vs]);
        let vdqs2 = VecDeque::from([vs]);

        let mylist = MyVecDequeList::from(vdqs);
        println!("mylist {:?}", mylist);
        let expected = MyVecDequeList { values: vdqs2 };
        assert_eq!(mylist, expected);
    }

    #[test]
    fn test_myvecdequelist_nil_case1() {
        let mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        let empty_list = VecDeque::new();
        assert_eq!(mylist, MyVecDequeList { values: empty_list });
    }

    #[test]
    fn test_myvecdequelist_cons_case1() {
        let mut mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        mylist.cons(String::from("Hello"));
        println!("mylist {:?}", mylist);
    }

    #[test]
    fn test_myvecdequelist_head_case1() {
        let mut mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        assert_eq!(mylist.head(), None);
    }

    #[test]
    fn test_myvecdequelist_head_case2() {
        let mut mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        mylist.cons(String::from("Hello"));
        assert_eq!(mylist.head(), Some(&String::from("Hello")));
    }

    #[test]
    fn test_myvecdequelist_tail_case1() {
        let mut mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        mylist.cons(String::from("Hello1"));
        mylist.cons(String::from("Hello2"));
        mylist.cons(String::from("Hello3"));

        mylist.tail();
        let tail = mylist;

        let mut expected: MyVecDequeList<String> = MyVecDequeList::nil();
        expected.cons(String::from("Hello1"));
        expected.cons(String::from("Hello2"));
        assert_eq!(tail, expected);
    }

    #[test]
    fn test_myvecdequelist_is_empty_case1() {
        let mut mylist: MyVecDequeList<String> = MyVecDequeList::nil();
        mylist.cons(String::from("Hello1"));
        mylist.cons(String::from("Hello2"));
        mylist.cons(String::from("Hello3"));
        assert!(!mylist.is_empty());
    }
}
