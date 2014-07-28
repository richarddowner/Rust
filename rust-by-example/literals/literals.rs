fn main() {
	
	// Integer addition
	println!("1 + 2 = {}", 1u + 2);

	// Integer subtraction - (i) signed int as the result will be negative
	println!("1 - 2 = {}", 1i - 2);

	// Short-circuiting boolean logic
	println!("true AND false is {}", true && false); // false
	println!("true OR false is {}", true || false); // true
	println!("NOT true is {}", !true); // false

	// Bitwise operations
	println!("0011 AND 0101 is {:04t}", 0b0011u & 0b0101);
	println!("0011 OR 0101 is {:04t}", 0b0011u | 0b0101);
	println!("0011 XOR 0101 is {:04t}", 0b0011u ^ 0b0101);
	println!("1 << 5 is {}", 1u << 5);
	println!("0x80 >> 2 is 0x{:x}", 0x80u >> 2);

	// Use underscores to improve readability!
	println!("One million is written as {}", 1_000_000u);

}