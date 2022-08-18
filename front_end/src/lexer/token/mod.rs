use std::{fmt::Display, rc::Rc};

use common::collections::string::StringPtr;

pub const BLANK: char = ' ';
pub const BLANK_TOKEN: Token = Token::Char(BLANK);

pub const While: Token = Token::Reserved(ReservedWord::While);
pub const Do: Token = Token::Reserved(ReservedWord::Do);
pub const If: Token = Token::Reserved(ReservedWord::If);
pub const Else: Token = Token::Reserved(ReservedWord::Else);
pub const Break: Token = Token::Reserved(ReservedWord::Break);
pub const True: Token = Token::Reserved(ReservedWord::True);
pub const False: Token = Token::Reserved(ReservedWord::False);

pub const Int: Token = Token::Type(Basic::Int);
pub const Float: Token = Token::Type(Basic::Float);
pub const Bool: Token = Token::Type(Basic::Bool);

pub const And: Token = Token::Word("&&");
pub const Or: Token = Token::Word("||");
pub const Le: Token = Token::Word("<=");
pub const Ge: Token = Token::Word(">=");
pub const Eq: Token = Token::Word("==");
pub const Ne: Token = Token::Word("!=");

pub const Lt: Token = Token::Char(C_LT);
pub const Gt: Token = Token::Char(C_GT);

pub const C_AND: char = '&';
pub const C_OR: char = '|';
pub const C_EQ: char = '=';
pub const C_NE: char = '!';
pub const C_LT: char = '<';
pub const C_GT: char = '>';
pub const C_SEMICOLON: char = ';';
pub const C_OPEN_ARRAY: char = '[';
pub const C_CLOSE_ARRAY: char = ']';

pub type TokenPtr = Rc<Token>;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Char(char),
    Id(StringPtr),
    Word(&'static str),
    Constant(Value),
    Type(Basic),
    Reserved(ReservedWord),
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::Char(x) => write!(f, "{}", x),
            Token::Id(x) => write!(f, "{}", x),
            Token::Word(x) => write!(f, "{}", x),
            Token::Constant(x) => write!(f, "{}", x),
            Token::Type(x) => write!(f, "{}", x),
            Token::Reserved(x) => write!(f, "{}", x),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Basic {
    Int,
    Float,
    Bool,
}
impl Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Basic::Int => write!(f, "int"),
            Basic::Float => write!(f, "float"),
            Basic::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f64),
}
impl Eq for Value {}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => x == y,
            (Value::Int(x), Value::Float(y)) => (*x as f64) == *y,
            (Value::Float(x), Value::Int(y)) => *x == (*y as f64),
            (Value::Float(x), Value::Float(y)) => x == y,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Int(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReservedWord {
    While,
    Do,
    If,
    Else,
    Break,
    True,
    False,
}
impl Display for ReservedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ReservedWord::While => write!(f, "while"),
            ReservedWord::Do => write!(f, "do"),
            ReservedWord::If => write!(f, "if"),
            ReservedWord::Else => write!(f, "else"),
            ReservedWord::Break => write!(f, "break"),
            ReservedWord::True => write!(f, "true"),
            ReservedWord::False => write!(f, "false"),
        }
    }
}

#[cfg(test)]
mod tests;
