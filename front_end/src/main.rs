use std::{env, process};

use front_end::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: <command> <file>");
        process::exit(1);
    } else {
        let file_name = args.get(1).unwrap();
        let parser_result = 
            Parser::new(file_name.to_string());
        match parser_result {
            Ok(mut parser) => {
                let program_result = parser.program();
                match &program_result {
                    Ok(()) => {},
                    Err(error_ptr) => {
                        eprintln!("{}",error_ptr.to_string());
                        process::exit(1);
                    }
                }
            },
            Err(error_ptr) => {
                eprintln!("{}",error_ptr.to_string());
                process::exit(1);
            }
        }
    }
}