//  You don't need to add/edit anything to the below solution. 
//  Click on the SUBMIT button to solve your first problem on CodeChef.

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let n: i32 = line.trim().parse().expect("Failed to parse n as int");

    for _i in 1..n+1 {
        line.clear();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let inputs: Vec<i32> = line.trim().split(" ")
            .map(|x| x.parse().expect("Failed to parse line as int: a, b"))
            .collect();
        println!("{}", inputs[0]+inputs[1]);
    }
}
