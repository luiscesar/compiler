use std::rc::Rc;

use crate::lexer::token::Basic;

pub type TypePtr = Rc<Type>;

pub trait Typed {
    fn element_type(&self) -> Type;
}

pub const INT:Type = Type::Basic(BasicType::Int);
pub const FLOAT:Type = Type::Basic(BasicType::Float);
pub const BOOL:Type = Type::Basic(BasicType::Bool);

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Type {
   Basic(BasicType),
   Array(BasicType,usize),
}
#[derive(Debug,PartialEq,Clone,Copy)]
pub enum BasicType {
    Int,Float,Bool,
}

pub fn basic_type(t:Type) -> BasicType {
    match t {
        Type::Array(t, size) => t,
        Type::Basic(t) => t,
    }
}
pub fn basic_type_from_token(token_type:&Basic) -> BasicType {
    match token_type {
        Basic::Int => BasicType::Int,
        Basic::Float => BasicType::Float,
        Basic::Bool => BasicType::Bool,
    }
}

#[cfg(test)]
mod tests;