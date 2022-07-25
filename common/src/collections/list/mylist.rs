
pub trait MyList<T> {
    fn nil() -> Self;
    fn cons(self:&mut Self, element:T);
    fn head(self:&mut Self) -> Option<&T>;
    //fn tail(self:&mut Self) -> Option<&mut Self> where Self: Sized;
    fn tail(self:&mut Self);
    fn is_empty(&self) -> bool;
}

