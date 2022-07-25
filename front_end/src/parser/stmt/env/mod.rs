use std::{rc::Rc, cell::RefCell, iter::Map, collections::HashMap};
use common::{pointer::Pointer, collections::string::StringPtr};
use crate::parser::expr::id::IdPtr;

pub mod scope;

pub type EnvMutPtr = Rc<RefCell<Env>>;
pub struct Env {
    loop_labels:Vec<usize>,
}
impl Env {
    pub fn new() -> Env {
        Env{loop_labels:Vec::new()}
    }
    pub fn new_mut_ptr() -> EnvMutPtr {
        let env = Env::new();
        Pointer::new_mut_pointer(env)
    }
    pub fn push_loop_label(&mut self,label:usize) {
        self.loop_labels.push(label);
    }
    pub fn pop_loop_label(&mut self) {
        self.loop_labels.pop();
    }
    pub fn top_loop_label(&self) -> Option<usize> {
        let len = self.loop_labels.len();
        if len > 0 {
            Some(self.loop_labels[len-1])
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests;