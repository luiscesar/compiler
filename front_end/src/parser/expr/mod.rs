use std::{fmt::{Display, self}, rc::Rc};
use common::{pointer::Pointer, collections::string::StringPtr};
use crate::error::CompilerError;

use self::{arith::{ArithOperation, op::{ArithBinaryOperation,ArithBinaryOperator, ArithUnaryOperation, ArithUnaryOperator}}, types::{Typed, Type, BasicType}, id::{Id, IdPtr}, constant::{Constant, ConstantPtr}, logical::{LogicalOperation, op::{LogicalBinaryOperation, LogicalBinaryOperator, LogicalUnaryOperation, LogicalUnaryOperator}}, relational::{RelationalOperationPtr, RelationalOperator, RelationalOperation}, access::{AccessPtr, Access}};

pub mod types;
pub mod id;
pub mod constant;
pub mod arith;
pub mod logical;
pub mod relational;
pub mod access;

pub trait ExprT:Typed+Display {
    fn reduce(&self) -> ExprPtr;
}

pub type ExprPtr = Rc<Expr>;
#[derive(Debug,PartialEq)]
pub enum Expr {
    CONST(ConstantPtr),
    ID(IdPtr),
    ARITH(ArithOperation),
    LOGICAL(LogicalOperation),
    RELATIONAL(RelationalOperationPtr),
    ACCESS(AccessPtr),
}
impl Expr {
    pub fn new_id_expr_ptr(name:String,id_type:Type) -> ExprPtr {
        let id = Id::new(Pointer::new_pointer(name),id_type);
        let id_ptr = Pointer::new_pointer(id);
        let id_expr = Expr::ID(id_ptr);
        Pointer::new_pointer(id_expr)
    }
    pub fn new_constant_expr_ptr(constant:Constant) -> ExprPtr {
        let constant_ptr = Pointer::new_pointer(constant);
        let constant_expr = Expr::CONST(constant_ptr);
        Pointer::new_pointer(constant_expr)
    }
    pub fn new_arith_binary_operation_expr_ptr(
            operator:ArithBinaryOperator,left:&ExprPtr,right:&ExprPtr)
         -> Result<ExprPtr,CompilerError> {
            let operation_ptr = ArithBinaryOperation::new_ptr(operator, left, right)?;
            let expr = Expr::ARITH(ArithOperation::BINARY(operation_ptr));
            let expr_ptr = Pointer::new_pointer(expr);
            Ok(expr_ptr)
    }
    pub fn new_arith_unary_operation_expr_ptr(
        operator:ArithUnaryOperator,expr:&ExprPtr) -> Result<ExprPtr,CompilerError> {
        let operation_ptr = 
            ArithUnaryOperation::new_ptr(operator,expr)?;
        let expr = Expr::ARITH(ArithOperation::UNARY(operation_ptr));
        let expr_ptr = Pointer::new_pointer(expr);
        Ok(expr_ptr)
    }

    pub fn new_logical_binary_expr_ptr(
        operator:LogicalBinaryOperator, 
        left:&ExprPtr,
        right:&ExprPtr) -> Result<ExprPtr,CompilerError> {
        let operation_ptr = 
            LogicalBinaryOperation::new_ptr(operator,left,right)?;
        let expr = Expr::LOGICAL(LogicalOperation::BINARY(operation_ptr));
        let expr_ptr = Pointer::new_pointer(expr);
        Ok(expr_ptr)
    }

    pub fn new_logical_unary_expr_ptr(
        operator:LogicalUnaryOperator, 
        expr:&ExprPtr) -> Result<ExprPtr,CompilerError> {
        let operation_ptr = 
            LogicalUnaryOperation::new_ptr(operator,expr)?;
        let expr = Expr::LOGICAL(LogicalOperation::UNARY(operation_ptr));
        let expr_ptr = Pointer::new_pointer(expr);
        Ok(expr_ptr)
    }

    pub fn new_relational_expr_ptr(operator:RelationalOperator,
        left:&ExprPtr,
        right:&ExprPtr) -> Result<ExprPtr,CompilerError> {
        let operation_ptr = 
            RelationalOperation::new_pointer(operator,left,right)?;
        let expr = Expr::RELATIONAL(operation_ptr);
        let expr_ptr = Pointer::new_pointer(expr);
        Ok(expr_ptr)
    }

    pub fn new_access_expr_ptr(array_name:&StringPtr,array_type:BasicType,index:&ExprPtr) 
        -> Result<ExprPtr,CompilerError> {
            let access_ptr = Access::new_ptr(array_name,array_type,index)?;
            let access_expr = Expr::ACCESS(access_ptr);
            Ok(Pointer::new_pointer(access_expr))
    }
}
impl ExprT for Expr {
    fn reduce(&self) -> ExprPtr {
        match &self {
            Expr::CONST(x) => x.reduce(),
            Expr::ID(x) => x.reduce(),
            Expr::ARITH(x) => x.reduce(),
            Expr::LOGICAL(x) => x.reduce(),
            Expr::RELATIONAL(x) => x.reduce(),
            Expr::ACCESS(x) => x.reduce(),
        }
    }
}
impl Typed for Expr {
    fn element_type(&self) -> Type {
        match &self {
            Expr::CONST(x) => x.element_type(),
            Expr::ID(x) => x.element_type(),
            Expr::ARITH(x) => x.element_type(),
            Expr::LOGICAL(x) => x.element_type(),
            Expr::RELATIONAL(x) => x.element_type(),
            Expr::ACCESS(x) => x.element_type(),
        }
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Expr::CONST(x) => x.fmt(f),
            Expr::ID(x) => x.fmt(f),
            Expr::ARITH(x) => x.fmt(f),
            Expr::LOGICAL(x) => x.fmt(f),
            Expr::RELATIONAL(x) => x.fmt(f),
            Expr::ACCESS(x) => x.fmt(f),
        }
    }
}
impl ExprT for ExprPtr {
    fn reduce(&self) -> ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for ExprPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}



#[cfg(test)]
mod tests;