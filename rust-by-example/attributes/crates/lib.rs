// The `crate_type` attribute can be used to tell the compiler whether a crate is a binary or
// a library (and even which type of library). And the `crate_name` attribute can be used to
// set the name of the crate.

// lib.rs
// This crate is a library
#![crate_type = "lib"]
// The libraryis named `erty`
#![crate_name = "erty"]

pub fn public_function() {
	println!("called erty's `public_function()`");
}

fn private_function() {
	println!("called erty's `private_function()`");
}

pub fn indirect_access() {
	print!("called erty's `indirect_access()`, that \n> ");

	private_function();
}
