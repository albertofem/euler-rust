// brute force solution
fn main() {
	let mut largest_palindrome: i32 = 0;

	for x in 100..999 {
		for y in 100..999 {
			let product: i32 = x*y;
			if is_palindrome(product) && product > largest_palindrome {
				largest_palindrome = product;
			}
		}
	}

	println!("Largest palindrome: {}", largest_palindrome);
}


fn is_palindrome(number :i32) -> bool {
	let number_str: String = number.to_string();
	let reversed_string: String = number_str.as_slice().chars().rev().collect();

	return reversed_string == number_str;
}
