// import io lib
use std::io;

fn main() {
  println!("Please choose the input type:");

  let mut input: char = char::new();
  
  if input == 'C' {
      println!("Converting C to F");
  } else if input == 'F' {
      println!("Converting F to C");
  } else {
      println!("Invalid input!");
  }
  
}