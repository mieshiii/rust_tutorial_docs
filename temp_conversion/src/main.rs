// import io lib
use std::io;

fn main() {
    println!("Please choose the conversion: C or F.");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
  
    if input == "C\n" {
        println!("Converting C to F");
        println!("Input the value to convert:");
        // input type is string, parsed and conerted to float
        let mut temp: String = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        // convert String to float
        let input_val: f64 = temp.trim().parse().expect("Invalid input");
        let converted_val: f64 = (input_val * 9.0/5.0) + 32.0;

        println!("Converted value in Fahrenheit is: {}", converted_val);

    } else if input == "F\n" {
        println!("Converting F to C");
        println!("Input the value to convert:");
        // input type is string, parsed and conerted to float
        let mut temp: String = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        // convert String to float
        let input_val: f64 = temp.trim().parse().expect("Invalid input");
        let converted_val: f64 = (input_val - 32.0) * (5.0/9.0);

        println!("Converted value in Celsius is: {}", converted_val);

    } else {
        println!("Invalid input!");
    }
    
}