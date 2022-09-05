use std::io;
use io::stdin;
use std::cmp::Ordering;

fn main() {
    // println!("input fahrenheit");
    // let mut fahrenheit = String::new();
    //
    // stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    // let celsius = fahrenheit_to_celsius(fahrenheit.trim().parse().expect("Floating Point number expected"));
    // println!("Celsius is {celsius}°C");

    println!("nth Fibonacci: ");
    let mut nth_fibonacci = String::new();
    stdin().read_line(&mut nth_fibonacci).expect("Failed to read line");
    let nth_fibonacci = fibonacci(nth_fibonacci.trim().parse().expect("u32 expected!"));
    println!("nth Fibonacci is {nth_fibonacci}")


}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn fibonacci(nth_fibonacci: u32) -> u32 {
    let mut fibonacci0 = 0;
    let mut fibonacci1  = 1;

    match nth_fibonacci.cmp(&fibonacci0) {
        Ordering::Equal => 0,
        Ordering::Greater => {
            for i in 1..nth_fibonacci {
                let next_fibonacci = fibonacci0 + fibonacci1;
                fibonacci0 = fibonacci1;
                fibonacci1 = next_fibonacci;
            }
            fibonacci1
        },
        Ordering::Less => {
            println!("Negative integer");
            0
        },
    }
}