use std::rc::Rc;

pub type StringPtr = Rc<String>;

#[cfg(test)]
mod tests;
