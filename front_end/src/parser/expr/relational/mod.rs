use std::{fmt::Display, rc::Rc};
use common::pointer::Pointer;

use crate::{error::{CompilerError, SyntaxError, messages::{RELATIONAL_OPERATOR_EXPECTED, RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING}}, parser::expr::{ExprPtr}, lexer::token::{Lt, Le, Ne, Gt, Ge, Eq, Token}};
use super::{types::{Typed, BOOL, Type}, ExprT, Expr};

pub const LT:RelationalOperator = RelationalOperator::LT;
pub const LE:RelationalOperator = RelationalOperator::LE;
pub const EQ:RelationalOperator = RelationalOperator::EQ;
pub const NE:RelationalOperator = RelationalOperator::NE;
pub const GT:RelationalOperator = RelationalOperator::GT;
pub const GE:RelationalOperator = RelationalOperator::GE;

#[derive(Debug,PartialEq,Copy,Clone)]
pub enum RelationalOperator {
    LT,LE,EQ,NE,GT,GE
}
impl RelationalOperator {
    pub fn op_from_token(token:&Token) -> Result<RelationalOperator,CompilerError> {
        match token {
            Token::Char('<') => Ok(LT),
            Token::Char('>') => Ok(GT),
            Token::Word("<=") => Ok(LE),
            Token::Word(">=") => Ok(GE),
            Token::Word("==") => Ok(EQ),
            Token::Word("!=") => Ok(NE),
            _ => {
                Err(SyntaxError::compiler_error(RELATIONAL_OPERATOR_EXPECTED))
            }
        }
    }
}
impl Display for RelationalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RelationalOperator::LT => write!(f, "{}", Lt),
            RelationalOperator::LE => write!(f, "{}", Le),
            RelationalOperator::EQ => write!(f, "{}", Eq),
            RelationalOperator::NE => write!(f, "{}", Ne),
            RelationalOperator::GT => write!(f, "{}", Gt),
            RelationalOperator::GE => write!(f, "{}", Ge),
        }
    }
}

pub type RelationalOperationPtr = Rc<RelationalOperation>;
#[derive(Debug,PartialEq)]
pub struct RelationalOperation {
    operator:RelationalOperator,
    left:ExprPtr,
    right:ExprPtr,
}
impl RelationalOperation {
    pub fn new(operator:RelationalOperator,left:&ExprPtr,right:&ExprPtr) -> 
        Result<RelationalOperation,CompilerError> {
            let left_type = left.as_ref().element_type();
            let right_type = right.as_ref().element_type();
            let relational_condition = 
                (left_type == right_type) || ((left_type!=BOOL) && (right_type!=BOOL));
            if relational_condition {
                let left_expr = Pointer::clone(left);
                let right_expr = Pointer::clone(right);
                Ok(RelationalOperation{operator:operator,left:left_expr,right:right_expr})
            } else {
                Err(SyntaxError::compiler_error(RELATIONAL_OPERATION_VALUES_EXPECTED_SAME_ORDERING))
            }
    }

     pub fn new_pointer(operator:RelationalOperator,left:&ExprPtr,right:&ExprPtr) -> 
        Result<RelationalOperationPtr,CompilerError> {
        let operation = RelationalOperation::new(operator,left,right)?;
        Ok(Pointer::new_pointer(operation))    
    }
}
impl ExprT for RelationalOperation {
    fn reduce(&self) -> ExprPtr {
        let e1 = self.left.reduce();
        let e2 = self.right.reduce();
        let relational_operation = 
            RelationalOperation{operator:self.operator,left:e1,right:e2};
        let expr = Expr::RELATIONAL(Pointer::new_pointer(relational_operation));
        Pointer::new_pointer(expr)
    }
}

impl Typed for RelationalOperation {
    fn element_type(&self) -> Type {
        BOOL
    }
}
impl Display for RelationalOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = self.left.to_string();
        let right = self.right.to_string();
        let op = self.operator.to_string();
        write!(f, "{} {} {}",left,op,right) 
    }
}

impl ExprT for RelationalOperationPtr {
    fn reduce(&self) -> super::ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for RelationalOperationPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}
#[cfg(test)]
mod tests;