// Rust provides a powerful module system that can be used to hierarchically
// split code in logical units (modules), and manage visibility (public/priv)
// between them.

// A module is a collection of items like: functions, structs, traits, impl blocks,
// and even other modules.

fn function() {
	println!("called `function()`");
}

// A module named `my`
mod my {
	// A module can contain items like functions
	#[allow(dead_code)]
	fn function() {
		println!("called `my::function()`");
	}

	// Modules can be nested
	mod nested {
		#[allow(dead_code)]
		fn function() {
			println!("called `my::nested::function()`");
		}
	}
}

fn main() {
	function();

	// items inside a module can be called using their full path
	// the `println` function lives in the stdio module
	// the `stdio` module lives in the `io` module
	// and the `io` module lives in the `std` crate
	std::io::stdio::println("Hello World!");

	// Error! `my::function` is private
	// my::function();
}
