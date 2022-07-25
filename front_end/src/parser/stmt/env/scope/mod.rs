use std::{hash::Hash, collections::HashMap, cell::RefCell, rc::Rc};
use common::pointer::Pointer;

pub type Ptr<T> = Rc<T>;
pub type Scope<K:Hash+Eq,V:PartialEq> = HashMap<K,V>;
pub type ScopeStack<K:Hash+Eq,V:PartialEq> = Vec<Scope<K,V>>;
pub type ScopeStackMutPtr<K:Hash+Eq,V:PartialEq> = Rc<RefCell<ScopeStack<K,V>>>;


pub fn new_scope<K,V>() -> Scope<K,V> where K:Hash+Eq, V:PartialEq {
    let scope:Scope<K,V> = HashMap::new();
    scope
}

pub fn new_scope_stack_mut_ptr<K,V>() -> ScopeStackMutPtr<K,V>
    where K:Hash+Eq, V:PartialEq {
        let stack:Vec<Scope<K,V>> = Vec::new();
        Pointer::new_mut_pointer(stack)
}

pub fn push<K,V>(scope:Scope<Ptr<K>,Ptr<V>>,scope_stack_ptr:&ScopeStackMutPtr<Ptr<K>,Ptr<V>>) 
    where K:Hash+Eq,V:PartialEq {
        scope_stack_ptr.as_ref().borrow_mut().push(scope);
}

pub fn pop<K,V>(scope_stack_ptr:&ScopeStackMutPtr<Ptr<K>,Ptr<V>>) 
    where K:Hash+Eq,V:PartialEq {
         scope_stack_ptr.as_ref().borrow_mut().pop();
}

pub fn find<K,V>(key:&Ptr<K>,scopes_ptr:&ScopeStackMutPtr<Ptr<K>,Ptr<V>>) -> 
    Option<Ptr<V>> where K: Hash+Eq,V:PartialEq {
        let scopes = 
            scopes_ptr.as_ref().borrow();
        let mut scopes_itr = 
            scopes.iter().rev();
        let map_result = 
            scopes_itr.find(|m| m.get(key) != None);
        if let Some(map) = map_result {
            let x = map.get(key).unwrap();
            Some(Pointer::clone(x))
        } else {
            None
        }
}

#[cfg(test)]
mod tests;