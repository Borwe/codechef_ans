use std::io::stdin;

fn main(){
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	let num: u32 = line.trim().parse().unwrap();
	if num % 5 == 0 && num % 11 == 0 {
		println!("BOTH");
	}else if (num %5==0 && num%11!=0) || (num %5!=0 && num % 11==0){
		println!("ONE");
	}else{
		println!("NONE");
	}
}
