// add io for input
use std::io;

fn main() {
    println!("Input the nth number:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_val: f64 = input.trim().parse().expect("Invalid input");

    println!("The number is {}", input_val);
}
