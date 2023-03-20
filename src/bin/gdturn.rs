use std::io::{stdin, BufRead};

pub fn main(){
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();
    let tests: u32 = line.trim().parse().unwrap();
    (0..tests).for_each(|_|{
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let mut split = line.trim().split(" ");
        let x: u8 = split.next().unwrap().parse().unwrap();
        let y: u8 = split.next().unwrap().parse().unwrap();
        if x+y > 6 {
            println!("YES");
        }else{
            println!("NO");
        }
    })
}
