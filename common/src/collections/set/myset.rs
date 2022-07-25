
pub trait MySet<T:Eq> {
    fn nil() -> Self;
    fn add(self:&mut Self, element:T);
    fn remove(self:&mut Self, element:&T);
    fn contains(&self, element:&T) -> bool;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_myset_case1() {
        println!("test_myset_case1");
    }
}