fn main() {
	
	print!("Jan has ");

	// the i indicates signed int
	println!("{} days", 31i); 

	// positional args can be reused
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	// named args
	println!("{subject} {verb} {predicate}",
			predicate="over the lazy dog",
			subject="the quick brown fox",
			verb="jumps");

	// special formatting
	println!("{} of {:t} people know binary, the other half don't", 1i, 2i);
}