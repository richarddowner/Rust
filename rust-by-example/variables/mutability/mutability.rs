fn main() {
	
	let _immutable_variable = 1i;
	let mut mutable_variable = 1i;

	println!("Before mutation: {}", mutable_variable);

	// Ok
	mutable_variable += 1;
	println!("After mutation: {}", mutable_variable);

	// Compile error
	// _immutable_variable += 1;
}