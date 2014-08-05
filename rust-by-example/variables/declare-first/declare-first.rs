fn main() {
	
	// Declare var
	let a_var;

	{
		let x = 2i;

		// Init the var
		a_var = x * x;		
	}

	println!("a variable: {}", a_var);

	let another_var;
	// This line will not cause an error,
	// vars can be declared first and init later,
	// however they must be init to be called.
	// println!("another var: {}", another_var);

	another_var = 1i;
	println!("another var: {}", another_var);	
}