pub trait MyQueue<T> {
    fn enqueue(self: &mut Self, element: T);
    fn dequeue(self: &mut Self);
    fn first(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
