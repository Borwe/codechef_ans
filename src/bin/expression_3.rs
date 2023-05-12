use std::io::{stdin, BufRead};

fn positive_or_negative(num: i128, i: u32) -> i128 {
    match i {
        0 => num,
        1 => num * -1,
        _ => panic!("Shouldn't reach here"),
    }
}

fn doable(digits: Vec<i128>) -> bool {
    for i in 0..2 {
        let s1 = positive_or_negative(digits[0], i);
        for j in 0..2 {
            let s2 = positive_or_negative(digits[1], j);
            for k in 0..2 {
                let s3 = positive_or_negative(digits[2], k);
                if s2 + s3 + s1 == 0 {
                    return true;
                }
            }
        }
    }
    false
}

pub fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let digits: Vec<i128> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        if doable(digits) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
