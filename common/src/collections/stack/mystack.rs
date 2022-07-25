pub trait MyStack<T> {
    fn push(self:&mut Self, element:T);
    fn pop(self:&mut Self);
    fn top(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}

