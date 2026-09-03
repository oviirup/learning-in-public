struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl Color {
  fn new(red: u8, green: u8, blue: u8) -> Color {
    Color { red, green, blue }
  }
  fn set_color(&mut self, red: u8, green: u8, blue: u8) {
    self.red = red;
    self.green = green;
    self.blue = blue;
  }
  // convert the color to a hexadecimal string
  fn to_hex(&self) -> String {
    format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
  }
  // convert the color to an RGB string
  fn to_rgb(&self) -> String {
    format!("rgb({},{},{})", self.red, self.green, self.blue)
  }
}

fn main() {
  let red = Color::new(255, 0, 0);
  println!("Red color in hex: {}", red.to_hex());
  println!("Red color in RGB: {}", red.to_rgb());

  let green = Color::new(0, 255, 0);
  println!("Green color in hex: {}", green.to_hex());
  println!("Green color in RGB: {}", green.to_rgb());
  let mut green = green;
  green.set_color(0, 145, 75);
  println!("Updated green color in hex: {}", green.to_hex());
  println!("Updated green color in RGB: {}", green.to_rgb());

  let blue = Color::new(0, 0, 255);
  println!("Blue color in hex: {}", blue.to_hex());
  println!("Blue color in RGB: {}", blue.to_rgb());
}
