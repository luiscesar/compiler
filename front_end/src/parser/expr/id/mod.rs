use std::{
    fmt::{self, Display},
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use common::{collections::string::StringPtr, pointer::Pointer};

use crate::parser::expr::types::{Type, TypePtr, Typed};

use super::{types::BOOL, Expr, ExprPtr, ExprT};

const ID_TMP_NAME_PREFIX: &str = "t";
static COUNT: AtomicUsize = AtomicUsize::new(0);

pub type IdPtr = Rc<Id>;

#[derive(Debug, PartialEq)]
pub struct Id {
    name: StringPtr,
    id_type: Type,
}

impl Id {
    pub fn new(name: StringPtr, id_type: Type) -> Id {
        Id {
            name: name,
            id_type: id_type,
        }
    }

    pub fn new_ptr(name: StringPtr, id_type: Type) -> IdPtr {
        let id = Id {
            name: name,
            id_type: id_type,
        };
        Pointer::new_pointer(id)
    }

    pub fn new_id_tmp_name() -> String {
        let count = COUNT.fetch_add(1, Ordering::Relaxed);
        format!("{}{}", ID_TMP_NAME_PREFIX, count)
    }

    pub fn new_id_tmp(id_type: Type) -> Id {
        Id::new(Pointer::new_pointer(Id::new_id_tmp_name()), id_type)
    }

    pub fn count() -> usize {
        let count = COUNT.fetch_add(0, Ordering::Relaxed);
        count
    }
}

impl Typed for Id {
    fn element_type(&self) -> Type {
        self.id_type
    }
}

impl ExprT for IdPtr {
    fn reduce(&self) -> ExprPtr {
        let t = Pointer::clone(self);
        Pointer::new_pointer(Expr::ID(t))
    }
}

impl Typed for IdPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests;
