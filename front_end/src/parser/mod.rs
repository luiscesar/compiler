use common::{collections::string::StringPtr, pointer::Pointer};

use crate::{
    error::{
        messages::{
            ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR, ARRAY_DIMENSION_GREATER_THAN_ZERO,
            BREAK_WITHOUT_LOOP, EXPECTED_TOKEN, EXPRESSION_EXPECTED, IDENTIFIER_NAME_EXPECTED,
            INTEGER_EXPECTED, LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR,
            UNDECLARED_IDENTIFIER,
        },
        CompilerError, SyntaxError,
    },
    lexer::{
        token::{
            And, Break, Do, Else, Eq, False, If, Lt, Ne, Or, ReservedWord, Token, TokenPtr, True,
            Value, While, C_CLOSE_ARRAY, C_GT, C_LT, C_OPEN_ARRAY, C_SEMICOLON,
        },
        Lexer,
    },
    parser::stmt::if_stmt::IfStmt,
};

use self::{
    expr::{
        access::Access,
        arith::op::{ArithBinaryOperator, ArithUnaryOperator},
        constant::Constant,
        id::{Id, IdPtr},
        logical::op::{LogicalUnaryOperator, AND, OR},
        relational::{RelationalOperator, EQ, LT, NE},
        types::{basic_type, basic_type_from_token, BasicType, Type, Typed, BOOL},
        Expr, ExprPtr,
    },
    stmt::{
        break_stmt::BreakStmt,
        do_stmt::DoStmt,
        env::{
            scope::{find, new_scope, new_scope_stack_mut_ptr, pop, push, Scope, ScopeStackMutPtr},
            Env, EnvMutPtr,
        },
        ifelse_stmt::IfElseStmt,
        seq_stmt::SeqStmt,
        set_elem_stmt::SetElemStmt,
        set_stmt::SetStmt,
        while_stmt::WhileStmt,
        Stmt, StmtPtr,
    },
};

pub mod expr;
pub mod stmt;

pub struct Parser {
    lexer: Lexer,
    look: TokenPtr,
    scope_stack_mut_ptr: ScopeStackMutPtr<StringPtr, IdPtr>,
    loops_in_scope: usize,
}

impl Parser {
    pub fn new(file_name: String) -> Result<Parser, CompilerError> {
        let mut lexer = Lexer::new(file_name.as_str())?;
        let look = lexer.scan()?;
        let scope_stack_mut_ptr: ScopeStackMutPtr<StringPtr, IdPtr> = new_scope_stack_mut_ptr();
        let parser = Parser {
            lexer: lexer,
            look: look,
            scope_stack_mut_ptr: scope_stack_mut_ptr,
            loops_in_scope: 0,
        };
        Ok(parser)
    }

    pub fn program(&mut self) -> Result<(), CompilerError> {
        let stmt = self.block()?;
        let env = Env::new_mut_ptr();
        stmt.gen(&env)?;
        Ok(())
    }

    fn block(&mut self) -> Result<StmtPtr, CompilerError> {
        self.match_token(&Token::Char('{'))?;
        let scope = self.decls()?;
        push(scope, &self.scope_stack_mut_ptr);
        let stmt_ptr = self.stmts()?;
        pop(&self.scope_stack_mut_ptr);
        Ok(stmt_ptr)
    }

    fn stmts(&mut self) -> Result<StmtPtr, CompilerError> {
        if *self.look == Token::Char('}') {
            let stmt = Stmt::Null;
            Ok(Pointer::new_pointer(stmt))
        } else {
            let stmt_ptr1 = self.stmt()?;
            let stmt_ptr2 = self.stmts()?;
            let seq_stmt_ptr = SeqStmt::new_ptr(&stmt_ptr1, &stmt_ptr2);
            let stmt = Stmt::Seq(seq_stmt_ptr);
            Ok(Pointer::new_pointer(stmt))
        }
    }

