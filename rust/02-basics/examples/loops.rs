// Loops in Rust are very similar to loops in javascript or any other language.

fn main() {
  // let mut count: u8 = 0;

  /*
  // for loop with range
  for i in 0..3 as u8 {
    println!("i: {i}");
  }
  */

  /*
  // while loop
  while count <= 100 {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", count);
    }
    // break the loop when count is 50
    if count == 50 {
      println!("Done!");
      break;
    }
    count += 1;
  }
  */

  /*
  // infinite loop
  loop {
    if count % 15 == 0 {
      println!("fizzbuzz");
    } else if count % 3 == 0 {
      println!("fizz");
    } else if count % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", count);
    }
    // break the loop when count is 50
    if count == 50 {
      println!("Done!");
      break;
    }
    count += 1;
  }
  */

  // loops with labels
  // labels are used to break out of nested loops
  'outer: for x in 0..5 as u8 {
    println!("x: {x}");
    'inner: for y in 0..10 as u8 {
      println!("y: {y}");
      if y == 2 {
        println!("Breaking out of inner loop");
        break 'inner;
      }
      if x == 3 {
        println!("Breaking out of outer loop");
        break 'outer;
      }
    }
  }
}
