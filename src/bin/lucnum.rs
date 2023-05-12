use std::io::{stdin, BufRead};

fn is_prime(primes: &mut Vec<usize>, num: usize) -> bool {
    if primes.contains(&num) {
        return true;
    } else if num < primes[primes.len() - 1] {
        return false;
    } else {
        let mut has_prime_divisor = false;
        for i in 1..primes.len() {
            if num % primes[i] == 0 {
                println!("Divisor: {}", primes[i]);
                has_prime_divisor = true;
                break;
            }
        }
        if has_prime_divisor {
            return false;
        }

        // start looking for prime numbers till we reach one
        // greater than num, if we do means, this number is not
        // prime
        let mut last_prime = primes[primes.len() - 1];
        while last_prime < num {
            last_prime += 1;
            let mut divisible = false;
            for i in 1..primes.len() {
                if last_prime % primes[i] == 0 {
                    divisible = true;
                    break;
                }
            }

            if divisible == false && last_prime == num {
                primes.push(last_prime);
                return true;
            } else if divisible == false {
                primes.push(last_prime);
            }
        }
        return false;
    }
}

fn find_p(mut num: usize) -> Result<usize, String> {
    let mut p_is = 0;

    println!("num_start: {num}");
    while num % 2 == 0 && num > 1 {
        p_is += 1;
        num /= 2;
        //println!("START: {num}");
    }
    if num != 1 {
        return Err("Has no 2^x".to_string());
    }

    println!("p_is: {p_is}");
    return Ok(p_is);
}

fn get_p(primes: &mut Vec<usize>, num: usize) -> bool {
    println!("DATA_IN: {num}, prmies: {:?}", primes);
    for i in 1..primes.len() {
        if num % primes[i] == 0 {
            if let Ok(x) = find_p(num / primes[i]) {
                if x % 2 == 0 {
                    return true;
                }
            }
        }
    }
    false
}

pub fn main() {
    let mut primes = vec![1, 2, 3, 5, 7, 11, 13];
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();

    let tests: usize = line.trim().parse().unwrap();
    for _ in 0..tests {
        line.clear();
        stdin().lock().read_line(&mut line).unwrap();
        let num: usize = line.trim().parse().unwrap();
        if num == 2 {
            //2^1 means p isn't even
            println!("0");
            continue;
        }

        if is_prime(&mut primes, num) {
            println!("1");
            continue;
        }

        if get_p(&mut primes, num) {
            println!("1")
        } else {
            println!("0")
        }
    }
}
