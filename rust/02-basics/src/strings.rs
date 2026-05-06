// In rust strings are of 2 types: String and &str

// &str - immutable, fixed-length string slice, stored in the binary
// String - growable, heap-allocated string, mutable

pub fn run() {
  // By default, string literals are of type &str
  let a = "Hello, world!";
  println!("a: {} is a string slice (&str) of length {}", a, a.len());

  // Declare a mutable string using String::from
  let mut hello = String::from("Hello ");
  println!("hello: '{}' is a growable string (String)", hello);
  // add a single character to the string
  hello.push('R');
  // add a string slice to the string
  hello.push_str("ust!");
  println!("{hello}");
  // get total capacity of the string
  println!("Capacity: {}", hello.capacity());
  // check if string contains a substring
  println!("Contains 'Rust': {}", hello.contains("Rust"));

  // Declare string from string literal
  let greet = "Good morning".to_string();
  println!("greet: '{greet}' is a String created from a string literal");
  // replacement
  let replaced = greet.replace("morning", "afternoon");
  println!("Replaced: {replaced}");

  // Repeat a string multiple times
  let repeated = "ha".repeat(3);
  println!("Repeated: {repeated}");
}
