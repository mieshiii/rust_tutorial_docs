// import io lib
use std::io::{self, Read};

fn main() {
    println!("Please choose the conversion: C or F.");

    let mut input: [u8; 1] = [0u8; 1];
    io::stdin().read_exact(&mut input).expect("Failed to read line");
    let character: u8 = input[0];
  
    if character == b'C' {
        println!("Converting C to F");
        println!("Input the value to convert:");
        // input type is a String(?)
        let mut val: String = String::new();
        io::stdin().read_line(&mut val).expect("Failed to read line");

        // convert String to float
        let input_val: f64 = val.trim().parse().expect("Invalid input");
        let converted_val: f64 = (input_val * 9.0/5.0) + 32.0;

        println!("Converted value in F is:{}", converted_val);

    } else if character == b'F' {
        println!("Converting F to C");
    } else {
        println!("Invalid input!");
    }
    
}