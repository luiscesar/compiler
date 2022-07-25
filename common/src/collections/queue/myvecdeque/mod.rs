use super::myqueue::MyQueue;
use std::collections::VecDeque;

#[derive(Debug,PartialEq)]
pub struct MyVecDeque<T> {
    values:VecDeque<T>,
}

impl<T> MyVecDeque<T> {
    pub fn new() -> Self {
        let mut values:VecDeque<T> = VecDeque::new();
        MyVecDeque { values: values }
    }
}

impl<T> MyQueue<T> for MyVecDeque<T> {
    fn enqueue(self:&mut Self, element:T) {
        self.values.push_back(element);
    }

    fn dequeue(self:&mut Self) {
        self.values.pop_front();
    }

    fn first(&self) -> Option<&T> {
        self.values.front()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn size(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::MyVecDeque;
    use super::MyQueue;


    #[test]
    fn test_myvecqueue_case1() {
        let myqueue:MyVecDeque<String> = MyVecDeque{values:VecDeque::new()};
        let myqueue2:MyVecDeque<String> = MyVecDeque{values:VecDeque::new()};
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_new_case1() {
        let myqueue:MyVecDeque<String> = MyVecDeque::new();
        let myqueue2:MyVecDeque<String> = MyVecDeque{values:VecDeque::new()};
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_enqueue_case1() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        myqueue.enqueue("World".to_string());
        let myqueue2:MyVecDeque<String> = MyVecDeque{
            values:VecDeque::from(
            [String::from("Hello"),String::from("World")]
        )};
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_dequeue_case1() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.dequeue();
        let myqueue2:MyVecDeque<String> = MyVecDeque::new();
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_dequeue_case2() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        myqueue.dequeue();
        let myqueue2:MyVecDeque<String> = MyVecDeque::new();
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_dequeue_case3() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        myqueue.enqueue("World".to_string());
        myqueue.dequeue();
        let mut myqueue2:MyVecDeque<String> = MyVecDeque::new();
        myqueue2.enqueue("World".to_string());
        assert_eq!(myqueue,myqueue2);
    }

    #[test]
    fn test_myvecqueue_first_case1() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        assert_eq!(myqueue.first(),None);
    }

    #[test]
    fn test_myvecqueue_first_case2() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        assert_eq!(myqueue.first(),Some(&"Hello".to_string()));
    }

    #[test]
    fn test_myvecqueue_first_case3() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        myqueue.enqueue("World".to_string());
        assert_eq!(myqueue.first(),Some(&"Hello".to_string()));
    }

    #[test]
    fn test_myvecqueue_is_empty_case2() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        assert!(!myqueue.is_empty());
    }

    #[test]
    fn test_myvecqueue_is_empty_case3() {
        let mut myqueue:MyVecDeque<String> = MyVecDeque::new();
        myqueue.enqueue("Hello".to_string());
        myqueue.enqueue("World".to_string());
        assert!(!myqueue.is_empty());
    }

    #[test]
    fn test_myvecqueue_size_case1() {
        let mut my_queue:MyVecDeque<String> = MyVecDeque::new();
        let expected_size = 0;
        assert_eq!(my_queue.size(),expected_size);
    }

    #[test]
    fn test_myvecqueue_size_case2() {
        let mut my_queue:MyVecDeque<String> = MyVecDeque::new();
        my_queue.enqueue("Hello".to_string());
        let expected_size = 1;
        assert_eq!(my_queue.size(),expected_size);
    }

    #[test]
    fn test_myvecqueue_size_case3() {
        let mut my_queue:MyVecDeque<String> = MyVecDeque::new();
        my_queue.enqueue("Hello".to_string());
        my_queue.enqueue("World".to_string());

        let expected_size = 2;
        assert_eq!(my_queue.size(),expected_size);
    }
}