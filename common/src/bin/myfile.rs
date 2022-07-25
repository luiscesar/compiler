use std::env;
use common::collections::list::myveclist::MyVecList;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("args {:?}", args);
    println!("Searching for {}", query);
    println!("In file {}", filename);
}

#[cfg(test)]
mod tests2 {
    use super::main;
    use std::env;

}