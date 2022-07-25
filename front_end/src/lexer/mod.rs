use std::{collections::HashMap, error::Error, sync::atomic::{AtomicUsize, Ordering}};


use common::{io::fs::myfile::MyFile,pointer::Pointer};

use self::{token::{TokenPtr, If,Else,While,Do,Break,True,False,And,Or,Eq,Ne,Le,Ge,Int,Float,Bool, Token, BLANK_TOKEN, C_AND, C_OR, C_EQ, C_NE, C_LT, C_GT, Value}};


pub mod token;

#[derive(Debug,PartialEq)]
pub struct Lexer {
    peek:Option<char>,
    lookahead:Option<char>,
    reserved_tokens:HashMap<String,TokenPtr>,
    myfile:MyFile,
}

static LINE: AtomicUsize = AtomicUsize::new(1);


impl Lexer {

    pub fn add_line() {
        LINE.fetch_add(1, Ordering::Relaxed);
    }

    pub fn line() -> usize {
        LINE.fetch_add(0, Ordering::Relaxed)
    }

    pub fn new(file_name:&str) -> Result<Lexer,Box<dyn Error>> {
        let my_file = MyFile::new(file_name)?;
        let peek = None;
        let lookahead: Option<char> = None;
        
        let reserved_tokens = Lexer::get_reserved_tokens();
         
        let lexer = Lexer {
            peek:peek,
            lookahead:lookahead,
            reserved_tokens:reserved_tokens,
            myfile:my_file,
        };
        Ok(lexer)
    }

    pub fn scan(&mut self) -> Result<TokenPtr, Box<dyn Error>> {
        self.skip_blanks();
        match self.peek {
            Some(C_AND) => Ok(self.scan_and_token()),
            Some(C_OR) => Ok(self.scan_or_token()),
            Some(C_EQ) => Ok(self.scan_eq_token()),
            Some(C_NE) => Ok(self.scan_ne_token()),
            Some(C_LT) => Ok(self.scan_le_token()),
            Some(C_GT) => Ok(self.scan_ge_token()),
            Some(x) if x.is_digit(10) => Ok(self.scan_numeric_token(x)),
            Some(x) if x.is_alphabetic() => Ok(self.scan_alphanumeric_token(x)),
            Some(x) => Ok(Pointer::new_pointer(Token::Char(x))),
            _ => Ok(Pointer::new_pointer(BLANK_TOKEN)),
        }
    }
 
    fn get_reserved_tokens() -> HashMap<String, TokenPtr> {
        let mut reserved_tokens:HashMap<String,TokenPtr> = HashMap::new();
       
        reserved_tokens.insert(If.to_string(),Pointer::new_pointer(If));
        reserved_tokens.insert(Else.to_string(),Pointer::new_pointer(Else));
        reserved_tokens.insert(While.to_string(),Pointer::new_pointer(While));
        reserved_tokens.insert(Do.to_string(), Pointer::new_pointer(Do));
        reserved_tokens.insert(Break.to_string(), Pointer::new_pointer(Break));
        reserved_tokens.insert(True.to_string(),Pointer::new_pointer(True));
        reserved_tokens.insert(False.to_string(),Pointer::new_pointer(False));
        reserved_tokens.insert(Int.to_string(), Pointer::new_pointer(Int));
        reserved_tokens.insert(Float.to_string(), Pointer::new_pointer(Float));
        reserved_tokens.insert(Bool.to_string(), Pointer::new_pointer(Bool));
        reserved_tokens
    }

    fn readch(&mut self) {
        if self.lookahead == None {
            self.peek = self.myfile.read();
        } else {
            self.peek = self.lookahead;
            self.lookahead = None;
        }
    }

    fn readch2(&mut self, c:char) -> Option<bool> {
        self.readch();
        let t = &self.peek;
        match t {
            Some(x) => 
                {   
                    if x == &c {
                        self.lookahead = None;
                        Some(true)
                    } else {
                        self.lookahead = self.peek;
                        Some(false)
                    }
                },
            None => None,
        }
    }

    fn skip_blanks(&mut self) {
        loop {
            self.readch();
            if (self.peek == Some(' ')) || (self.peek == Some('\t')) || 
                (self.peek == Some('\r')) {
                ;
            } else if self.peek == Some('\n') {
                Lexer::add_line();
            } else {
                break;
            }
        }   
    }

    fn scan_and_token(&mut self) -> TokenPtr {
        let result = self.readch2('&');
        match result {
            Some(is_and) if is_and => Pointer::new_pointer(And),
            _ => Pointer::new_pointer(Token::Char(C_AND)),
        }
    }

    fn scan_or_token(&mut self) -> TokenPtr {
        let result = self.readch2('|');
        match result {
            Some(is_or) if is_or => Pointer::new_pointer(Or),
            _ => Pointer::new_pointer(Token::Char(C_OR)),
        }
    }

    fn scan_eq_token(&mut self) -> TokenPtr {
        let result = self.readch2('=');
        match result {
            Some(is_eq) if is_eq => Pointer::new_pointer(Eq),
            _ => Pointer::new_pointer(Token::Char(C_EQ)),
        }
    }

    fn scan_ne_token(&mut self) -> TokenPtr {
        let result = self.readch2('=');
        match result {
            Some(is_ne) if is_ne => Pointer::new_pointer(Ne),
            _ => Pointer::new_pointer(Token::Char(C_NE)),
        }
    }

    fn scan_le_token(&mut self) -> TokenPtr {
        let result = self.readch2('=');
        match result {
            Some(is_le) if is_le => Pointer::new_pointer(Le),
            _ => Pointer::new_pointer(Token::Char(C_LT)),
        }
    }

    fn scan_ge_token(&mut self) -> TokenPtr {
        let result = self.readch2('=');
        match result {
            Some(is_ge) if is_ge => Pointer::new_pointer(Ge),
            _ => Pointer::new_pointer(Token::Char(C_GT)),
        }
    }

    fn scan_integer(&mut self, x:char, mut v:String) -> String {
        v.push(x);
        loop {
            self.readch();
            if let Some(y) = self.peek {
                if !y.is_digit(10) {
                    self.lookahead = Some(y);
                    break;
                } else {
                    v.push(y);
                }
            } else {
                break;
            }
        };
        v
    }

    fn scan_numeric_token(&mut self,x:char) -> TokenPtr {
        let mut v:String = String::new();
        v = self.scan_integer(x, v);
        if let Some(z) = self.lookahead {
            if z == '.' {
                self.lookahead = None;
                v = self.scan_integer(z, v);
                let w:f64 = v.parse::<f64>().unwrap();
                Pointer::new_pointer(Token::Constant(Value::Float(w)))
            } else {
                let v:i32 = v.parse::<i32>().unwrap();
                Pointer::new_pointer(Token::Constant(Value::Int(v)))
            }
        } else {
            let v:i32 = v.parse::<i32>().unwrap();
            Pointer::new_pointer(Token::Constant(Value::Int(v)))
        } 
    }

    fn scan_alphanumeric_token(&mut self,x:char) -> TokenPtr {
        let mut buffer = String::new();
        buffer.push(x);
        loop {
            self.readch();
            match self.peek {
                None => break,
                Some(y) => {
                    if (y.is_alphanumeric()) {
                        buffer.push(y);
                    } else {
                        self.lookahead = Some(y);
                        break;
                    }
                },
            }
        };
        if let Some(x) = self.reserved_tokens.get(buffer.as_str()) {
            Pointer::clone(x)
        } else {
            Pointer::new_pointer(Token::Id(Pointer::new_pointer(buffer)))
        }
    }

}

#[cfg(test)]
mod tests;
