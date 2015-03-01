fn main() {
	let target: i64 = 600851475143;
	let mut target2: i64 = target;
	let mut x = 2;
	let mut largest_fact: i64 = 0;

	while x * x <= target2 {
		if target2 % x == 0 {
			target2 = target2 / x;
			largest_fact = x;
		} else {
			x += 1;
		}
	}

	if target2 > largest_fact {
		largest_fact = target2;
	}

	println!("Largest factor: {}", largest_fact);
}

#[allow(dead_code)]

// functions to brute force solution, not really feasible

fn brute_force() {
	let factors: Vec<i64> = calculate_factors(600851475143);

	let mut max_factor: i64 = 0;

	for &number in factors.iter() {
		if number > max_factor {
			max_factor = number;
		}
	}

	println!("Max factor: {}", max_factor);
}

fn calculate_factors(number: i64) -> Vec<i64> {
	let max_factor: i64 = number / 2;
	let mut primes: Vec<i64> = Vec::new();

	for x in 2..max_factor {
		if number % x == 0 && is_prime(x) {
			primes.push(x);
		}
	}

	return primes;
}

fn is_prime(number: i64) -> bool {
	let max_factor: i64 = number / 2;

	for x in 2..max_factor {
		if number % x != 0 {
			return true;
		}
	}

	return false;
}