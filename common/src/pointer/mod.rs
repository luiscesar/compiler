use std::{cell::RefCell, rc::Rc};

pub struct Pointer {}

impl Pointer {
    pub fn new_box_pointer<T>(data: T) -> Box<T> {
        Box::new(data)
    }

    pub fn new_pointer<T>(data: T) -> Rc<T> {
        Rc::new(data)
    }
    pub fn clone<T>(p: &Rc<T>) -> Rc<T> {
        Rc::clone(p)
    }

    pub fn new_mut_pointer<T>(data: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(data))
    }
}

#[cfg(test)]
mod tests;
