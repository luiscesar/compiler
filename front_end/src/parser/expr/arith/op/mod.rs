use common::pointer::Pointer;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::{
    error::{
        messages::{ARITH_OPERATION_NUMBERS_EXPECTED, ARITH_OPERATOR_EXPECTED},
        CompilerError, SyntaxError,
    },
    lexer::token::Token,
    parser::expr::{
        id::Id,
        types::{Type, TypePtr, Typed, BOOL, FLOAT},
        Expr, ExprPtr, ExprT,
    },
};

pub type ArithBinaryOperationPtr = Rc<ArithBinaryOperation>;
pub type ArithUnaryOperationPtr = Rc<ArithUnaryOperation>;

#[derive(Debug, PartialEq)]
pub enum ArithBinaryOperator {
    PLUS,
    MINUS,
    MULT,
    DIV,
}
impl ArithBinaryOperator {
    pub fn op_from_token(token: &Token) -> Result<ArithBinaryOperator, CompilerError> {
        match token {
            Token::Char('+') => Ok(ArithBinaryOperator::PLUS),
            Token::Char('-') => Ok(ArithBinaryOperator::MINUS),
            Token::Char('*') => Ok(ArithBinaryOperator::MULT),
            Token::Char('/') => Ok(ArithBinaryOperator::DIV),
            _ => Err(SyntaxError::compiler_error(ARITH_OPERATOR_EXPECTED)),
        }
    }
}
impl Display for ArithBinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ArithBinaryOperator::PLUS => write!(f, "+"),
            ArithBinaryOperator::MINUS => write!(f, "-"),
            ArithBinaryOperator::MULT => write!(f, "*"),
            ArithBinaryOperator::DIV => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArithUnaryOperator {
    MINUS,
}
impl Display for ArithUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ArithUnaryOperator::MINUS => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ArithBinaryOperation {
    op: ArithBinaryOperator,
    left: ExprPtr,
    right: ExprPtr,
}

impl ArithBinaryOperation {
    pub fn new(
        op: ArithBinaryOperator,
        left: &ExprPtr,
        right: &ExprPtr,
    ) -> Result<ArithBinaryOperation, CompilerError> {
        let left_type = left.as_ref().element_type();
        let right_type = right.as_ref().element_type();
        if (left_type != BOOL) && (right_type != BOOL) {
            let left_expr = Pointer::clone(left);
            let right_expr = Pointer::clone(right);
            Ok(ArithBinaryOperation {
                op: op,
                left: left_expr,
                right: right_expr,
            })
        } else {
            Err(SyntaxError::compiler_error(
                ARITH_OPERATION_NUMBERS_EXPECTED,
            ))
        }
    }
    pub fn new_ptr(
        op: ArithBinaryOperator,
        left: &ExprPtr,
        right: &ExprPtr,
    ) -> Result<ArithBinaryOperationPtr, CompilerError> {
        let binary_operation = ArithBinaryOperation::new(op, left, right)?;
        Ok(Pointer::new_pointer(binary_operation))
    }
}

impl ExprT for ArithBinaryOperation {
    fn reduce(&self) -> ExprPtr {
        let e1 = self.left.reduce();
        let e2 = self.right.reduce();
        let op = self.op.to_string();

        let t = Id::new_id_tmp(self.element_type());

        println!(
            "{} = {} {} {}",
            t.to_string(),
            e1.to_string(),
            op,
            e2.to_string()
        );
        let p = Pointer::new_pointer(t);
        let expr = Expr::ID(p);
        Pointer::new_pointer(expr)
    }
}

impl Typed for ArithBinaryOperation {
    fn element_type(&self) -> Type {
        let left_type = self.left.element_type();
        let right_type = self.right.element_type();
        if left_type == right_type {
            left_type
        } else {
            FLOAT
        }
    }
}

impl Display for ArithBinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left = self.left.to_string();
        let right = self.right.to_string();
        let op = self.op.to_string();
        write!(f, "{} {} {}", left, op, right)
    }
}

#[derive(Debug, PartialEq)]
pub struct ArithUnaryOperation {
    op: ArithUnaryOperator,
    expr: ExprPtr,
}
impl ArithUnaryOperation {
    pub fn new(
        op: ArithUnaryOperator,
        expr: &ExprPtr,
    ) -> Result<ArithUnaryOperation, CompilerError> {
        let expr_type = expr.as_ref().element_type();
        if expr_type != BOOL {
            let expr_clone = Pointer::clone(expr);
            Ok(ArithUnaryOperation {
                op: op,
                expr: expr_clone,
            })
        } else {
            Err(SyntaxError::compiler_error(
                ARITH_OPERATION_NUMBERS_EXPECTED,
            ))
        }
    }
    pub fn new_ptr(
        op: ArithUnaryOperator,
        expr: &ExprPtr,
    ) -> Result<ArithUnaryOperationPtr, CompilerError> {
        let result = ArithUnaryOperation::new(op, expr)?;
        Ok(Pointer::new_pointer(result))
    }
}

impl Typed for ArithUnaryOperation {
    fn element_type(&self) -> Type {
        let expr_type = self.expr.as_ref().element_type();
        expr_type
    }
}

impl ExprT for ArithUnaryOperation {
    fn reduce(&self) -> ExprPtr {
        let e = self.expr.reduce();
        let op = self.op.to_string();
        let t = Id::new_id_tmp(self.element_type());
        println!("{} = {}({})", t.to_string(), op, e.to_string());
        let p = Pointer::new_pointer(t);
        let expr = Expr::ID(p);
        Pointer::new_pointer(expr)
    }
}

impl Display for ArithUnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = self.expr.to_string();
        let op = self.op.to_string();
        write!(f, "{}({})", op, expr)
    }
}

#[cfg(test)]
mod tests;
