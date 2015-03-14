use std::num::Int;

//  is prime trait, just for Rust's learning sake
trait IsPrime {
	fn is_prime(&self) -> bool;
}

impl IsPrime for i32 {
	fn is_prime(&self) -> bool {
		if self % 2 == 0 {
			return false;
		}

		let mut prime = 3;
		
		while prime < self.pow(1) as i32 +1 {
			if self % prime == 0 {
				return false;
			}
			
			prime = prime+2
		}

		return true;
	}
}

fn main() {
	let mut prime = 2;
	let mut count = 1;
	let mut iter = 3;

	while count < 10001 {
		if iter.is_prime() {
			prime = iter;
			count = count + 1;
		}

		iter = iter + 2;
	}

	println!("10001th prime is: {}", prime);
}
