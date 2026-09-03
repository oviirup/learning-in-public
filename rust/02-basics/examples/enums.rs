// Enums in Rust let you define a type by enumerating its possible variants,
// and each variant can optionally carry associated data.

#[allow(unused)] // allow unused code since this is just a demo

fn main() {
  // 1. Basic enum with match
  enum Direction {
    North,
    South,
    East,
    West,
  }
  let dir = Direction::North;
  let label = match dir {
    Direction::North => "Heading north",
    Direction::South => "Heading south",
    Direction::East => "Heading east",
    Direction::West => "Heading west",
  };
  println!("Direction: {}", label);

  // 2. Enum with associated data — shape area using Heron's formula for triangles
  enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
  }
  impl Shape {
    fn area(&self) -> f64 {
      match self {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
      }
    }
    fn label(&self) -> &str {
      match self {
        Shape::Circle(_) => "Circle",
        Shape::Rectangle(_, _) => "Rectangle",
      }
    }
  }
  let shapes = [Shape::Circle(3.0), Shape::Rectangle(4.0, 5.0)];
  for shape in &shapes {
    println!("{} area = {:.2}", shape.label(), shape.area());
  }

  // 3. Option<char> — get a character at a specified index from a string
  // chars().nth() returns Some(char) if in bounds, None otherwise
  let word = "rustacean";
  for i in [0_usize, 4, 8, 20] {
    match word.chars().nth(i) {
      Some(c) => println!("char at index {i}: '{c}'"),
      None => println!("index {i} is out of bounds for \"{word}\""),
    }
  }

  // 4. Enum with struct-like variants — simple command parser
  enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
  }

  let commands = [
    Command::Move { x: 10, y: -3 },
    Command::Write(String::from("hello, enums")),
    Command::Quit,
  ];
  for cmd in commands {
    match cmd {
      Command::Quit => println!("Quitting..."),
      Command::Move { x, y } => println!("Moving to ({x}, {y})"),
      Command::Write(text) => println!("Writing: {text}"),
    }
  }
}
