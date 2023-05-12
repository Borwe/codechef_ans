use std::io::stdin;

pub fn main(){
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	let num :u32= line.parse().unwrap();
	let mut facts: Vec<u32> = Vec::new();
	for i in 1..num+1{
		if num % i == 0 {
			facts.push(i);
		}
	}
	println!("{}",facts.len());
	facts.iter().for_each(|l|{
		print!("{} ",l);
	})
}
