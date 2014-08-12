// In Rust, almost every statement is an expression, this means that the statement returns a value.
// This may not always be desired, so the output can be suppresed by ending the expression with a ;

// Blocks are expressions too, so they can be used as r-values in assignments. The last expression
// in the block will eb assigned to the l-value. But, if the last expression of the block ends with
// a semicolon, the return value will be (). 

fn main() {
	let x = 5u;

	let y = {
		let x_squared = x * x;
		let x_cube = x_squared * x;

		// This expression will be assigned to `y`
		x_cube + x_squared + x
	};

	let z = {
		// The semicolon supresses this expression `()` is assigned to `z`
		2 * x;
	};

	println!("x is {}", x);
	println!("y is {}", y);
	println!("z is {}", z);
}
