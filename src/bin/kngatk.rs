use std::io::{stdin, BufRead};

pub fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let digits: usize = line.trim().parse().unwrap();
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let tmps: Vec<usize> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        let mut unordered = Vec::from(&tmps[0..digits]);
        unordered.sort_by(|a, b| a.cmp(b));
        println!("{}", unordered[1]);
    }
}
