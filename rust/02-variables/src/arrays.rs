pub fn run() {
  // array of 5 i8 integers
  let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
  // get a value at index
  let first = arr[0];
  let last = arr[4];
  println!("First: {first}, Last: {last}");
  // change a value at index
  arr[2] = 10;
  println!("Updated array: {:?}", arr);

  // create a slice from the array
  let slice = &arr[1..3];
  println!("Slice: {:?}", slice);
  // notice the "&" symbol, it is use to create a reference to the array,
  // it is called BORROWING
}
