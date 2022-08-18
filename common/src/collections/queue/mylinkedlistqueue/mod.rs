use super::myqueue::MyQueue;
use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub struct MyLinkedListQueue<T> {
    values: LinkedList<T>,
}

impl<T> MyLinkedListQueue<T> {
    pub fn new() -> MyLinkedListQueue<T> {
        MyLinkedListQueue {
            values: LinkedList::new(),
        }
    }
}

impl<T> MyQueue<T> for MyLinkedListQueue<T> {
    fn enqueue(self: &mut Self, element: T) {
        self.values.push_back(element);
    }

    fn dequeue(self: &mut Self) {
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
    use crate::collections::queue::myqueue::MyQueue;

    use super::MyLinkedListQueue;
    use std::collections::LinkedList;

    #[test]
    fn test_mylinkedlistqueue_new_case1() {
        let myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let expected_myqueue: MyLinkedListQueue<String> = MyLinkedListQueue {
            values: LinkedList::new(),
        };
        assert_eq!(myqueue, expected_myqueue);
    }

    #[test]
    fn test_mylinkedlistqueue_new_enqueue_case1() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        myqueue.enqueue(element);

        let element = String::from("Hello");
        let expected_myqueue: MyLinkedListQueue<String> = MyLinkedListQueue {
            values: LinkedList::from([element]),
        };

        assert_eq!(myqueue, expected_myqueue);
    }

    #[test]
    fn test_mylinkedlistqueue_new_enqueue_case2() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        let element2 = String::from("World");
        myqueue.enqueue(element);
        myqueue.enqueue(element2);

        let element = String::from("Hello");
        let element2 = String::from("World");
        let expected_myqueue: MyLinkedListQueue<String> = MyLinkedListQueue {
            values: LinkedList::from([element, element2]),
        };

        assert_eq!(myqueue, expected_myqueue);
    }

    #[test]
    fn test_mylinkedlistqueue_first_case1() {
        let myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();

        let first = myqueue.first();
        let expected_first = None;
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_first_case2() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        myqueue.enqueue(element);

        let first = myqueue.first();

        let element = String::from("Hello");
        let expected_first = Some(&element);
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_first_case3() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        let element2 = String::from("World");
        myqueue.enqueue(element);
        myqueue.enqueue(element2);

        let first = myqueue.first();

        let element = String::from("Hello");
        let expected_first = Some(&element);
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_dequeue_case1() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();

        myqueue.dequeue();

        let first = myqueue.first();
        let expected_first = None;
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_dequeue_case2() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        myqueue.enqueue(element);

        myqueue.dequeue();

        let first = myqueue.first();
        let expected_first = None;
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_dequeue_case3() {
        let mut myqueue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        let element2 = String::from("World");
        myqueue.enqueue(element);
        myqueue.enqueue(element2);

        myqueue.dequeue();

        let first = myqueue.first();
        let element = String::from("World");
        let expected_first = Some(&element);
        assert_eq!(first, expected_first);
    }

    #[test]
    fn test_mylinkedlistqueue_is_empty_case1() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        assert!(my_queue.is_empty());
    }

    #[test]
    fn test_mylinkedlistqueue_is_empty_case2() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        my_queue.enqueue(element);

        assert!(!my_queue.is_empty());
    }

    #[test]
    fn test_mylinkedlistqueue_is_empty_case3() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        my_queue.enqueue(element);
        my_queue.dequeue();
        assert!(my_queue.is_empty());
    }

    #[test]
    fn test_mylinkedlistqueue_size_case1() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let expected_size = 0;
        assert_eq!(my_queue.size(), expected_size);
    }

    #[test]
    fn test_mylinkedlistqueue_size_case2() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        my_queue.enqueue(element);
        let expected_size = 1;
        assert_eq!(my_queue.size(), expected_size);
    }

    #[test]
    fn test_mylinkedlistqueue_size_case3() {
        let mut my_queue: MyLinkedListQueue<String> = MyLinkedListQueue::new();
        let element = String::from("Hello");
        my_queue.enqueue(element);
        my_queue.dequeue();
        let expected_size = 0;
        assert_eq!(my_queue.size(), expected_size);
    }
}
