use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct MyFile {
    reader:BufReader<File>,
    name:String,
}

impl MyFile {
    pub fn new(filename:&str) -> Result<MyFile,Box<dyn Error>> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);
        Ok(MyFile{reader:reader,name:String::from(filename)})
    } 

    pub fn read(&mut self) -> Option<char> {
        let mut buffer = [0;1];
        
        let x = self.reader.read(&mut buffer[..]);
        match x {
            Ok(c) if c > 0 => Some(buffer[0] as char),
            _ => None,
        }
    }
}

impl PartialEq for MyFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};
    use std::path::Path;
    use std::process;
    use super::MyFile;

    #[test]
    fn test_myfile_new_case1() {
        let name = "resources/poem.txt";
        let my_file = MyFile::new(name);
        assert!(match my_file {
            Ok(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_myfile_new_case2() {
        let name = "resources/poem3.txt";
        let my_file = MyFile::new(name);
        assert!(match my_file {
            Err(_) => true,
            _ => false,
        });    
    }

    #[test]
    fn test_myfile_read_case1() {
        let name = "resources/poem.txt";
        let f = MyFile::new(name);
        if let Ok(mut my_file) = f {
            loop {
                let c = my_file.read();
                println!("char: {:?}", c);
                if c==None {
                    break;
                }
            }
        }
    }

    #[test]
    fn test_myfile_read_case2() {
        let name = "resources/poem2.txt";
        let f = MyFile::new(name);
        let mut result = String::new();
        if let Ok(mut my_file) = f {
            loop {
                if let Some(c) = my_file.read() {
                    result.push(c);
                } else {
                    break;
                }
            }
        }
        assert_eq!(result,String::from("aeiou"));
    }

    #[test]
    fn test_myfile_read_case21() {
        let name = "resources/poem21.txt";
        let f = MyFile::new(name);
        let mut result = String::new();
        if let Ok(mut my_file) = f {
            loop {
                if let Some(c) = my_file.read() {
                    result.push(c);
                } else {
                    break;
                }
            }
        }
        assert_eq!(result,String::from("aeiou\r\n"));
    }

    #[test]
    fn test_myfile_read_case3() {
        let name = "resources/poem3.txt";
        let f = MyFile::new(name);
        let os_error = std::io::Error::last_os_error();
        println!("os_error {:?}", os_error.to_string());
        if let Err(x) = f {
            println!("x {:?}", x.to_string());
            assert_eq!(x.to_string(),os_error.to_string());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_myfile_fs_read_case1() {
        let filename = "resources/poem.txt";
        let contents = 
            fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents);
    }

    #[test]
    fn test_myfile_fs_read_case2() {
        let filename = "resources/poem.txt";
        let contents = fs::read_to_string(filename);

        println!("result {:?}", contents);
    }

    #[test]
    fn test_myfile_fs_read_case3() {
        let filename = "resources/poem2.txt";
        let contents = fs::read_to_string(filename);

        println!("result {:?}", contents);
    }

    #[test]
    fn test_myfile_fs_read_case4() {
        let filename = "resources/poem.txt";
        let contents = fs::read_to_string(filename);
        match contents {
            Ok(x) => {println!("OK \n {}", x)},
            Err(x) => {println!("Err \n {}", x)},
        }
    }

    #[test]
    fn test_myfile_fs_read_case5() {
        let filename = "resources/poem2.txt";
        let contents = fs::read_to_string(filename);
        match contents {
            Ok(x) => {println!("OK \n {}", x)},
            Err(x) => {println!("Err \n {}", x)},
        }
    }

    #[test]
    fn test_myfile_fs_read_case6() {
        let filename = "resources/poem.txt";
        let contents = fs::read_to_string(filename);
        match contents {
            Ok(x) => {show_contents_lines(x);},
            Err(x) => {println!("Err \n {}", x)},
        }
    }

    #[test]
    fn test_myfile_fs_read_case7() {
        let filename = "resources/poem.txt";
        let contents = fs::read_to_string(filename);
        match contents {
            Ok(x) => {show_contents_lines(x);},
            Err(x) => {println!("Err \n {}", x)},
        }
    }

    #[test]
    fn test_myfile_fs_read_case8_0() {
        let filename = "resources/poem.txt";
        if let Err(x) = show_contents_lines2(filename) {
            eprintln!("{}", x);
            eprintln!("===============================");
            process::exit(1);
        }
    }

    #[test]
    fn test_myfile_fs_read_case8_1() {
        let filename = "resources/poem2.txt";
        if let Err(x) = show_contents_lines2(filename) {
            eprintln!("{}", x);
            eprintln!("===============================");
            //process::exit(1);
        }
    }

    #[test]
    fn test_myfile_fs_read_case9() {
        let filename = "resources/poem.txt";
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("LINE: {}", ip);
                }
            }
        }        
    }

    #[test]
    fn test_myfile_fs_read_case10() {
        let filename = "resources/poem2.txt";
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("LINE: {}", ip);
                    ip.as_bytes().iter().for_each(|b| println!("{}",b));
                }
            }
        }        
    }

    #[test]
    fn test_myfile_fs_read_case11() {
        let filename = "resources/poem.txt";
        if let Err(x) = process_file(filename) {
            eprintln!("{:?}",x);
        }
    }

    #[test]
    fn test_myfile_fs_read_case12() {
        let filename = "resources/poem2.txt";
        if let Err(x) = process_file(filename) {
            eprintln!("{:?}",x);
        }
    }

    fn process_file(filename:&str) -> Result<(),Box<dyn Error>> {
        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
    
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;
            println!("====================================");
            println!("LINE: {}",line);
            println!("Line is {} bytes long", len);
            println!("Line is also {} bytes long", line.len());
            println!("====================================");
            if len == 0 {
                break;
            }
        }
        Ok(())
    }

    fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>> 
        where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufRead::lines(io::BufReader::new(file)))
    }

    fn show_contents_lines(contents:String) {
        for line in contents.lines() {
            println!("LINE: {}", line);
            for word in line.split_ascii_whitespace() {
                println!("WORD: {}", word);
            }
        }
    }

    fn show_contents_lines2(filename:&str) -> Result<(),Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;
        contents.lines().for_each(|line| println!("LINE: {}", line));
        Ok(())
    }

}