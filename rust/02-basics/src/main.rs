mod arrays;
mod strings;
mod types;

fn main() {
  gap("TYPES");
  types::run();
  gap("STRINGS");
  strings::run();
  gap("ARRAYS");
  arrays::run();
}

fn gap(name: &str) {
  let length = 40;
  let line = "-".repeat(length - name.len() - 1);
  println!("\n{name} {line}\n");
}
