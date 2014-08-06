/*
Signed integers: i8, i16, i32, i64 and int (machine word size)
Unsigned integers: u8, u16, u32, u64 and uint (machine word size)
Floating point: f32, f64
char Unicode scalar values like 'a' (4 bytes each)
bool either true or false
and the unit type (), whose only value is also ()
*/
fn main() {
	
	// Type annotated variable
	let a_float: f64 = 1.0;

	// This is an `int`
	let mut an_int = 5i;

	// Error! The type of a variable can't be changed
	// an_int = true;
}