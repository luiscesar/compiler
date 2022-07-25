use std::vec::Vec;

use super::mystack::MyStack;

#[derive(Debug,PartialEq)]
pub struct MyVecStack<T> {
    values:Vec<T>,
}

impl<T> MyVecStack<T> {
    pub fn new() -> Self {
        let vals:Vec<T> = Vec::new();
        MyVecStack {values:vals}
    }

    pub fn new2(element:T) -> Self {
        let mut vals:Vec<T> = Vec::new();
        vals.push(element);
        MyVecStack {values:vals}
    }

    pub fn from(vals:Vec<T>) -> Self {
        MyVecStack {values:vals}
    }
}

impl<T> MyStack<T> for MyVecStack<T> {
    fn push(self:&mut Self, element:T) {
        self.values.push(element);
    }

    fn pop(self:&mut Self) {
        self.values.pop();
    }

    fn top(&self) -> Option<&T> {
        if (self.values.is_empty()) {
            None
        } else {
            Some(&(self.values[self.values.len()-1]))
        }
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use super::MyVecStack;
    use super::MyStack;

    #[test]
    fn test_myvecstack_case1() {
        let mystack: MyVecStack<String> = 
            MyVecStack {values:Vec::new()};
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::new()};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_new_case1() {
        let mystack: MyVecStack<String> = 
            MyVecStack::new();
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::new()};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_new2_case1() {
        let mystack = 
            MyVecStack::new2(String::from("Hello"));
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::from([String::from("Hello")])};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_from_case1() {
        let mystack = 
            MyVecStack::from(Vec::from([String::from("Hello")]));
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::from([String::from("Hello")])};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_push_case1() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::from([String::from("Hello")])};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_push_case2() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        mystack.push(String::from("World"));
        let mystack2: MyVecStack<String> = 
            MyVecStack {
                values:Vec::from(
                    [String::from("Hello"),String::from("World")]
                )};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_pop_case1() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        mystack.pop();
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::new()};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_pop_case2() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        mystack.push(String::from("World"));
        mystack.pop();
        let mystack2: MyVecStack<String> = 
            MyVecStack {values:Vec::from([String::from("Hello")])};
        assert_eq!(mystack,mystack2)
    }

    #[test]
    fn test_myvecstack_top_case1() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        let top = mystack.top();
        assert_eq!(top,None)
    }

    #[test]
    fn test_myvecstack_top_case2() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        let top = mystack.top();
        assert_eq!(top,Some(&String::from("Hello")));
    }

    #[test]
    fn test_myvecstack_top_case3() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        mystack.push(String::from("World"));
        let top = mystack.top();
        assert_eq!(top,Some(&String::from("World")));
    }

    #[test]
    fn test_myvecstack_is_empty_case1() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        assert!(mystack.is_empty());
    }

    #[test]
    fn test_myvecstack_is_empty_case2() {
        let mut mystack:MyVecStack<String> = 
            MyVecStack::new();
        mystack.push(String::from("Hello"));
        assert!(!mystack.is_empty());
    }
}