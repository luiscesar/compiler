use std::error::Error;

use common::pointer::Pointer;

use crate::lexer::{token::{C_AND, C_OR, C_LT, C_EQ, C_SEMICOLON, C_OPEN_ARRAY, C_CLOSE_ARRAY, Or, Le, Int, Eq, Float, While, True}};

use super::{Lexer, token::{Token, And, Value, ReservedWord}};


#[test]
fn test_lexer_core_new_case1() {
    let file_name = "resources/input1.txt";
    let lexer = Lexer::new(file_name);
}

#[test]
fn test_lexer_core_new_case2() {
    let file_name = "resources/input1a.txt";
    let lexer = Lexer::new(file_name);
    if let Ok(l) = lexer {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn test_lexer_core_readch_case1() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        l.readch();
        let expected_peek = Some('9');
        assert_eq!(l.peek,expected_peek);
    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch_case2() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        l.readch();
        l.readch();
        let expected_peek = Some('-');
        assert_eq!(l.peek,expected_peek);
    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch_case3() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        l.readch();
        l.readch();
        l.readch();
        l.readch();
        l.readch();
        l.readch();
        let expected_peek = None;
        assert_eq!(l.peek,expected_peek);
    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch_case4() {
    let file_name = "resources/input1a.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Err(error) = lexer {
        println!("error {:?}", error);
        assert!(true)
    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch2_case1() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        if let Some(result) = l.readch2('9') {
            assert!(result);
            let expected_peek = Some('9');
            assert_eq!(l.peek,expected_peek);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch2_case2() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        l.readch2('9');
        if let Some(result) = l.readch2('-') {
            assert!(result);
            let expected_peek = '-';
            assert_eq!(l.peek,Some(expected_peek));
        }

    } else {
        assert!(false);
    }
}

#[test]
fn test_lexer_core_readch2_case3() {
    let file_name = "resources/input1.txt";
    let mut lexer = Lexer::new(file_name);
    //input1.txt:9-5+2
    if let Ok(mut l) = lexer {
        if let Some(result) = l.readch2('8') {
            assert!(!result);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

//#[test]
fn test_lexer_line_case1() {
    let line = Lexer::line();
    assert_eq!(line,1);
}

//#[test]
fn test_lexer_line_case2() {
    let line = Lexer::line();
    assert_eq!(line,1);
    Lexer::add_line();
    let line = Lexer::line();
    assert_eq!(line,2);
}

#[test]
fn test_lexer_line_case3() {
    let line = Lexer::line();
    assert_eq!(Lexer::line(),line);
    Lexer::add_line();
    Lexer::add_line();
    Lexer::add_line();
    Lexer::add_line();
    assert_eq!(Lexer::line(),line+4);
}

#[test]
fn test_lexer_core_scan_case1() {
    let file_name = "resources/input41.txt";
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("x".to_string())));
    assert_eq!(lexer.peek,None);
}

#[test]
fn test_lexer_core_scan_case2() {
    let file_name = "resources/input42.txt";
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("x".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("y".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("z".to_string())));
}

#[test]
fn test_lexer_core_scan_case3() {
    let file_name = "resources/input43.txt";
    let mut buffer = String::new();
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Int(10)));
}

#[test]
fn test_lexer_core_scan_case4() {
    let file_name = "resources/input44.txt";
    let mut buffer = String::new();
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Float(101.231)));
}

#[test]
fn test_lexer_core_scan_case5() {
    /*
    && &
    || |
    <= < 
    */
    let file_name = "resources/input45.txt";
    let mut lexer = Lexer::new(file_name).unwrap();

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,And);

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,Token::Char(C_AND));

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,Or);

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,Token::Char(C_OR));

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,Le);

    let token = &*lexer.scan().unwrap();
    assert_eq!(*token,Token::Char(C_LT));

}

#[test]
fn test_lexer_core_scan_case6() {
    let file_name = "resources/input46.txt";
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Int);
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("i".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(C_EQ));
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Int(1)));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(';'));
}

#[test]
fn test_lexer_core_scan_case7() {
    let file_name = "resources/input47.txt";
    //input: float x; float[100] a;
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), Float);
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("x".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(';'));
    assert_eq!(*lexer.scan().unwrap(), Float);
    assert_eq!(*lexer.scan().unwrap(), Token::Char('['));
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Int(100)));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(']'));
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("a".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(';'));
}

#[test]
fn test_lexer_core_scan_case8() {
    let file_name = "resources/input48.txt";
    /*  while( true ) {
            x = 102.330;
            y = 10;
        } 
    */
    let mut lexer = Lexer::new(file_name).unwrap();
    assert_eq!(*lexer.scan().unwrap(), While);
    assert_eq!(*lexer.scan().unwrap(), Token::Char('('));
    assert_eq!(*lexer.scan().unwrap(), True);
    assert_eq!(*lexer.scan().unwrap(), Token::Char(')'));
    assert_eq!(*lexer.scan().unwrap(), Token::Char('{'));
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("x".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(C_EQ));
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Float(102.330)));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(';'));
    assert_eq!(*lexer.scan().unwrap(), Token::Id(Pointer::new_pointer("y".to_string())));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(C_EQ));
    assert_eq!(*lexer.scan().unwrap(), Token::Constant(Value::Int(10)));
    assert_eq!(*lexer.scan().unwrap(), Token::Char(';'));
    assert_eq!(*lexer.scan().unwrap(), Token::Char('}'));
}



