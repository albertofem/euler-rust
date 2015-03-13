fn main() {
	let mut multiple = 1;

	for i in 2..20 {
		multiple *= i / gcd(i, multiple);
	}

	println!("Least common multiple: {}", multiple);
}


fn gcd(a: i32, b: i32) -> i32 {
	if b == 0 {
		return a;
	}

	gcd(b, a % b)
}