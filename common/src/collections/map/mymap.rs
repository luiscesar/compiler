pub trait MyMap<K, V> {
    fn nil() -> Self;
    fn put(self: &mut Self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(self: &mut Self, key: &K);
    fn size(&self) -> usize;
}
