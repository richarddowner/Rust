// Functions are declared using the fn keyword. Its arguments are type
// annotated, just like variables; and, if the function returns a value
// the return type must be specified after an arrow ->

// The final expression in the function will be used as return value.
// Alternatively, the return statement can be used to return a value
// earlier from within the function, even from inside of loops or ifs.

// Function that returns a boolean value
fn is_divisible_by(lhs: uint, rhs: uint) -> bool {
	// Corner case, early return
	if rhs == 0 {
		return false;
	}

	// This is an expression, the `return` keyword is not needed here
	lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: uint) -> () {
	if is_divisible_by(n, 15) {
		println!("fizzbuzz");
	} else if is_divisible_by(n, 3) {
		println!("fizz");
	} else if is_divisible_by(n, 5) {
		println!("buzz");
	} else {
		println!("{}", n);
	}
}

// When a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: uint) {
	for n in range(1, n + 1) {
		fizzbuzz(n);
	}
}

fn main() {
	fizzbuzz_to(100);
}
