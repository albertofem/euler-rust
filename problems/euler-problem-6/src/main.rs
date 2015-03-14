fn main() {
	let sum = sum(100);
	let sum_squares = sum_squares(100);

	println!("Difference: {}", sum*sum - sum_squares);
}

fn sum(n: i32) -> i32 {
	(n * (n + 1) / 2)
}

fn sum_squares(n: i32) -> i32 {
	((n * (n+1)*(2*n+1))/6)
}
