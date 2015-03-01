fn main() {
	let fibonacci: Vec<i32> = calculate_fibonacci();

	let mut addition: i32 = 0;

	for &number in fibonacci.iter() {
		if number % 2 != 0 {
			continue;
		}

		addition += number
	}

	println!("Total result: {}", addition);
}

fn calculate_fibonacci() -> Vec<i32> {
	let mut fibonacci: Vec<i32> = Vec::new();

	// first 2 terms
	fibonacci.push(0);
	fibonacci.push(1);

	// start in term 3
	let mut x: i32 = 2;

	loop {
		let position: i32 = x-1;
		let position2: i32 = position-1;

		let prev_term: i32 = fibonacci[position as usize];
		let prev_term2: i32 = fibonacci[position2 as usize];

		let fibonacci_term: i32 = prev_term + prev_term2;

		fibonacci.push(fibonacci_term);

		if fibonacci_term >= 4000000 {
			break;
		}

		x += 1;
	}

	return fibonacci;
}
