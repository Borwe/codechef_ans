use std::io::{self, BufRead, StdinLock};

pub fn main(){
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    println!("{}",line);
}