    fn stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        let token = Pointer::clone(&self.look);
        match &*token {
            Token::Char(';') => Ok(self.null_stmt()?),
            Token::Reserved(ReservedWord::If) => Ok(self.if_stmt()?),
            Token::Reserved(ReservedWord::While) => Ok(self.while_stmt()?),
            Token::Reserved(ReservedWord::Do) => Ok(self.do_stmt()?),
            Token::Reserved(ReservedWord::Break) => Ok(self.break_stmt()?),
            Token::Char('{') => Ok(self.block()?),
            _ => Ok(self.assign_stmt()?),
        }
    }

    fn null_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        self.move_scan()?;
        let stmt = Stmt::Null;
        let stmt_ptr = Pointer::new_pointer(stmt);
        Ok(stmt_ptr)
    }

    fn while_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        self.match_token(&While)?;
        self.match_token(&Token::Char('('))?;
        let condition_ptr = self.bool()?;
        self.match_token(&Token::Char(')'))?;

        self.loops_in_scope += 1;
        let stmt1_ptr = self.stmt()?;
        self.loops_in_scope -= 1;

        let while_stmt_ptr = WhileStmt::new_ptr(&condition_ptr, &stmt1_ptr)?;
        let stmt = Stmt::While(while_stmt_ptr);
        Ok(Pointer::new_pointer(stmt))
    }

    fn do_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        self.match_token(&Do)?;

        self.loops_in_scope += 1;
        let stmt1_ptr = self.stmt()?;
        self.loops_in_scope -= 1;

        self.match_token(&While)?;
        self.match_token(&Token::Char('('))?;
        let condition_ptr = self.bool()?;
        self.match_token(&Token::Char(')'))?;
        self.match_token(&Token::Char(';'))?;

        let do_stmt_ptr = DoStmt::new_ptr(&condition_ptr, &stmt1_ptr)?;
        let stmt = Stmt::Do(do_stmt_ptr);
        Ok(Pointer::new_pointer(stmt))
    }

    fn break_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        self.match_token(&Break)?;
        if self.loops_in_scope > 0 {
            self.match_token(&Token::Char(';'))?;
            let break_stmt_ptr = BreakStmt::new_ptr();
            let stmt = Stmt::Break(break_stmt_ptr);
            Ok(Pointer::new_pointer(stmt))
        } else {
            Err(SyntaxError::compiler_error(BREAK_WITHOUT_LOOP))
        }
    }

    fn if_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        self.match_token(&If)?;
        self.match_token(&&Token::Char('('))?;
        let condition = self.bool()?;
        self.match_token(&&Token::Char(')'))?;
        let stmt1_ptr = self.stmt()?;
        if *self.look == Else {
            self.match_token(&Else)?;
            let stmt2_ptr = self.stmt()?;
            let if_else_stmt_ptr = IfElseStmt::new_ptr(&condition, &stmt1_ptr, &stmt2_ptr)?;
            let stmt = Stmt::IfElse(if_else_stmt_ptr);
            Ok(Pointer::new_pointer(stmt))
        } else {
            let if_stmt_ptr = IfStmt::new_ptr(&condition, &stmt1_ptr)?;
            let stmt = Stmt::If(if_stmt_ptr);
            Ok(Pointer::new_pointer(stmt))
        }
    }

    fn assign_stmt(&mut self) -> Result<StmtPtr, CompilerError> {
        let id_name = self.match_id_token()?;
        let id_ref = find(&id_name, &self.scope_stack_mut_ptr);
        if let Some(id) = id_ref {
            match id.element_type() {
                Type::Array(basic_type, dimension) => {
                    self.match_token(&Token::Char('['))?;
                    let index = self.bool()?;
                    let access_ptr = Access::new_ptr(&id_name, basic_type, &index)?;
                    self.match_token(&Token::Char(']'))?;
                    self.match_token(&Token::Char('='))?;

                    let expr_ptr = self.bool()?;

                    let access_stmt_ptr = SetElemStmt::new_ptr(&access_ptr, &expr_ptr)?;

                    let stmt = Stmt::SetElem(access_stmt_ptr);
                    self.match_token(&Token::Char(';'))?;
                    Ok(Pointer::new_pointer(stmt))
                }
                _ => {
                    self.match_token(&Token::Char('='))?;
                    let expr_ptr = self.bool()?;
                    let set_stmt_ptr = SetStmt::new_ptr(&id, &expr_ptr)?;
                    let stmt = Stmt::Set(set_stmt_ptr);
                    self.match_token(&Token::Char(';'))?;
                    Ok(Pointer::new_pointer(stmt))
                }
            }
        } else {
            let msg = format!("{}: {}", UNDECLARED_IDENTIFIER, id_name);
            Err(SyntaxError::compiler_error(msg.as_str()))
        }
    }

    fn move_scan(&mut self) -> Result<(), CompilerError> {
        self.look = self.lexer.scan()?;
        Ok(())
    }

    fn decls(&mut self) -> Result<Scope<StringPtr, IdPtr>, CompilerError> {
        let mut scope: Scope<StringPtr, IdPtr> = new_scope();
        loop {
            let token = Pointer::clone(&self.look);
            match &*token {
                Token::Type(x) => {
                    let id_basic_type = basic_type_from_token(x);
                    self.move_scan()?;
                    let id_type = self.match_type_token(id_basic_type)?;
                    let id_name = self.match_id_token()?;
                    self.match_token(&Token::Char(C_SEMICOLON))?;
                    let id = Id::new_ptr(Pointer::clone(&id_name), id_type);
                    scope.insert(Pointer::clone(&id_name), id);
                }
                _ => break,
            }
        }
        Ok(scope)
    }

    fn match_id_token(&mut self) -> Result<StringPtr, CompilerError> {
        let token = Pointer::clone(&self.look);
        let result = match &*token {
            Token::Id(id_name) => {
                self.move_scan()?;
                Ok(Pointer::clone(&id_name))
            }
            _ => Err(SyntaxError::compiler_error(IDENTIFIER_NAME_EXPECTED)),
        };
        result
    }

    fn match_int_token(&mut self) -> Result<i32, CompilerError> {
        let token = Pointer::clone(&self.look);
        match &*token {
            Token::Constant(Value::Int(x)) => {
                self.move_scan()?;
                Ok(*x)
            }
            _ => Err(SyntaxError::compiler_error(
                ARRAY_DIMENSION_GREATER_THAN_ZERO,
            )),
        }
    }

    fn match_type_token(&mut self, id_basic_type: BasicType) -> Result<Type, CompilerError> {
        let token = Pointer::clone(&self.look);
        match &*token {
            Token::Char(C_OPEN_ARRAY) => {
                self.move_scan()?;
                let size = self.match_int_token()?;
                let size = size as usize;
                self.match_token(&Token::Char(C_CLOSE_ARRAY))?;
                Ok(Type::Array(id_basic_type, size))
            }
            _ => Ok(Type::Basic(id_basic_type)),
        }
    }

    fn match_token(&mut self, expected_token: &Token) -> Result<(), CompilerError> {
        if *self.look == *expected_token {
            self.move_scan()?;
            Ok(())
        } else {
            let err_msg = format!("{}: {}", EXPECTED_TOKEN, expected_token.to_string());
            Err(SyntaxError::compiler_error(err_msg.as_str()))
        }
    }

    fn bool(&mut self) -> Result<ExprPtr, CompilerError> {
        let mut expr_ptr = self.join()?;
        loop {
            if *self.look == Or {
                self.move_scan()?;
                let expr_ptr2 = self.join()?;
                expr_ptr = Expr::new_logical_binary_expr_ptr(OR, &expr_ptr, &expr_ptr2)?;
            } else {
                break;
            }
        }
        Ok(expr_ptr)
    }

    fn join(&mut self) -> Result<ExprPtr, CompilerError> {
        let mut expr_ptr = self.equality()?;
        loop {
            if *self.look == And {
                self.move_scan()?;
                let expr_ptr2 = self.equality()?;
                expr_ptr = Expr::new_logical_binary_expr_ptr(AND, &expr_ptr, &expr_ptr2)?;
            } else {
                break;
            }
        }
        Ok(expr_ptr)
    }

    fn equality(&mut self) -> Result<ExprPtr, CompilerError> {
        let mut expr_ptr = self.rel()?;
        loop {
            if *self.look == Eq {
                self.move_scan()?;
                let expr_ptr2 = self.rel()?;
                expr_ptr = Expr::new_relational_expr_ptr(EQ, &expr_ptr, &expr_ptr2)?;
            } else if *self.look == Ne {
                self.move_scan()?;
                let expr_ptr2 = self.rel()?;
                expr_ptr = Expr::new_relational_expr_ptr(NE, &expr_ptr, &expr_ptr2)?;
            } else {
                break;
            }
        }
        Ok(expr_ptr)
    }

    fn rel(&mut self) -> Result<ExprPtr, CompilerError> {
        let expr_ptr = self.expr()?;
        if (*self.look == Token::Char(C_LT))
            || (*self.look == Token::Char(C_GT))
            || (*self.look == Token::Word("<="))
            || (*self.look == Token::Word(">="))
        {
            let op = RelationalOperator::op_from_token(&self.look)?;
            self.move_scan()?;
            let expr_ptr2 = self.expr()?;
            let rel_expr_ptr = Expr::new_relational_expr_ptr(op, &expr_ptr, &expr_ptr2)?;
            Ok(rel_expr_ptr)
        } else {
            Ok(expr_ptr)
        }
    }

    fn expr(&mut self) -> Result<ExprPtr, CompilerError> {
        let mut expr_ptr = self.term()?;
        loop {
            if (*self.look == Token::Char('+')) || (*self.look == Token::Char('-')) {
                let op = ArithBinaryOperator::op_from_token(&self.look)?;
                self.move_scan()?;
                let expr_ptr2 = self.term()?;
                expr_ptr = Expr::new_arith_binary_operation_expr_ptr(op, &expr_ptr, &expr_ptr2)?;
            } else {
                break;
            }
        }
        Ok(expr_ptr)
    }

    fn term(&mut self) -> Result<ExprPtr, CompilerError> {
        let mut expr_ptr = self.unary()?;
        loop {
            if (*self.look == Token::Char('*')) || (*self.look == Token::Char('/')) {
                let op = ArithBinaryOperator::op_from_token(&self.look)?;
                self.move_scan()?;
                let expr_ptr2 = self.unary()?;
                expr_ptr = Expr::new_arith_binary_operation_expr_ptr(op, &expr_ptr, &expr_ptr2)?;
            } else {
                break;
            }
        }
        Ok(expr_ptr)
    }

    fn unary(&mut self) -> Result<ExprPtr, CompilerError> {
        if *self.look == Token::Char('-') {
            self.move_scan()?;
            let unary_expr_ptr = self.unary()?;
            if unary_expr_ptr.element_type() != BOOL {
                let op = ArithUnaryOperator::MINUS;
                let expr_ptr = Expr::new_arith_unary_operation_expr_ptr(op, &unary_expr_ptr)?;
                Ok(expr_ptr)
            } else {
                Err(SyntaxError::compiler_error(
                    ARITH_EXPR_REQUIRED_UNDER_ARITH_UNARY_OPERATOR,
                ))
            }
        } else if *self.look == Token::Char('!') {
            self.move_scan()?;
            let unary_expr_ptr = self.unary()?;
            if unary_expr_ptr.element_type() == BOOL {
                let op = LogicalUnaryOperator::NOT;
                let expr_ptr = Expr::new_logical_unary_expr_ptr(op, &unary_expr_ptr)?;
                Ok(expr_ptr)
            } else {
                Err(SyntaxError::compiler_error(
                    LOGICAL_EXPR_REQUIRED_UNDER_LOGICAL_UNARY_OPERATOR,
                ))
            }
        } else {
            let expr_ptr = self.factor()?;
            Ok(expr_ptr)
        }
    }

    fn factor(&mut self) -> Result<ExprPtr, CompilerError> {
        let token = Pointer::clone(&self.look);
        match &*token {
            Token::Char('(') => {
                self.move_scan()?;
                let expr_ptr = self.bool()?;
                self.match_token(&Token::Char(')'))?;
                Ok(expr_ptr)
            }
            Token::Constant(Value::Int(x)) => {
                let expr_ptr = Expr::new_constant_expr_ptr(Constant::INT(*x));
                self.move_scan()?;
                Ok(expr_ptr)
            }
            Token::Constant(Value::Float(x)) => {
                let expr_ptr = Expr::new_constant_expr_ptr(Constant::FLOAT(*x));
                self.move_scan()?;
                Ok(expr_ptr)
            }
            &True => {
                let expr_ptr = Expr::new_constant_expr_ptr(Constant::BOOL(true));
                self.move_scan()?;
                Ok(expr_ptr)
            }
            &False => {
                let expr_ptr = Expr::new_constant_expr_ptr(Constant::BOOL(false));
                self.move_scan()?;
                Ok(expr_ptr)
            }
            Token::Id(id_name) => {
                let id_ref = find(id_name, &self.scope_stack_mut_ptr);
                if let Some(id) = id_ref {
                    self.move_scan()?;
                    match id.element_type() {
                        Type::Array(basic_type, dimension) => {
                            self.match_token(&Token::Char('['))?;
                            let index = self.bool()?;
                            self.match_token(&Token::Char(']'))?;
                            let expr_ptr = Expr::new_access_expr_ptr(&id_name, basic_type, &index)?;
                            Ok(expr_ptr)
                        }
                        _ => {
                            let expr = Expr::ID(id);
                            let expr_ptr = Pointer::new_pointer(expr);
                            Ok(expr_ptr)
                        }
                    }
                } else {
                    let msg = format!("{}: {}", UNDECLARED_IDENTIFIER, id_name);
                    Err(SyntaxError::compiler_error(msg.as_str()))
                }
            }
            _ => Err(SyntaxError::compiler_error(EXPRESSION_EXPECTED)),
        }
    }
}
#[cfg(test)]
mod tests;
