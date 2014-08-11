// The type statement can be used to give a new name to an existing type.
// Types must have CamelCase names, or the compiler will raise a warning.
// The exception to this rule are the primitive types: uint, f32 etc.

// The main use of aliases is to reduce typing, for example the IoResult<T> type
// is an alias for the Result<T, IoError> type

// `NanoSecond` is a new name for `u64`
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence warning
#[allow(non_camel_case_types)]
type uint64_t = u64;

fn main() {
	// `NanoSecond` = `Inch` = `uint64_t` = `u64`
	let nanoseconds: NanoSecond = 5 as uint64_t;
	let inches: Inch = 2 as uint64_t;

	// Note that type aliases *don't* provide any extra type safety, because
	// aliases are *not* new types
	println!("{} nanoseconds + {} inches = {} unit?",
			nanoseconds,
			inches,
			nanoseconds + inches);
}


