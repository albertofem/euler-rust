fn main() {
	let fibonacci: Vec<i32> = calculate_fibonacci();

	let mut addition: i32 = 0;

	for &number in fibonacci.iter() {
		if (number % 2 != 0) {
			continue;
		}

		addition += number
	}

	println!("Total result: {}", addition);
}

fn calculate_fibonacci() -> Vec<i32> {
	let mut fibonacci: Vec<i32> = Vec::new();

	for x in 1..4000000 {
		if (x == 1) {
			fibonacci.push(x);
		} else {
			let position: i32 = x-2;
			let last_term: i32 = fibonacci[position as usize];
			fibonacci.push(x + last_term);
		}
	}

	return fibonacci;
}
