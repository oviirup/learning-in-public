// Functions are declared using the `fn` keyword, followed by a name, parentheses `()`, and a body enclosed in curly braces `{}`.
// Functions can take parameters and return values. Unlike TypeScript, Rust cannot infer the return type, so it must be explicitly specified in case the function returns a value

fn main() {
  basic();
  greeting("John Nolan");

  let numbers = [10, 20, 30, 40, 50];
  let avg = mean(&numbers);
  println!("The mean is: {}", avg);

  let tree = christmas_tree(5);
  println!("{}", tree);
}

// This is the simplest form of a function
fn basic() {
  println!("This is a basic function");
}

// Functions can also take parameters
// The arguments cannot have a default value, but they can be optional by using the `Option` type
fn greeting(name: &str) {
  println!("Hello, {}!", name);
}

// the "&" is use to make a reference to the array, not to copy it
fn mean(numbers: &[u16]) -> f32 {
  let sum: u16 = numbers.iter().sum();
  let count = numbers.len() as f32;
  // return sum as f32 / count; ... we can omit the return keyword and the semicolon to return the value of the last expression
  sum as f32 / count
}

// Function that returns a value must have a return type
fn christmas_tree(height: u16) -> String {
  let mut tree = String::new();
  for i in 0..height {
    let spaces = " ".repeat((height - i - 1) as usize);
    let stars = "*".repeat((2 * i + 1) as usize);
    tree.push_str(&format!("{}{}\n", spaces, stars));
  }
  return tree;
}
