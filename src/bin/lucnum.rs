use std::io::{stdin, BufRead};

fn is_p_even(mut num: usize) -> bool {
    let mut ps = 0;
    while num % 2 == 0 {
        ps = ps + 1;
        num = num / 2;
    }

    if ps % 2 == 0 {
        true
    } else {
        false
    }
}

pub fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let num: usize = line.trim().parse().unwrap();
        if is_p_even(num) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
