use std::io::{stdin, BufRead};

pub fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let ins: Vec<f64> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
        if ins[3] != 0.0 {
            println!("{}", ins[0] + ((ins[2] * ins[0]) / 100.0));
        } else {
            println!("{}", ins[0] - ((ins[1] * ins[0]) / 100.0));
        }
    }
}
