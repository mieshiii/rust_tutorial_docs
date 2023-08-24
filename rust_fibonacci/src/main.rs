// add io for input
use std::io;

// fib seq logic
fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut term1 = 1;
    let mut term2 = 0;
    let mut fib_n = 0;

    for _ in 2..=n {
        fib_n = term1 + term2;
        term2 = term1;
        term1 = fib_n;
    }

    fib_n
}

fn main() {
    println!("Input the nth number:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Invalid input, input a valid number");
    
    println!("Fibonacci sequence up to {} terms:", n);

    for i in 0..n {
        println!("F({}) = {}", i, fib(i));
    };
}
