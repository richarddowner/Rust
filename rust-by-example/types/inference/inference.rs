fn main() {
	
	// Using local inference, the compiler knows that `elem` has type u8
	let elem = 5u8;

	// Create an empty vector (a growable array)
	let mut vec = Vec::new();
	// At this poin the compiler doesn't know the exact type of `vec`, it
	// just knows that it's a vector of something (`Vec<_>`)

	// Insert `elem` in the vector
	vec.push(elem);
	// Aha! Now the comiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

	println!("{}", vec);
}