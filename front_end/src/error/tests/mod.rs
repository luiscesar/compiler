use std::error::Error;

use super::SyntaxError;

#[test]
fn test_lexer_syntaxerror_new_case1() {
    let error = SyntaxError::new("Test message");
    println!("error {:?}", error);
}

#[test]
fn test_lexer_syntaxerror_new_case2() {
    let value = compute(1);
    if let Err(x) = value {
        assert_eq!((*x).to_string(), "Test message".to_string());
    } else {
        assert!(false);
    }
}

fn compute(value: i32) -> Result<i32, Box<dyn Error>> {
    Err(Box::new(SyntaxError::new("Test message")))
}
