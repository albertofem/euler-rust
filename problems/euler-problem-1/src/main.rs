fn main() {
	let mut multiples:Vec<i32> = Vec::new();

    for x in 1..1000 {
		if x % 3 == 0 || x % 5 == 0 {
			multiples.push(x);
		}
	}

	let mut addition: i32 = 0;

	for &number in multiples.iter() {
		addition += number;
	}

	println!("Total sum: {}", addition);
}
