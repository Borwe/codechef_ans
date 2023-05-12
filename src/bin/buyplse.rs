use std::{io, io::BufRead};

pub fn main(){
	let mut line = String::new();
	io::stdin().lock().read_line(&mut line).unwrap();
	let split: Vec<u32> = line.trim().split(" ").map(|s| s.parse().unwrap()).collect();
	let output: u32 = (split.get(0).unwrap() * split.get(2).unwrap())+
		(split.get(1).unwrap() * split.get(3).unwrap());
	println!("{output}");
}
