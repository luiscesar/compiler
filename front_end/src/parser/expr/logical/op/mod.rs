use crate::{
    error::{messages::LOGICAL_OPERATION_BOOLEANS_EXPECTED, CompilerError, SyntaxError},
    parser::expr::{
        id::Id,
        types::{BasicType, Type, Typed, BOOL},
        Expr, ExprPtr, ExprT,
    },
};
use common::pointer::Pointer;
use std::{
    fmt::{self, Display},
    rc::Rc,
};

pub const OR: LogicalBinaryOperator = LogicalBinaryOperator::OR;
pub const AND: LogicalBinaryOperator = LogicalBinaryOperator::AND;

#[derive(Debug, PartialEq)]
pub enum LogicalBinaryOperator {
    OR,
    AND,
}
impl Display for LogicalBinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LogicalBinaryOperator::OR => write!(f, "||"),
            LogicalBinaryOperator::AND => write!(f, "&&"),
        }
    }
}

pub type LogicalBinaryOperationPtr = Rc<LogicalBinaryOperation>;
#[derive(Debug, PartialEq)]
pub struct LogicalBinaryOperation {
    operator: LogicalBinaryOperator,
    left: ExprPtr,
    right: ExprPtr,
}

impl LogicalBinaryOperation {
    pub fn new(
        operator: LogicalBinaryOperator,
        left: &ExprPtr,
        right: &ExprPtr,
    ) -> Result<LogicalBinaryOperation, CompilerError> {
        let left_type = left.as_ref().element_type();
        let right_type = right.as_ref().element_type();
        if (left_type == BOOL) && (right_type == BOOL) {
            let left_expr = Pointer::clone(left);
            let right_expr = Pointer::clone(right);
            Ok(LogicalBinaryOperation {
                operator: operator,
                left: left_expr,
                right: right_expr,
            })
        } else {
            Err(SyntaxError::compiler_error(
                LOGICAL_OPERATION_BOOLEANS_EXPECTED,
            ))
        }
    }
    pub fn new_ptr(
        operator: LogicalBinaryOperator,
        left: &ExprPtr,
        right: &ExprPtr,
    ) -> Result<LogicalBinaryOperationPtr, CompilerError> {
        let result = LogicalBinaryOperation::new(operator, left, right)?;
        Ok(Pointer::new_pointer(result))
    }
}
impl ExprT for LogicalBinaryOperation {
    fn reduce(&self) -> ExprPtr {
        let e1 = self.left.reduce();
        let e2 = self.right.reduce();
        let op = self.operator.to_string();

        let t = Id::new_id_tmp(self.element_type());

        println!(
            "{} = {} {} {}",
            t.to_string(),
            e1.to_string(),
            op,
            e2.to_string()
        );
        let p = Pointer::new_pointer(t);
        Pointer::new_pointer(Expr::ID(p))
    }
}
impl Typed for LogicalBinaryOperation {
    fn element_type(&self) -> Type {
        BOOL
    }
}
impl Display for LogicalBinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left = self.left.to_string();
        let right = self.right.to_string();
        let op = self.operator.to_string();
        write!(f, "{} {} {}", left, op, right)
    }
}

impl ExprT for LogicalBinaryOperationPtr {
    fn reduce(&self) -> ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for LogicalBinaryOperationPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalUnaryOperator {
    NOT,
}
impl Display for LogicalUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LogicalUnaryOperator::NOT => write!(f, "!"),
        }
    }
}

pub type LogicalUnaryOperationPtr = Rc<LogicalUnaryOperation>;
#[derive(Debug, PartialEq)]
pub struct LogicalUnaryOperation {
    operator: LogicalUnaryOperator,
    expr: ExprPtr,
}
impl LogicalUnaryOperation {
    pub fn new(
        operator: LogicalUnaryOperator,
        expr: &ExprPtr,
    ) -> Result<LogicalUnaryOperation, CompilerError> {
        let expr_type = expr.as_ref().element_type();
        if expr_type == BOOL {
            Ok(LogicalUnaryOperation {
                operator: operator,
                expr: Pointer::clone(expr),
            })
        } else {
            Err(SyntaxError::compiler_error(
                LOGICAL_OPERATION_BOOLEANS_EXPECTED,
            ))
        }
    }
    pub fn new_ptr(
        operator: LogicalUnaryOperator,
        expr: &ExprPtr,
    ) -> Result<LogicalUnaryOperationPtr, CompilerError> {
        let booleanUnaryOperation = LogicalUnaryOperation::new(operator, expr)?;
        Ok(Pointer::new_pointer(booleanUnaryOperation))
    }
}
impl ExprT for LogicalUnaryOperation {
    fn reduce(&self) -> ExprPtr {
        let e = self.expr.reduce();
        let op = self.operator.to_string();

        let t = Id::new_id_tmp(self.element_type());

        println!("{} = {}{}", t.to_string(), op, e.to_string());
        let p = Pointer::new_pointer(t);
        Pointer::new_pointer(Expr::ID(p))
    }
}
impl Typed for LogicalUnaryOperation {
    fn element_type(&self) -> Type {
        Type::Basic(BasicType::Bool)
    }
}
impl Display for LogicalUnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = self.expr.to_string();
        let op = self.operator.to_string();
        write!(f, "{}({})", op, expr)
    }
}
impl ExprT for LogicalUnaryOperationPtr {
    fn reduce(&self) -> ExprPtr {
        self.as_ref().reduce()
    }
}
impl Typed for LogicalUnaryOperationPtr {
    fn element_type(&self) -> Type {
        self.as_ref().element_type()
    }
}

#[cfg(test)]
mod tests;
