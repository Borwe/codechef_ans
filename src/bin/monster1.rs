use std::io::{stdin, BufRead};

pub fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let hits: Vec<usize> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        if hits[1] > hits[2] {
            println!("1");
        } else {
            println!("0");
        }
    }
}
