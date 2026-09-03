// RUST is a statically typed language, but also infers types when possible

// Primitive Types ...

// Unsigned integers: u8, u16, u32, u64, u128
// Signed integers: i8, i16, i32, i64, i128
// Floating point: f32, f64
// Boolean: bool
// Character: char

// Tuples, Array, Slices, Vectors, Strings, etc. are also available

fn main() {
  // NUMBERS

  // inferred as i32
  let a = 10;
  println!("a: {} is a simple integer", a);

  // inferred as f64
  let b = 2.5;
  println!("b: {} is a float", b);

  // add explicit type
  let c: i64 = 986214765847;
  println!("c: {} is a long integer", c);

  // It is possible to declare a variable, and assign later
  // But it cannot be used before assignment
  let seconds: u32;
  seconds = 60 * 60 * 24;
  println!("Seconds in a day: {seconds} is an unsigned integer");

  // Use underscores for readability of large integers
  let z: i64 = -697_354_123_456_874;
  println!("z: {z} is a large integer with underscores for readability");

  // get the max size of types
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // BOOLEAN

  let is_active: bool = true;
  println!("is_active: {} is a boolean", is_active);

  // infer from expression
  let is_odd = a % 2 == 1;
  println!("is_odd: {} is a boolean", is_odd);
}
