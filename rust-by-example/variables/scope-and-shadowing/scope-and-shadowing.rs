fn main() {
	
	// This var lives in the main func
	let long_lived_variable = 1i;

	// This is a block, smaller scope than main func
	{
		// Only exists in this block
		let short_lived_variable = 2i;
		println!("inner short: {}", short_lived_variable);

		// This var shadows the outer one
		let long_lived_variable = 5_f32;
		println!("inner long: {}", long_lived_variable);
	} 

	// Compile error
	// println!("outer short: {}", short_lived_variable);

	println!("outer long: {}", long_lived_variable);
}