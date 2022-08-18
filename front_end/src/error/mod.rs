use crate::lexer::Lexer;
use std::{error::Error, fmt::Display};

pub mod messages;

pub type CompilerError = Box<dyn Error>;

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    message: String,
}

impl SyntaxError {
    pub fn new(str: &str) -> SyntaxError {
        let message = String::from(str);
        SyntaxError { message: message }
    }
    pub fn compiler_error(msg: &str) -> CompilerError {
        let line = Lexer::line();
        let error_msg = format!("Error: near line {}: {}", line, msg.to_string());
        let syntax_error = SyntaxError::new(error_msg.as_str());
        Box::new(syntax_error)
    }
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests;
