use std::rc::Rc;

pub type MyList<T> = Box<MyEnumList<T>>;

#[derive(Debug, PartialEq)]
pub enum MyEnumList<T> {
    Nil(),
    Cons(Rc<T>, MyList<T>),
}

impl<T> MyEnumList<T> {
    pub fn head(list: MyEnumList<T>) -> Option<Rc<T>> {
        match list {
            MyEnumList::Nil() => None,
            MyEnumList::Cons(x, y) => Some(Rc::clone(&x)),
        }
    }

    pub fn tail(list: MyEnumList<T>) -> Option<Box<MyEnumList<T>>> {
        match list {
            MyEnumList::Nil() => None,
            MyEnumList::Cons(x, tail) => Some(tail),
        }
    }
}
